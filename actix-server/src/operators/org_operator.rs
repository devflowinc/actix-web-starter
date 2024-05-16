use crate::{
    data::models::{Org, OrgUserLink, PgPool},
    errors::ServiceError,
};
use actix_web::web;
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_org_query(
    name: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Org, ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org = Org::from_details(name);
    let org = diesel::insert_into(orgs_columns::orgs)
        .values(&org)
        .get_result::<Org>(&mut conn)
        .await
        .map_err(|_| {
            ServiceError::InternalServerError(
                "Error creating user for create_user_query".to_string(),
            )
        })?;

    Ok(org)
}

#[tracing::instrument(skip(pg_pool))]
pub async fn link_org_with_user(
    org_id: uuid::Uuid,
    user_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<OrgUserLink, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org_user_link = OrgUserLink::from_details(user_id, org_id);

    let org_user_link = diesel::insert_into(orgs_users_columns::org_users)
        .values(&org_user_link)
        .get_result::<OrgUserLink>(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => ServiceError::BadRequest("User and organization already linked".to_string()),
            _ => ServiceError::InternalServerError(
                "Error connecting user with organization".to_string(),
            ),
        })?;

    Ok(org_user_link)
}
