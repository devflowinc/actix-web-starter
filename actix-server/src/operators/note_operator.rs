use crate::{
    data::models::{Note, PgPool},
    errors::ServiceError,
    prefixes::{NotePrefix, OrgPrefix, PrefixedUuid},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
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

pub async fn delete_note_query(
    note_id: PrefixedUuid<NotePrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::notes::dsl as notes_columns;

    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(notes_columns::notes)
        .filter(notes_columns::id.eq(note_id))
        .execute(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error deleting note".to_string()))?;

    Ok(())
}

pub async fn update_note_body_query(
    note_id: PrefixedUuid<NotePrefix>,
    new_body: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Note, ServiceError> {
    use crate::data::schema::notes::dsl as notes_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let target = notes_columns::notes.filter(notes_columns::id.eq(note_id));

    let updated_note = diesel::update(target)
        .set(notes_columns::body.eq(new_body))
        .get_result::<Note>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error updating note body".to_string()))?;

    Ok(updated_note)
}

pub async fn get_note_by_id_query(
    note_id: PrefixedUuid<NotePrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Note, ServiceError> {
    use crate::data::schema::notes::dsl as notes_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let note = notes_columns::notes
        .filter(notes_columns::id.eq(note_id))
        .first::<Note>(&mut conn)
        .await
        .map_err(|_| ServiceError::NotFound)?;

    Ok(note)
}

pub async fn get_notes_for_org_query(
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Note>, ServiceError> {
    use crate::data::schema::notes::dsl as notes_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let notes = notes_columns::notes
        .filter(notes_columns::org_id.eq(org_id))
        .limit(limit)
        .offset(offset)
        .load::<Note>(&mut conn)
        .await
        .map_err(|_| ServiceError::InternalServerError("Error retrieving notes".to_string()))?;

    Ok(notes)
}
