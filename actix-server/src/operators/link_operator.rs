use crate::{
    data::models::{Link, PgPool},
    errors::ServiceError,
    prefixes::{LinkPrefix, OrgPrefix, PrefixedUuid},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_link_query(
    org_id: PrefixedUuid<OrgPrefix>,
    link: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Link, ServiceError> {
    use crate::data::schema::links::dsl as links_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_link = Link::from_details(link, org_id);
    let link = diesel::insert_into(links_columns::links)
        .values(&new_link)
        .get_result::<Link>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating links".to_string()))?;
    Ok(link)
}

pub async fn delete_link_query(
    link_id: PrefixedUuid<LinkPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::links::dsl as links_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(links_columns::links)
        .filter(links_columns::id.eq(link_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting link".to_string()))?;

    Ok(())
}

pub async fn update_link_query(
    link_id: PrefixedUuid<LinkPrefix>,
    link: Option<String>,
    pg_pool: web::Data<PgPool>,
) -> Result<Link, ServiceError> {
    use crate::data::schema::links::dsl as links_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let target = links_columns::links.filter(links_columns::id.eq(link_id));
    let updated_link = diesel::update(target)
        .set((link.map(|link| links_columns::link.eq(link)),))
        .get_result::<Link>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating link".to_string()))?;
    Ok(updated_link)
}

pub async fn get_link_by_id(
    link_id: PrefixedUuid<LinkPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Link, ServiceError> {
    use crate::data::schema::links::dsl as links_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let link = links_columns::links
        .filter(links_columns::id.eq(link_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(link)
}
