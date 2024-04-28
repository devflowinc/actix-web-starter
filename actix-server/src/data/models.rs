use super::schema::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type Pool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "email": "developers@trieve.ai",
    "name": "Trieve Team",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = users)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>>(email: S, name: Option<S>) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            name: name.map(|n| n.into()),
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }

    pub fn from_details_with_id<S: Into<String>, T: Into<uuid::Uuid>>(
        id: T,
        email: S,
        name: Option<S>,
    ) -> Self {
        User {
            id: id.into(),
            email: email.into(),
            name: name.map(|n| n.into()),
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}
