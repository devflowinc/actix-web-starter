use super::auth_handler::AuthedMember;
use crate::{data::models::PgPool, operators::note_operator::create_note_query};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateNoteReqPayload {
    title: String,
}

#[utoipa::path(
  post,
  path = "/notes",
  context_path = "/api",
  tag = "notes",
  request_body(content = CreateNoteReqPayload, description = "JSON request payload to create a new note", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the note that was created", body = Note),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_note(
    req_payload: web::Json<CreateNoteReqPayload>,
    org_member: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let title = req_payload.title.clone();
    let note = create_note_query(title, org_member.org_id, pg_pool).await?;
    Ok(HttpResponse::Created().json(note))
}
