use crate::{
    data::models::{Pool, User},
    errors::ServiceError,
};
use actix_web::web;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pool))]
pub async fn get_user_by_id_query(
    user_id: &uuid::Uuid,
    pool: web::Data<Pool>,
) -> Result<User, ServiceError> {
    use crate::data::schema::users::dsl as users_columns;

    let mut conn = pool.get().await.unwrap();

    let user: User = users_columns::users
        .filter(users_columns::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .map_err(|_| {
            ServiceError::BadRequest(
                "Error loading user for get_user_by_id_query"
                    .to_string(),
            )
        })?;

    Ok(user)
}

#[tracing::instrument(skip(pool))]
pub async fn create_user_query(
    user_id: uuid::Uuid,
    email: String,
    name: Option<String>,
    pool: web::Data<Pool>,
) -> Result<User, ServiceError> {
    use crate::data::schema::users::dsl as users_columns;

    let mut conn = pool.get().await.unwrap();

    let user = User::from_details_with_id(user_id, email, name);

    let user = diesel::insert_into(users_columns::users)
        .values(&user)
        .get_result::<User>(&mut conn)
        .await
        .map_err(|_| {
            ServiceError::BadRequest(
                "Error creating user for create_user_query"
                    .to_string(),
            )
        })?;

    Ok(user)
}
