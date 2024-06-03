use crate::{
    data::models::{Contact, PgPool},
    errors::ServiceError,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_contact_query(
    org_id: uuid::Uuid,
    first_name: String,
    last_name: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Contact, ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_contact = Contact::from_details(org_id, first_name, last_name);
    let contact = diesel::insert_into(contacts_columns::contacts)
        .values(&new_contact)
        .get_result::<Contact>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating contact".to_string()))?;
    Ok(contact)
}

pub async fn delete_contact_query(
    contact_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(contacts_columns::contacts)
        .filter(contacts_columns::id.eq(contact_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting contact".to_string()))?;

    Ok(())
}

pub async fn update_contact_query(
    contact_id: uuid::Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    pg_pool: web::Data<PgPool>,
) -> Result<Contact, ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let target = contacts_columns::contacts.filter(contacts_columns::id.eq(contact_id));
    let updated_contact = diesel::update(target)
        .set((
            first_name.map(|first_name| contacts_columns::first_name.eq(first_name)),
            last_name.map(|last_name| contacts_columns::last_name.eq(last_name)),
        ))
        .get_result::<Contact>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating contact".to_string()))?;
    Ok(updated_contact)
}

pub async fn get_contact_by_id(
    contact_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<Contact, ServiceError> {
    use crate::data::schema::contacts::dsl as contacts_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let contact = contacts_columns::contacts
        .filter(contacts_columns::id.eq(contact_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(contact)
}
