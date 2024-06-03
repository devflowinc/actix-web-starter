use crate::{
    data::models::{PgPool, User},
    operators::{
        org_operator::get_org_user_link_query, user_operator::get_user_from_api_key_query,
    },
    prefixes::{OrgPrefix, PrefixedUuid},
};
use actix_identity::Identity;
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use sentry::Transaction;
use std::{
    future::{ready, Ready},
    rc::Rc,
    str::FromStr,
};

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Clone the Rc pointers so we can move them into the async block.
        let srv = self.service.clone();
        Box::pin(async move {
            let tx_ctx =
                sentry::TransactionContext::new("middleware", "get dataset, org, and/or user");
            let transaction = sentry::start_transaction(tx_ctx);
            sentry::configure_scope(|scope| scope.set_span(Some(transaction.clone().into())));

            let get_user_span = transaction.start_child("get_user", "Getting user");

            let (http_req, pl) = req.parts_mut();
            let user = get_user(http_req, pl, transaction.clone()).await;
            if let Some(ref user) = user {
                req.extensions_mut().insert(user.clone());

                // Try to grab the organization from the header and verify membership
                if let Some(org_header) = req.headers().get("Organization") {
                    if let Ok(org_header) = org_header.to_str() {
                        if let Ok(org_uuid) = PrefixedUuid::<OrgPrefix>::from_str(org_header) {
                            let org_user_link = get_org_user_link_query(
                                user.id,
                                org_uuid,
                                req.app_data::<web::Data<PgPool>>()
                                    .expect("PgPool will always be in server state")
                                    .to_owned(),
                            )
                            .await
                            .ok();

                            if let Some(org_user_link) = org_user_link {
                                req.extensions_mut().insert(org_user_link);
                            }
                        }
                    }
                }
            };

            get_user_span.finish();

            transaction.finish();

            let res = srv.call(req).await?;

            Ok(res)
        })
    }
}

async fn get_user(req: &HttpRequest, pl: &mut Payload, tx: Transaction) -> Option<User> {
    let get_user_from_identity_span =
        tx.start_child("get_user_from_identity", "Getting user from identity");

    if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
        if let Ok(user_json) = identity.id() {
            if let Ok(user) = serde_json::from_str::<User>(&user_json) {
                return Some(user);
            }
        }
    }

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_header) = auth_header.to_str() {
            let user = get_user_from_api_key_query(
                auth_header,
                req.app_data::<web::Data<PgPool>>()
                    .expect("PgPool will always be in server state")
                    .to_owned(),
            )
            .await
            .ok();
            get_user_from_identity_span.finish();
            return user;
        }
    }

    get_user_from_identity_span.finish();

    None
}

pub struct AuthMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}
