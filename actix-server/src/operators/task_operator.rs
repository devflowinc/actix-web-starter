use crate::{
    data::models::{Contact, PgPool, Task},
    errors::ServiceError,
    prefixes::{ContactPrefix, OrgPrefix, PrefixedUuid, TaskPrefix},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_task_query(
    org_id: PrefixedUuid<OrgPrefix>,
    contact_id: Option<PrefixedUuid<ContactPrefix>>,
    description: Option<String>,
    deadline: Option<chrono::NaiveDateTime>,
    pg_pool: web::Data<PgPool>,
) -> Result<Task, ServiceError> {
    use crate::data::schema::tasks::dsl as tasks_columns;
    let mut conn = pg_pool.get().await.unwrap();
    if let Some(contact_id) = contact_id {
        contact_exists(contact_id, pg_pool.clone()).await?;
    }
    let new_task = Task::from_details(org_id, deadline, description, contact_id);
    let task = diesel::insert_into(tasks_columns::tasks)
        .values(&new_task)
        .get_result::<Task>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating task".to_string()))?;
    Ok(task)
}

pub async fn delete_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::tasks::dsl as tasks_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(tasks_columns::tasks)
        .filter(tasks_columns::id.eq(task_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting task".to_string()))?;

    Ok(())
}

pub async fn update_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    description: Option<String>,
    deadline: Option<chrono::NaiveDateTime>,
    contact_id: Option<PrefixedUuid<ContactPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<Task, ServiceError> {
    use crate::data::schema::tasks::dsl as tasks_columns;
    let mut conn = pg_pool.get().await.unwrap();
    if let Some(contact_id) = contact_id {
        contact_exists(contact_id, pg_pool.clone()).await?;
    }
    let target = tasks_columns::tasks.filter(tasks_columns::id.eq(task_id));
    let updated_task = diesel::update(target)
        .set((
            description.map(|description| tasks_columns::description.eq(description)),
            deadline.map(|deadline| tasks_columns::deadline.eq(deadline)),
            contact_id.map(|contact_id| tasks_columns::contact_id.eq(contact_id)),
        ))
        .get_result::<Task>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating task".to_string()))?;
    Ok(updated_task)
}

pub async fn get_task_by_id(
    task_id: PrefixedUuid<TaskPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Task, ServiceError> {
    use crate::data::schema::tasks::dsl as tasks_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let task = tasks_columns::tasks
        .filter(tasks_columns::id.eq(task_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(task)
}

async fn contact_exists(
    contact_id: PrefixedUuid<ContactPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    contacts_columns::contacts
        .filter(contacts_columns::id.eq(contact_id))
        .first::<Contact>(&mut conn)
        .await
        .map_err(|_| ServiceError::BadRequest("Contact not found".to_string()))?;
    Ok(())
}
