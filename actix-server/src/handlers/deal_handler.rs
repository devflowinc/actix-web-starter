use super::auth_handler::OwnerMember;
use crate::{
    data::models::PgPool,
    operators::deal_operator::{
        create_deal_query, delete_deal_query, get_deal_by_id, update_deal_query,
    },
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateDealReqPayload {
    size: Option<f32>,
    name: Option<String>,
    active: Option<bool>,
}

#[utoipa::path(
  post,
  path = "/deals",
  context_path = "/api",
  tag = "deals",
  request_body(content = CreateDealReqPayload, description = "JSON request payload to create a new deal", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the deal that was created", body = Deal),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("Organization" = String, Header, description = "The org id to use for the request"),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_deal(
    req_payload: web::Json<CreateDealReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal = create_deal_query(
        org_user.org_id,
        req_payload.name.clone(),
        req_payload.size,
        req_payload.active.unwrap_or_default(),
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Created().json(deal))
}

#[utoipa::path(
  delete,
  path = "/deals/{deal_id}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 204, description = "No content response indicating that the deal was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request")
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_deal(
    org_user: OwnerMember,
    path: web::Path<uuid::Uuid>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal_id = path.into_inner();
    delete_deal_query(deal_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/deals/{deal_id}",
  context_path = "/api",
  tag = "deals",
  responses(
      (status = 200, description = "JSON object representing the requested deal", body = Deal),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request")
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_deal(
    path: web::Path<uuid::Uuid>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal_id = path.into_inner();
    match get_deal_by_id(deal_id, pg_pool).await {
        Ok(deal) => Ok(HttpResponse::Ok().json(deal)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateDealReqPayload {
    name: Option<String>,
    size: Option<f32>,
    active: Option<bool>,
}

#[utoipa::path(
  put,
  path = "/deals/{deal_id}",
  context_path = "/api",
  tag = "deals",
  request_body(content = UpdateDealReqPayload, description = "JSON request payload to update the deal", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed deal", body = Deal),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("deal_id" = String, description = "The deal id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request")
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_deal(
    req_payload: web::Json<UpdateDealReqPayload>,
    path: web::Path<uuid::Uuid>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let deal_id = path.into_inner();
    let deal = update_deal_query(
        deal_id,
        req_payload.name.clone(),
        req_payload.size,
        req_payload.active,
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Ok().json(deal))
}
