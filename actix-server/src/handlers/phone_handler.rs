use super::auth_handler::OwnerMember;
use crate::{
    data::models::PgPool,
    operators::phone_operator::{
        create_phone_query, delete_phone_query, get_phone_by_id, update_phone_query,
    },
    prefixes::{PhonePrefix, PrefixedUuid},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreatePhoneReqPayload {
    number: String,
}

#[utoipa::path(
  post,
  path = "/phones",
  context_path = "/api",
  tag = "phones",
  request_body(content = CreatePhoneReqPayload, description = "JSON request payload to create a new phone", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the phone that was created", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_phone(
    req_payload: web::Json<CreatePhoneReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let phone = create_phone_query(org_user.org_id, req_payload.number.clone(), pg_pool).await?;
    Ok(HttpResponse::Created().json(phone))
}

#[utoipa::path(
  delete,
  path = "/phones/{phone_id}",
  context_path = "/api",
  tag = "phones",
  responses(
      (status = 204, description = "No content response indicating that the phone was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Phone" = String, Header, description = "The phone id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_phone(
    org_user: OwnerMember,
    path: web::Path<PrefixedUuid<PhonePrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let phone_id = path.into_inner();
    delete_phone_query(phone_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/phones/{phone_id}",
  context_path = "/api",
  tag = "phones",
  responses(
      (status = 200, description = "JSON object representing the requested phone", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_phone(
    path: web::Path<PrefixedUuid<PhonePrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let phone_id = path.into_inner();
    match get_phone_by_id(phone_id, pg_pool).await {
        Ok(phone) => Ok(HttpResponse::Ok().json(phone)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePhoneReqPayload {
    number: Option<String>,
}

#[utoipa::path(
  put,
  path = "/phones/{phone_id}",
  context_path = "/api",
  tag = "phones",
  request_body(content = UpdatePhoneReqPayload, description = "JSON request payload to update the phone", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the altered phone", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_phone(
    req_payload: web::Json<UpdatePhoneReqPayload>,
    path: web::Path<PrefixedUuid<PhonePrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let phone_id = path.into_inner();
    let phone = update_phone_query(phone_id, req_payload.number.clone(), pg_pool).await?;
    Ok(HttpResponse::Ok().json(phone))
}
