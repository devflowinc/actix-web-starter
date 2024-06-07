use crate::{
    data::models::{PgPool, Task, TaskDeal, TaskLink, TaskUser},
    errors::ServiceError,
    prefixes::{
        ContactPrefix, DealPrefix, LinkPrefix, OrgPrefix, PrefixedUuid, TaskPrefix, UserPrefix,
    },
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

pub async fn create_deal_for_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    deal_id: PrefixedUuid<DealPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<TaskDeal, ServiceError> {
    use crate::data::schema::task_deals::dsl as task_deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_task_deal = TaskDeal::from_details(task_id, deal_id);
    let task_deal = diesel::insert_into(task_deals_columns::task_deals)
        .values(&new_task_deal)
        .get_result::<TaskDeal>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating task deal".to_string()))?;
    Ok(task_deal)
}

pub async fn delete_deal_from_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    deal_id: PrefixedUuid<DealPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::task_deals::dsl as task_deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    diesel::delete(task_deals_columns::task_deals)
        .filter(task_deals_columns::task_id.eq(task_id))
        .filter(task_deals_columns::deal_id.eq(deal_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting task deal".to_string()))?;
    Ok(())
}

pub async fn create_link_for_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    link_id: PrefixedUuid<LinkPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<TaskLink, ServiceError> {
    use crate::data::schema::task_links::dsl as task_links_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_task_deal = TaskLink::from_details(task_id, link_id);
    let task_link = diesel::insert_into(task_links_columns::task_links)
        .values(&new_task_deal)
        .get_result::<TaskLink>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating task link".to_string()))?;
    Ok(task_link)
}

pub async fn delete_link_from_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    link_id: PrefixedUuid<LinkPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::task_links::dsl as task_links_columns;
    let mut conn = pg_pool.get().await.unwrap();
    diesel::delete(task_links_columns::task_links)
        .filter(task_links_columns::task_id.eq(task_id))
        .filter(task_links_columns::link_id.eq(link_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting task link".to_string()))?;
    Ok(())
}

pub async fn create_user_for_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    user_id: PrefixedUuid<UserPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<TaskUser, ServiceError> {
    use crate::data::schema::task_users::dsl as task_users_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_task_user = TaskUser::from_details(task_id, user_id);
    let task_user = diesel::insert_into(task_users_columns::task_users)
        .values(&new_task_user)
        .get_result::<TaskUser>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating task user".to_string()))?;
    Ok(task_user)
}

pub async fn delete_user_from_task_query(
    task_id: PrefixedUuid<TaskPrefix>,
    user_id: PrefixedUuid<UserPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::task_users::dsl as task_users_columns;
    let mut conn = pg_pool.get().await.unwrap();
    diesel::delete(task_users_columns::task_users)
        .filter(task_users_columns::task_id.eq(task_id))
        .filter(task_users_columns::user_id.eq(user_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting task user".to_string()))?;
    Ok(())
}
