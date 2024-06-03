use super::auth_handler::AuthedMember;
use crate::{
    data::models::PgPool,
    operators::note_operator::{
        create_note_query, delete_note_query, get_note_by_id_query, get_notes_for_org_query,
        update_note_body_query,
    },
    prefixes::{NotePrefix, PrefixedUuid},
};
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

#[utoipa::path(
  delete,
  path = "/notes/{note_id}",
  context_path = "/api",
  tag = "notes",
  responses(
      (status = 204, description = "No content response indicating that the note was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("note_id" = String, Path, description = "The id of the note you want to delete."),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_note(
    org_user: AuthedMember,
    note_id: web::Path<PrefixedUuid<NotePrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let note_id = note_id.into_inner();

    delete_note_query(note_id, pg_pool).await?;

    return Ok(HttpResponse::NoContent().finish());
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateNoteReqPayload {
    body: String,
}

#[utoipa::path(
  put,
  path = "/notes/{note_id}",
  context_path = "/api",
  tag = "notes",
  request_body(content = UpdateNoteReqPayload, description = "JSON request payload to rename the organization", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed note", body = Note),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("note_id" = String, Path, description = "The id of the note you want to update."),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_note(
    req_payload: web::Json<UpdateNoteReqPayload>,
    note_id: web::Path<PrefixedUuid<NotePrefix>>,
    org_member: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let note_id = note_id.into_inner();

    let new_body = req_payload.body.clone();

    let new_note = update_note_body_query(note_id, new_body, pg_pool).await?;

    Ok(HttpResponse::Ok().json(new_note))
}

#[derive(Debug, Deserialize)]
pub struct GetNotesForOrgReqQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[utoipa::path(
  get,
  path = "/notes",
  context_path = "/api",
  tag = "notes",
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
pub async fn get_notes_for_org(
    query: web::Query<GetNotesForOrgReqQuery>,
    authed_user: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let org_notes =
        get_notes_for_org_query(authed_user.org_id, pg_pool, query.limit, query.offset).await?;

    Ok(HttpResponse::Ok().json(org_notes))
}

#[utoipa::path(
  get,
  path = "/notes/{note_id}",
  context_path = "/api",
  tag = "notes",
  responses(
      (status = 200, description = "JSON object representing the requested note", body = Note),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("note_id" = String, Path, description = "The id of the organization you want to fetch."),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_note_by_id(
    note_id: web::Path<PrefixedUuid<NotePrefix>>,
    org_member: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let note_id = note_id.into_inner();

    let note = get_note_by_id_query(note_id, pg_pool).await?;

    if note.org_id != org_member.org_id {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    Ok(HttpResponse::Ok().json(note))
}
