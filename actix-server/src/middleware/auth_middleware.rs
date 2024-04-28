use crate::{
    data::models::{Pool, User},
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

            let pool = req.app_data::<web::Data<Pool>>().unwrap().to_owned();

            let get_user_span = transaction.start_child("get_user", "Getting user");

            let (http_req, pl) = req.parts_mut();
            let user = get_user(http_req, pl, transaction.clone()).await;
            if let Some(ref user) = user {
                req.extensions_mut().insert(user.clone());
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
