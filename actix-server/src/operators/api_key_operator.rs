use crate::{
    data::models::{ApiKey, PgPool},
    errors::ServiceError,
};
use actix_web::web;
use diesel_async::RunQueryDsl;
use rand::{distributions::Alphanumeric, Rng};

#[tracing::instrument]
pub fn generate_api_key() -> String {
    let rng = rand::thread_rng();
    let api_key: String = format!(
        "tr-{}",
        rng.sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>()
    );

    api_key
}

#[tracing::instrument]
pub fn hash_api_key(password: &str) -> String {
    blake3::hash(password.as_bytes()).to_string()
}

#[tracing::instrument(skip(pg_pool))]
pub async fn create_api_key_query(
    user_id: uuid::Uuid,
    name: String,
    pg_pool: web::Data<PgPool>,
) -> Result<String, ServiceError> {
    use crate::data::schema::api_keys::dsl as api_keys_columns;

    let raw_api_key = generate_api_key();
    let blake3_hash = hash_api_key(&raw_api_key);

    let mut conn = pg_pool.get().await.unwrap();

    let api_key_struct = ApiKey::from_details(user_id, name, blake3_hash.clone());

    diesel::insert_into(api_keys_columns::api_keys)
        .values(&api_key_struct)
        .execute(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::BadRequest(format!(
                "Error creating api key for create_api_key_query: {}",
                e
            ))
        })?;

    Ok(raw_api_key)
}
