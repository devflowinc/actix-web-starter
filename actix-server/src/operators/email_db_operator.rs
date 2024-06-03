use crate::{
    data::models::{Email, PgPool},
    errors::ServiceError,
    prefixes::{EmailPrefix, OrgPrefix, PrefixedUuid},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use lettre;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_email_query(
    org_id: PrefixedUuid<OrgPrefix>,
    email: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Email, ServiceError> {
    use crate::data::schema::emails::dsl as emails_columns;
    let mut conn = pg_pool.get().await.unwrap();
    email
        .parse::<lettre::Address>()
        .map_err(|_| ServiceError::BadRequest("Invalid email".to_string()))?;
    let new_email = Email::from_details(email, org_id);
    let email = diesel::insert_into(emails_columns::emails)
        .values(&new_email)
        .get_result::<Email>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating emails".to_string()))?;
    Ok(email)
}

pub async fn delete_email_query(
    email_id: PrefixedUuid<EmailPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::emails::dsl as emails_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(emails_columns::emails)
        .filter(emails_columns::id.eq(email_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting email".to_string()))?;

    Ok(())
}

pub async fn update_email_query(
    email_id: PrefixedUuid<EmailPrefix>,
    email: Option<String>,
    pg_pool: web::Data<PgPool>,
) -> Result<Email, ServiceError> {
    use crate::data::schema::emails::dsl as emails_columns;
    let mut conn = pg_pool.get().await.unwrap();
    if let Some(email) = &email {
        email
            .parse::<lettre::Address>()
            .map_err(|_| ServiceError::BadRequest("Invalid email".to_string()))?;
    }
    let target = emails_columns::emails.filter(emails_columns::id.eq(email_id));
    let updated_email = diesel::update(target)
        .set((email.map(|email| emails_columns::email.eq(email)),))
        .get_result::<Email>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating email".to_string()))?;
    Ok(updated_email)
}

pub async fn get_email_by_id(
    email_id: PrefixedUuid<EmailPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Email, ServiceError> {
    use crate::data::schema::emails::dsl as emails_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let email = emails_columns::emails
        .filter(emails_columns::id.eq(email_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(email)
}
