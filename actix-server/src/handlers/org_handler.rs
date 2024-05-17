use crate::{
    data::models::{Org, PgPool},
    operators::org_operator::{
        create_org_query, delete_org_query, get_my_orgs_query, get_org_by_id_query,
        rename_org_query, user_in_org,
    },
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::auth_handler::AuthedUser;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrgNameReqPayload {
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SingleOrgResp {
    org: Org,
}

#[utoipa::path(
  post,
  path = "/orgs",
  context_path = "/api",
  tag = "orgs",
  request_body(content = OrgNameReqPayload, description = "JSON request payload to create a new organization", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the organization that was created", body = SingleOrgResp),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_org(
    req_payload: web::Json<OrgNameReqPayload>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = req_payload.name.clone();
    let org = create_org_query(name, authed_user, pg_pool).await?;

    Ok(HttpResponse::Created().json(SingleOrgResp { org }))
}

#[utoipa::path(
  delete,
  path = "/orgs/{org_id}",
  context_path = "/api",
  tag = "orgs",
  responses(
      (status = 200, description = "Blank body indicating that the organization was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_org(
    authed_user: AuthedUser,
    path: web::Path<uuid::Uuid>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let org_id = path.into_inner();

    match user_in_org(org_id, authed_user.id, &pg_pool).await {
        Err(e) => Err(e.into()),
        Ok(false) => Ok(HttpResponse::Unauthorized().finish()),
        Ok(true) => delete_org_query(org_id.into(), pg_pool)
            .await
            .map(|_| Ok(HttpResponse::Ok().finish()))?,
    }
}

#[utoipa::path(
  get,
  path = "/orgs/{org_id}",
  context_path = "/api",
  tag = "orgs",
  responses(
      (status = 200, description = "Blank body indicating that the organization was successfully deleted", body = SingleOrgResp),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_org_by_id(
    path: web::Path<uuid::Uuid>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let org_id = path.into_inner();
    match user_in_org(org_id, authed_user.id, &pg_pool).await {
        Err(e) => Err(e.into()),
        Ok(false) => Ok(HttpResponse::Unauthorized().finish()),
        Ok(true) => {
            return get_org_by_id_query(org_id.into(), &pg_pool)
                .await
                .map(|_| Ok(HttpResponse::Ok().finish()))?;
        }
    }
}

#[utoipa::path(
  put,
  path = "/orgs/{org_id}",
  context_path = "/api",
  tag = "orgs",
  request_body(content = OrgNameReqPayload, description = "JSON request payload to rename the organization", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed organization", body = SingleOrgResp),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_org_name(
    req_payload: web::Json<OrgNameReqPayload>,
    path: web::Path<uuid::Uuid>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let org_id = path.into_inner();
    // TODO: Should check if user has permissions to rename
    match user_in_org(org_id, authed_user.id, &pg_pool).await {
        Err(e) => Err(e.into()),
        Ok(false) => Ok(HttpResponse::Unauthorized().finish()),
        Ok(true) => rename_org_query(org_id, req_payload.name.clone(), &pg_pool)
            .await
            .map(|org| Ok(HttpResponse::Ok().json(SingleOrgResp { org })))?,
    }
}

#[utoipa::path(
  get,
  path = "/orgs",
  context_path = "/api",
  tag = "orgs",
  params(
      ("limit" = Option<i64>, Query, description = "Limit the number of results. Default is 30"),
      ("skip" = Option<i64>, Query, description = "Skip the number of results"),
  ),
  responses(
      (status = 200, description = "List of organizations the user belongs to", body = [Org]),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_my_orgs(
    query: web::Query<MyOrgsQuery>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let MyOrgsQuery { limit, skip } = query.into_inner();
    match get_my_orgs_query(authed_user.id, &pg_pool, limit, skip).await {
        Err(e) => Err(e.into()),
        Ok(orgs) => Ok(HttpResponse::Ok().json(orgs)),
    }
}

#[derive(Debug, Deserialize)]
pub struct MyOrgsQuery {
    limit: Option<i64>,
    skip: Option<i64>,
}
