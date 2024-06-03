use crate::{
    data::models::{PgPool, Phone},
    errors::ServiceError,
    prefixes::{OrgPrefix, PhonePrefix, PrefixedUuid},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

const MIN_PHONE_LENGTH: usize = 9;
// 9 is the minimum length of a phone number (e.g. +1234567890) in E.164 format

#[tracing::instrument(skip(pg_pool))]
pub async fn create_phone_query(
    org_id: PrefixedUuid<OrgPrefix>,
    phone: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Phone, ServiceError> {
    use crate::data::schema::phones::dsl as phones_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let new_phone = Phone::from_details(org_id, phone);
    if new_phone.number.len() < MIN_PHONE_LENGTH {
        return Err(ServiceError::BadRequest("Invalid phone number".to_string()));
    }
    let phone = diesel::insert_into(phones_columns::phones)
        .values(&new_phone)
        .get_result::<Phone>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error creating phone".to_string()))?;
    Ok(phone)
}

pub async fn delete_phone_query(
    phone_id: PrefixedUuid<PhonePrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::phones::dsl as phones_columns;
    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(phones_columns::phones)
        .filter(phones_columns::id.eq(phone_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting phone".to_string()))?;

    Ok(())
}

pub async fn update_phone_query(
    phone_id: PrefixedUuid<PhonePrefix>,
    phone: Option<String>,
    pg_pool: web::Data<PgPool>,
) -> Result<Phone, ServiceError> {
    use crate::data::schema::phones::dsl as phones_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let target = phones_columns::phones.filter(phones_columns::id.eq(phone_id));
    if let Some(phone) = &phone {
        if phone.len() < MIN_PHONE_LENGTH {
            return Err(ServiceError::BadRequest("Invalid phone number".to_string()));
        }
    }
    let updated_phone = diesel::update(target)
        .set((phone.map(|phone| phones_columns::number.eq(phone)),))
        .get_result::<Phone>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating phone".to_string()))?;
    Ok(updated_phone)
}

pub async fn get_phone_by_id(
    phone_id: PrefixedUuid<PhonePrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Phone, ServiceError> {
    use crate::data::schema::phones::dsl as phones_columns;
    let mut conn = pg_pool.get().await.unwrap();
    let phone = phones_columns::phones
        .filter(phones_columns::id.eq(phone_id))
        .first(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;
    Ok(phone)
}
