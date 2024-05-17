use crate::{
    data::models::{Org, PgPool},
    operators::org_operator::create_org_query,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::auth_handler::AuthedUser;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrgReqPayload {
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateOrgResp {
    org: Org,
}

#[utoipa::path(
  post,
  path = "/orgs",
  context_path = "/api",
  tag = "orgs",
  request_body(content = CreateOrgReqPayload, description = "JSON request payload to create a new organization", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the organization that was created", body = CreateApiKeyRespPayload),
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

    Ok(HttpResponse::Created().json(CreateOrgResp { org }))
}
