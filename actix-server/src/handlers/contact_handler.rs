use super::auth_handler::OwnerMember;
use crate::{
    data::models::PgPool,
    operators::contact_operator::{
        create_contact_query, delete_contact_query, get_contact_by_id, update_contact_query,
    },
    prefixes::{ContactPrefix, PrefixedUuid},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateContactReqPayload {
    first_name: String,
    last_name: String,
}

#[utoipa::path(
  post,
  path = "/contacts",
  context_path = "/api",
  tag = "contacts",
  request_body(content = CreateContactReqPayload, description = "JSON request payload to create a new contact", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the contact that was created", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_contact(
    req_payload: web::Json<CreateContactReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let contact = create_contact_query(
        org_user.org_id,
        req_payload.first_name.clone(),
        req_payload.last_name.clone(),
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Created().json(contact))
}

#[utoipa::path(
  delete,
  path = "/contacts/{contact_id}",
  context_path = "/api",
  tag = "contacts",
  responses(
      (status = 204, description = "No content response indicating that the contacts was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("contact_id" = String, Path, description = "The contacts id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_contact(
    org_user: OwnerMember,
    path: web::Path<PrefixedUuid<ContactPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let contact_id = path.into_inner();
    delete_contact_query(contact_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/contacts/{contact_id}",
  context_path = "/api",
  tag = "contacts",
  responses(
      (status = 200, description = "JSON object representing the requested contact", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("contact_id" = String, Path, description = "The contacts id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_contact(
    path: web::Path<PrefixedUuid<ContactPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let contact_id = path.into_inner();
    match get_contact_by_id(contact_id, pg_pool).await {
        Ok(contact) => Ok(HttpResponse::Ok().json(contact)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateContactReqPayload {
    first_name: Option<String>,
    last_name: Option<String>,
}

#[utoipa::path(
  put,
  path = "/contacts/{contact_id}",
  context_path = "/api",
  tag = "contacts",
  request_body(content = UpdateContactReqPayload, description = "JSON request payload to update the contact", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed contact", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("contact_id" = String, Path, description = "The contacts id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_contact(
    req_payload: web::Json<UpdateContactReqPayload>,
    path: web::Path<PrefixedUuid<ContactPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let contact_id = path.into_inner();
    let contact = update_contact_query(
        contact_id,
        req_payload.first_name.clone(),
        req_payload.last_name.clone(),
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Ok().json(contact))
}
