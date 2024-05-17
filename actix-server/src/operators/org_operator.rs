use crate::{
    data::models::{Org, OrgUserLink, OrgUserPerm, Perm, PgPool},
    errors::ServiceError,
    handlers::auth_handler::AuthedUser,
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

    // TODO: Maybe want a db transaction for all 3 steps?
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

    let Ok(link) = link_org_with_user(org.id, authed_user.id, &pg_pool).await else {
        return Err(ServiceError::InternalServerError(
            "Error linking user with organization".to_string(),
        ));
    };

    let Ok(_) = grant_user_all_perms(link.id, &pg_pool).await else {
        return Err(ServiceError::InternalServerError(
            "Error granting user all permissions".to_string(),
        ));
    };

    Ok(org)
}

pub async fn user_in_org(
    org_id: uuid::Uuid,
    user_id: uuid::Uuid,
    pg_pool: &PgPool,
) -> Result<bool, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let user_in_org = diesel::select(diesel::dsl::exists(
        orgs_users_columns::org_users
            .filter(orgs_users_columns::org_id.eq(org_id))
            .filter(orgs_users_columns::user_id.eq(user_id)),
    ))
    .get_result::<bool>(&mut conn)
    .await
    .map_err(|_| ServiceError::InternalServerError("Error checking if user in org".to_string()))?;

    Ok(user_in_org)
}

pub async fn delete_org_query(
    org_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(orgs_columns::orgs)
        .filter(orgs_columns::id.eq(org_id))
        .execute(&mut conn)
        .await
        .map_err(|_| {
            ServiceError::InternalServerError("Error deleting organization".to_string())
        })?;

    Ok(())
}

#[tracing::instrument(skip(pg_pool))]
pub async fn link_org_with_user(
    org_id: uuid::Uuid,
    user_id: uuid::Uuid,
    pg_pool: &PgPool,
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

pub async fn grant_user_all_perms(
    org_user_id: uuid::Uuid,
    pg_pool: &PgPool,
) -> Result<Vec<OrgUserPerm>, ServiceError> {
    use crate::data::schema::org_users_perms::dsl as orgs_users_perms_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let perms = build_all_perms(org_user_id);

    diesel::insert_into(orgs_users_perms_columns::org_users_perms)
        .values(&perms)
        .execute(&mut conn)
        .await
        .map_err(|_| {
            ServiceError::InternalServerError("Error granting user all perms".to_string())
        })?;

    Ok(perms)
}

pub fn build_all_perms(org_user_id: uuid::Uuid) -> Vec<OrgUserPerm> {
    let perm_list: Vec<OrgUserPerm> = Perm::ALL_PERMS
        .map(|perm| OrgUserPerm {
            org_user_id,
            perm: Some(perm),
            has: true,
        })
        .to_vec();

    perm_list
}

pub async fn get_org_by_id_query(
    org_id: uuid::Uuid,
    pg_pool: &PgPool,
) -> Result<Org, ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org = orgs_columns::orgs
        .filter(orgs_columns::id.eq(org_id))
        .first::<Org>(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;

    Ok(org)
}

pub async fn rename_org_query(
    org_id: uuid::Uuid,
    new_name: String,
    pg_pool: &PgPool,
) -> Result<Org, ServiceError> {
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let org = diesel::update(orgs_columns::orgs.filter(orgs_columns::id.eq(org_id)))
        .set(orgs_columns::name.eq(new_name))
        .get_result::<Org>(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;

    Ok(org)
}

pub async fn get_my_orgs_query(
    user_id: uuid::Uuid,
    pg_pool: &PgPool,
    limit: Option<i64>,
    skip: Option<i64>,
) -> Result<Vec<Org>, ServiceError> {
    use crate::data::schema::org_users::dsl as orgs_users_columns;
    use crate::data::schema::orgs::dsl as orgs_columns;

    let mut conn = pg_pool.get().await.unwrap();

    // Set default values if limit and skip are None
    let limit = limit.unwrap_or(30); // Default limit of 10
    let skip = skip.unwrap_or(0); // Default skip of 0

    let orgs = orgs_columns::orgs
        .inner_join(orgs_users_columns::org_users)
        .filter(orgs_users_columns::user_id.eq(user_id))
        .select(Org::as_select())
        .limit(limit)
        .offset(skip)
        .load::<Org>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error getting user's orgs".to_string()))?;

    Ok(orgs)
}
