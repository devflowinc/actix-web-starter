use crate::{
    data::models::{Note, PgPool},
    errors::ServiceError,
    prefixes::{OrgPrefix, PrefixedUuid},
};
use actix_web::web;
use diesel_async::RunQueryDsl;

pub async fn create_note_query(
    title: String,
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Note, ServiceError> {
    use crate::data::schema::notes::dsl as notes_columns;

    let mut conn = pg_pool.get().await.unwrap();
    let note = Note::from_title(title, org_id);

    let note = diesel::insert_into(notes_columns::notes)
        .values(&note)
        .get_result::<Note>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::InternalServerError(
                format!("Error creating note for create_note_query: {}", e).to_string(),
            )
        })?;

    Ok(note)
}
