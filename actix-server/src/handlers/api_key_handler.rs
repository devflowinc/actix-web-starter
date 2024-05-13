use super::auth_handler::AuthedUser;
use crate::{data::models::PgPool, operators::api_key_operator::create_api_key_query};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateApiKeyReqPayload {
    /// The name which will be assigned to the new api key.
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateApiKeyRespPayload {
    /// The api key which was created. This is the value which should be used in the Authorization header.
    api_key: String,
}

/// Set User Api Key
///
/// Create a new api key for the auth'ed user. Successful response will contain the newly created api key. The api key will have permission level of the auth'ed user who calls this endpoint.
#[utoipa::path(
  post,
  path = "/api_key",
  context_path = "/api",
  tag = "api_key",
  request_body(content = CreateApiKeyReqPayload, description = "JSON request payload to create a new user api key", content_type = "application/json"),
  responses(
      (status = 200, description = "JSON body representing the api_key for the user", body = CreateApiKeyRespPayload),
      (status = 400, description = "Service error relating to creating api_key for the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_api_key(
    authed_user: AuthedUser,
    req_payload: web::Json<CreateApiKeyReqPayload>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = authed_user.id;
    let name = req_payload.name.clone();

    let api_key = create_api_key_query(user_id, name, pg_pool).await?;

    Ok(HttpResponse::Created().json(CreateApiKeyRespPayload { api_key }))
}
