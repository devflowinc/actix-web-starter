use crate::{
    data::models::{PgPool, Plan},
    errors::ServiceError,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_plan_query(
    stripe_id: String,
    num_users: i32,
    num_deals: i32,
    price_per_month: i32,
    pg_pool: PgPool,
) -> Result<Plan, ServiceError> {
    use crate::data::schema::plans::dsl as plans_columns;

    let mut conn = pg_pool.get().await.unwrap();

    // TODO: Maybe want a db transaction for all 3 steps?
    let plan = Plan::from_details(stripe_id, num_users, num_deals, price_per_month);

    let plan = diesel::insert_into(plans_columns::plans)
        .values(&plan)
        .get_result::<Plan>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating plan".to_string()))?;

    Ok(plan)
}

pub async fn delete_plan_query(
    plan_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::plans::dsl as plans_columns;

    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(plans_columns::plans)
        .filter(plans_columns::id.eq(plan_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting plan".to_string()))?;

    Ok(())
}

pub async fn update_plan_query(
    plan_id: uuid::Uuid,
    stripe_id: Option<String>,
    num_users: Option<i32>,
    num_deals: Option<i32>,
    price_per_month: Option<i32>,
    pg_pool: web::Data<PgPool>,
) -> Result<Plan, ServiceError> {
    use crate::data::schema::plans::dsl as plans_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let target = plans_columns::plans.filter(plans_columns::id.eq(plan_id));

    let updated_plan = diesel::update(target)
        .set((
            stripe_id.map(|s| plans_columns::stripe_id.eq(s)),
            num_users.map(|u| plans_columns::num_users.eq(u)),
            num_deals.map(|d| plans_columns::num_deals.eq(d)),
            price_per_month.map(|p| plans_columns::price_per_month.eq(p)),
        ))
        .get_result::<Plan>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating plan".to_string()))?;

    Ok(updated_plan)
}

pub async fn get_plan_by_id(
    plan_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<Plan, ServiceError> {
    use crate::data::schema::plans::dsl as plans_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let plan = plans_columns::plans
        .filter(plans_columns::id.eq(plan_id))
        .first::<Plan>(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;

    Ok(plan)
}
