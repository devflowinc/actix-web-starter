use crate::{
    data::models::{Org, OrgUserLink, PgPool, UserRole},
    errors::ServiceError,
    handlers::auth_handler::AuthedUser,
    prefixes::{OrgPrefix, OrgUserPrefix, PrefixedUuid, UserPrefix},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_org_query(
    name: String,
    authed_user: AuthedUser,
    pg_pool: web::Data<PgPool>,
) -> Result<Org, ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org = Org::from_details(name);
    let org = diesel::insert_into(orgs_columns::orgs)
        .values(&org)
        .get_result::<Org>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::InternalServerError(
                format!("Error creating org for create_org_query: {}", e).to_string(),
            )
        })?;

    // Make the user an owner
    add_user_to_org_query(authed_user.id, org.id, UserRole::Owner, pg_pool).await?;

    Ok(org)
}

pub async fn remove_user_from_org_query(
    org_id: PrefixedUuid<OrgPrefix>,
    user_id: PrefixedUuid<UserPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(
        orgs_users_columns::org_users
            .filter(orgs_users_columns::user_id.eq(user_id))
            .filter(orgs_users_columns::org_id.eq(org_id)),
    )
    .execute(&mut conn)
    .await
    .map_err(|e| {
        ServiceError::InternalServerError(format!("Error removing user from org: {}", e))
    })?;

    Ok(())
}

pub async fn add_user_to_org_query(
    user_id: PrefixedUuid<UserPrefix>,
    org_id: PrefixedUuid<OrgPrefix>,
    role: UserRole,
    pg_pool: web::Data<PgPool>,
) -> Result<OrgUserLink, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org_user_link = OrgUserLink {
        id: PrefixedUuid::create(OrgUserPrefix),
        user_id,
        org_id,
        role: role.into(),
    };

    let org_user_link = diesel::insert_into(orgs_users_columns::org_users)
        .values(&org_user_link)
        .get_result::<OrgUserLink>(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => ServiceError::BadRequest("User and organization already linked".to_string()),
            e => ServiceError::InternalServerError(format!(
                "Error linking user with organization: {}",
                e
            )),
        })?;

    Ok(org_user_link)
}

pub async fn user_in_org_query(
    org_id: PrefixedUuid<OrgPrefix>,
    user_id: PrefixedUuid<UserPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Option<Org>, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let orgs: Vec<Org> = orgs_columns::orgs
        .inner_join(orgs_users_columns::org_users)
        .filter(orgs_columns::id.eq(org_id))
        .filter(orgs_users_columns::user_id.eq(user_id))
        .select(Org::as_select())
        .load::<Org>(&mut conn)
        .await
        .map_err(|e| match e {
            _ => ServiceError::InternalServerError(format!("Error validating user_in_org: {}", e)),
        })?;

    Ok(orgs.into_iter().next())
}

pub async fn delete_org_query(
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(orgs_columns::orgs)
        .filter(orgs_columns::id.eq(org_id))
        .execute(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::InternalServerError(format!("Error deleting org by id: {}", e))
        })?;

    Ok(())
}

pub async fn update_org_query(org: Org, pg_pool: web::Data<PgPool>) -> Result<Org, ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org = diesel::update(orgs_columns::orgs.filter(orgs_columns::id.eq(org.id)))
        .set(&org)
        .get_result::<Org>(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError(format!("Error renaming org: {}", e)),
        })?;

    Ok(org)
}

pub async fn get_orgs_for_user_query(
    user_id: PrefixedUuid<UserPrefix>,
    pg_pool: web::Data<PgPool>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Org>, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let orgs = orgs_columns::orgs
        .inner_join(orgs_users_columns::org_users)
        .filter(orgs_users_columns::user_id.eq(user_id))
        .select(Org::as_select())
        .limit(limit)
        .offset(offset)
        .load::<Org>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::InternalServerError(format!("Error getting orgs for user: {}", e))
        })?;

    Ok(orgs)
}

pub async fn get_org_user_link_query(
    user_id: PrefixedUuid<UserPrefix>,
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<OrgUserLink, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org_user_link = orgs_users_columns::org_users
        .filter(orgs_users_columns::user_id.eq(user_id))
        .filter(orgs_users_columns::org_id.eq(org_id))
        .select(OrgUserLink::as_select())
        .first::<OrgUserLink>(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError(format!("Error getting org user link: {}", e)),
        })?;

    Ok(org_user_link)
}
