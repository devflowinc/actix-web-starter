use super::auth_handler::{AuthedOrgMembership, AuthedUser};
use crate::{
    data::models::{Org, PgPool},
    operators::org_operator::{
        create_org_query, delete_org_query, get_orgs_for_user_query, update_org_query,
        user_in_org_query,
    },
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrgReqPayload {
    name: String,
}

#[utoipa::path(
  post,
  path = "/orgs",
  context_path = "/api",
  tag = "orgs",
  request_body(content = CreateOrgReqPayload, description = "JSON request payload to create a new organization", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the organization that was created", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_org(
    req_payload: web::Json<CreateOrgReqPayload>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = req_payload.name.clone();
    let org = create_org_query(name, authed_user, pg_pool).await?;

    Ok(HttpResponse::Created().json(org))
}

#[utoipa::path(
  delete,
  path = "/orgs/{org_id}",
  context_path = "/api",
  tag = "orgs",
  responses(
      (status = 204, description = "No content response indicating that the organization was successfully deleted"),
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

    match user_in_org_query(org_id, authed_user.id, &pg_pool).await? {
        Some(org) => delete_org_query(org.id, pg_pool)
            .await
            .map(|_| Ok(HttpResponse::NoContent().finish()))?,
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[utoipa::path(
  get,
  path = "/orgs/{org_id}",
  context_path = "/api",
  tag = "orgs",
  responses(
      (status = 200, description = "JSON object representing the requested organization", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_org(
    path: web::Path<uuid::Uuid>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let org_id = path.into_inner();

    match user_in_org_query(org_id, authed_user.id, &pg_pool).await? {
        Some(org) => Ok(HttpResponse::Ok().json(org)),
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateOrgReqPayload {
    name: String,
}

#[utoipa::path(
  put,
  path = "/orgs/{org_id}",
  context_path = "/api",
  tag = "orgs",
  request_body(content = UpdateOrgReqPayload, description = "JSON request payload to rename the organization", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed organization", body = Org),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_org(
    req_payload: web::Json<UpdateOrgReqPayload>,
    path: web::Path<uuid::Uuid>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let org_id = path.into_inner();
    let mut org = Org::from_details_with_id(org_id, req_payload.name.clone());

    match user_in_org_query(org_id, authed_user.id, &pg_pool).await {
        Ok(opt_org) => match opt_org {
            Some(prev_org) => {
                org.created_at = prev_org.created_at;
                update_org_query(org, &pg_pool)
                    .await
                    .map(|org| Ok(HttpResponse::Ok().json(org)))?
            }
            None => Ok(HttpResponse::Unauthorized().finish()),
        },
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug, Deserialize)]
pub struct GetMyOrgsReqQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[utoipa::path(
  get,
  path = "/orgs",
  context_path = "/api",
  tag = "orgs",
  params(
      ("limit" = Option<i64>, Query, description = "Limit the number of results. Default is 10"),
      ("offset" = Option<i64>, Query, description = "Offset the results. Default is 0"),
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
pub async fn get_orgs_for_authed_user(
    query: web::Query<GetMyOrgsReqQuery>,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_orgs =
        get_orgs_for_user_query(authed_user.id, &pg_pool, query.limit, query.offset).await?;

    Ok(HttpResponse::Ok().json(user_orgs))
}
