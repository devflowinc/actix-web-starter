use crate::{
    data::models::{Deal, PgPool},
    errors::ServiceError,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_deal_query(
    org_id: uuid::Uuid,
    name: Option<String>,
    size: Option<f32>,
    active: bool,
    pg_pool: web::Data<PgPool>,
) -> Result<Deal, ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_deal = Deal::from_details(org_id, name, size, active);
    let deal = diesel::insert_into(deals_columns::deals)
        .values(&new_deal)
        .get_result::<Deal>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating deal".to_string()))?;
    Ok(deal)
}

pub async fn delete_deal_query(
    deal_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(deals_columns::deals)
        .filter(deals_columns::id.eq(deal_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting deal".to_string()))?;

    Ok(())
}

pub async fn update_deal_query(
    deal_id: uuid::Uuid,
    name: Option<String>,
    size: Option<f32>,
    active: Option<bool>,
    pg_pool: web::Data<PgPool>,
) -> Result<Deal, ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let target = deals_columns::deals.filter(deals_columns::id.eq(deal_id));
    let updated_deal = diesel::update(target)
        .set((
            name.map(|name| deals_columns::name.eq(name)),
            size.map(|size| deals_columns::size.eq(size)),
            active.map(|active| deals_columns::active.eq(active)),
        ))
        .get_result::<Deal>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating deal".to_string()))?;
    Ok(updated_deal)
}

pub async fn get_deal_by_id(
    deal_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<Deal, ServiceError> {
    use crate::data::schema::deals::dsl as deals_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let deal = deals_columns::deals
        .filter(deals_columns::id.eq(deal_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(deal)
}
