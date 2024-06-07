use std::str::FromStr;

use super::auth_handler::OwnerMember;
use crate::{
    data::models::{Deal, Link, PgPool, TaskDeal, TaskLink, TaskUser, User},
    errors::ServiceError,
    operators::{
        deal_operator::list_deals_by_task_id,
        link_operator::list_links_by_task_id,
        task_operator::{
            create_deal_for_task_query, create_link_for_task_query, create_task_query,
            create_user_for_task_query, delete_deal_from_task_query, delete_link_from_task_query,
            delete_task_query, delete_user_from_task_query, get_task_by_id, update_task_query,
        },
        user_operator::list_users_by_task_id,
    },
    prefixes::{ContactPrefix, DealPrefix, LinkPrefix, PrefixedUuid, TaskPrefix, UserPrefix},
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

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskResource {
    Link(TaskLink),
    Deal(TaskDeal),
    User(TaskUser),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskResType {
    Link,
    Deal,
    User,
}

#[utoipa::path(
  post,
  path = "/tasks/{task_id}/{resource_type}/{resource_id}",
  context_path = "/api",
  tag = "tasks",
  responses(
      (status = 200, description = "Object representing the created relationship", body = TaskResource),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  params(
    ("task_id" = String, description = "The task id to use for the request"),
    ("resource_type" = TaskResType, description = "The resource type to use for the request"),
    ("resource_id" = String, description = "The resource id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
pub async fn create_task_resource(
    path: web::Path<(PrefixedUuid<TaskPrefix>, TaskResType, String)>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let (task_id, resource, resource_id) = path.into_inner();
    match resource {
        TaskResType::Link => {
            let link_id = PrefixedUuid::<LinkPrefix>::from_str(&resource_id)?;
            let task_link = create_link_for_task_query(task_id, link_id, pg_pool).await?;
            Ok(HttpResponse::Created().json(TaskResource::Link(task_link)))
        }
        TaskResType::Deal => {
            let deal_id = PrefixedUuid::<DealPrefix>::from_str(&resource_id)?;
            let task_deal = create_deal_for_task_query(task_id, deal_id, pg_pool).await?;
            Ok(HttpResponse::Created().json(TaskResource::Deal(task_deal)))
        }
        TaskResType::User => {
            let user_id = PrefixedUuid::<UserPrefix>::from_str(&resource_id)?;
            let task_user = create_user_for_task_query(task_id, user_id, pg_pool).await?;
            Ok(HttpResponse::Created().json(TaskResource::User(task_user)))
        }
    }
}

#[utoipa::path(
  delete,
  path = "/tasks/{task_id}/{resource_type}/{resource_id}",
  context_path = "/api",
  tag = "tasks",
  responses(
      (status = 204, description = "No content response indicating that the task resource was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  params(
    ("task_id" = String, description = "The task id to use for the request"),
    ("resource_type" = TaskResType, description = "The resource type to use for the request"),
    ("resource_id" = String, description = "The resource id to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
pub async fn delete_task_resource(
    path: web::Path<(PrefixedUuid<TaskPrefix>, TaskResType, String)>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let (task_id, resource, resource_id) = path.into_inner();
    match resource {
        TaskResType::Link => {
            let link_id = PrefixedUuid::<LinkPrefix>::from_str(&resource_id)?;
            delete_link_from_task_query(task_id, link_id, pg_pool).await?;
        }
        TaskResType::Deal => {
            let deal_id = PrefixedUuid::<DealPrefix>::from_str(&resource_id)?;
            delete_deal_from_task_query(task_id, deal_id, pg_pool).await?;
        }
        TaskResType::User => {
            let user_id = PrefixedUuid::<UserPrefix>::from_str(&resource_id)?;
            delete_user_from_task_query(task_id, user_id, pg_pool).await?;
        }
    }
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetTaskResourceQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TaskResourceListWithPagination {
    pub data: TaskResourceList,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskResourceList {
    Link(Vec<Link>),
    Deal(Vec<Deal>),
    User(Vec<User>),
}

#[utoipa::path(
  get,
  path = "/tasks/{task_id}/{resource_type}",
  context_path = "/api",
  tag = "tasks",
  responses(
      (status = 200, description = "List of objects of resource_type", body = TaskResourceListWithPagination),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
      (status = 400, description = "Service error relating to the request payload", body = ErrorRespPayload),
  ),
  params(
    ("task_id" = String, description = "The task id to use for the request"),
    ("resource_type" = TaskResType, description = "The resource type to use for the request"),
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("limit" = i64, Query, description = "The number of records to return"),
    ("offset" = i64, Query, description = "The number of records to skip"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  ),
)]
pub async fn list_task_resource(
    path: web::Path<(PrefixedUuid<TaskPrefix>, TaskResType)>,
    query: web::Query<GetTaskResourceQuery>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let (task_id, resource) = path.into_inner();
    let GetTaskResourceQuery { limit, offset } = query.into_inner();
    match resource {
        TaskResType::Link => {
            let (links, count) = list_links_by_task_id(task_id, pg_pool, offset, limit).await?;
            Ok(HttpResponse::Ok().json(TaskResourceListWithPagination {
                data: TaskResourceList::Link(links),
                total: count,
            }))
        }
        TaskResType::Deal => {
            let (deals, count) = list_deals_by_task_id(task_id, pg_pool, offset, limit).await?;
            Ok(HttpResponse::Ok().json(TaskResourceListWithPagination {
                data: TaskResourceList::Deal(deals),
                total: count,
            }))
        }
        TaskResType::User => {
            let (users, count) = list_users_by_task_id(task_id, pg_pool, offset, limit).await?;
            Ok(HttpResponse::Ok().json(TaskResourceListWithPagination {
                data: TaskResourceList::User(users),
                total: count,
            }))
        }
    }
}
