use super::auth_handler::OwnerMember;
use crate::{
    data::models::PgPool,
    operators::email_db_operator::{
        create_email_query, delete_email_query, get_email_by_id, update_email_query,
    },
    prefixes::{EmailPrefix, PrefixedUuid},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateEmailReqPayload {
    email: String,
}

#[utoipa::path(
  post,
  path = "/emails",
  context_path = "/api",
  tag = "emails",
  request_body(content = CreateEmailReqPayload, description = "JSON request payload to create a new email", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the email that was created", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_email(
    req_payload: web::Json<CreateEmailReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = create_email_query(org_user.org_id, req_payload.email.clone(), pg_pool).await?;
    Ok(HttpResponse::Created().json(email))
}

#[utoipa::path(
  delete,
  path = "/emails/{email_id}",
  context_path = "/api",
  tag = "emails",
  responses(
      (status = 204, description = "No content response indicating that the email was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("email_id" = String, Path, description = "The email id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_email(
    org_user: OwnerMember,
    path: web::Path<PrefixedUuid<EmailPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let email_id = path.into_inner();
    delete_email_query(email_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/emails/{email_id}",
  context_path = "/api",
  tag = "emails",
  responses(
      (status = 200, description = "JSON object representing the requested email", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("email_id" = String, Path, description = "The email id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_email(
    path: web::Path<PrefixedUuid<EmailPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let email_id = path.into_inner();
    match get_email_by_id(email_id, pg_pool).await {
        Ok(email) => Ok(HttpResponse::Ok().json(email)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateEmailReqPayload {
    email: Option<String>,
}

#[utoipa::path(
  put,
  path = "/emails/{email_id}",
  context_path = "/api",
  tag = "emails",
  request_body(content = UpdateEmailReqPayload, description = "JSON request payload to update the email", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed email", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("email_id" = String, Path, description = "The email id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_email(
    req_payload: web::Json<UpdateEmailReqPayload>,
    path: web::Path<PrefixedUuid<EmailPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let email_id = path.into_inner();
    let email = update_email_query(email_id, req_payload.email.clone(), pg_pool).await?;
    Ok(HttpResponse::Ok().json(email))
}
