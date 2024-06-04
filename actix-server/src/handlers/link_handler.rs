use super::auth_handler::OwnerMember;
use crate::{
    data::models::PgPool,
    operators::link_operator::{
        create_link_query, delete_link_query, get_link_by_id, update_link_query,
    },
    prefixes::{LinkPrefix, PrefixedUuid},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateLinkReqPayload {
    link: String,
}

#[utoipa::path(
  post,
  path = "/links",
  context_path = "/api",
  tag = "links",
  request_body(content = CreateLinkReqPayload, description = "JSON request payload to create a new link", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the link that was created", body = Link),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
      ("Organization" = String, Header, description = "The org id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_link(
    req_payload: web::Json<CreateLinkReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let link = create_link_query(org_user.org_id, req_payload.link.clone(), pg_pool).await?;
    Ok(HttpResponse::Created().json(link))
}

#[utoipa::path(
  delete,
  path = "/links/{link_id}",
  context_path = "/api",
  tag = "links",
  responses(
      (status = 204, description = "No content response indicating that the link was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("link_id" = String, description = "The link id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_link(
    org_user: OwnerMember,
    path: web::Path<PrefixedUuid<LinkPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let link_id = path.into_inner();
    delete_link_query(link_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/links/{link_id}",
  context_path = "/api",
  tag = "links",
  responses(
      (status = 200, description = "JSON object representing the requested link", body = Link),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("link_id" = String, description = "The link id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request"),
  ),
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_link(
    path: web::Path<PrefixedUuid<LinkPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let link_id = path.into_inner();
    match get_link_by_id(link_id, pg_pool).await {
        Ok(link) => Ok(HttpResponse::Ok().json(link)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateLinkReqPayload {
    link: Option<String>,
}

#[utoipa::path(
  put,
  path = "/links/{link_id}",
  context_path = "/api",
  tag = "links",
  request_body(content = UpdateLinkReqPayload, description = "JSON request payload to update the link", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed link", body = Link),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("link_id" = String, description = "The link id to use for the request"),
    ("Organization" = String, Header, description = "The org id to use for the request"),
  ),
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_link(
    req_payload: web::Json<UpdateLinkReqPayload>,
    path: web::Path<PrefixedUuid<LinkPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let link_id = path.into_inner();
    let link = update_link_query(link_id, req_payload.link.clone(), pg_pool).await?;
    Ok(HttpResponse::Ok().json(link))
}
