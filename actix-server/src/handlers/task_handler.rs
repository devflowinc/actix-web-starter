use super::auth_handler::OwnerMember;
use crate::{
    data::models::PgPool,
    operators::task_operator::{
        create_task_query, delete_task_query, get_task_by_id, update_task_query,
    },
    prefixes::{ContactPrefix, PrefixedUuid, TaskPrefix},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTaskReqPayload {
    description: Option<String>,
    #[schema(example = "2021-01-01T00:00:00")]
    deadline: Option<chrono::NaiveDateTime>,
    contact_id: Option<PrefixedUuid<ContactPrefix>>,
}

#[utoipa::path(
  post,
  path = "/tasks",
  context_path = "/api",
  tag = "tasks",
  request_body(content = CreateTaskReqPayload, description = "JSON request payload to create a new task", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the task that was created", body = Task),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request")
  ),
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_task(
    req_payload: web::Json<CreateTaskReqPayload>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let task = create_task_query(
        org_user.org_id,
        req_payload.contact_id,
        req_payload.description.clone(),
        req_payload.deadline,
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Created().json(task))
}

#[utoipa::path(
  delete,
  path = "/tasks/{task_id}",
  context_path = "/api",
  tag = "tasks",
  responses(
      (status = 204, description = "No content response indicating that the task was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("task_id" = String, description = "The task id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_task(
    org_user: OwnerMember,
    path: web::Path<PrefixedUuid<TaskPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let task_id = path.into_inner();
    delete_task_query(task_id, pg_pool)
        .await
        .map(|_| Ok(HttpResponse::NoContent().finish()))?
}

#[utoipa::path(
  get,
  path = "/tasks/{task_id}",
  context_path = "/api",
  tag = "tasks",
  responses(
      (status = 200, description = "JSON object representing the requested task", body = Task),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("task_id" = String, description = "The task id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request")
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_task(
    path: web::Path<PrefixedUuid<TaskPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let task_id = path.into_inner();
    match get_task_by_id(task_id, pg_pool).await {
        Ok(task) => Ok(HttpResponse::Ok().json(task)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTaskReqPayload {
    description: Option<String>,
    #[schema(example = "2021-01-01T00:00:00")]
    deadline: Option<chrono::NaiveDateTime>,
    contact_id: Option<PrefixedUuid<ContactPrefix>>,
}

#[utoipa::path(
  put,
  path = "/tasks/{task_id}",
  context_path = "/api",
  tag = "tasks",
  request_body(content = UpdateTaskReqPayload, description = "JSON request payload to update the task", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the updated task", body = Task),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  params(
    ("task_id" = String, description = "The task id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_task(
    req_payload: web::Json<UpdateTaskReqPayload>,
    path: web::Path<PrefixedUuid<TaskPrefix>>,
    org_user: OwnerMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let task_id = path.into_inner();
    let task = update_task_query(
        task_id,
        req_payload.description.clone(),
        req_payload.deadline,
        req_payload.contact_id,
        pg_pool,
    )
    .await?;
    Ok(HttpResponse::Ok().json(task))
}
