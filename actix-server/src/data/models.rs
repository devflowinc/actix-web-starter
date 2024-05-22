use super::schema::*;
use bb8_redis::{bb8, RedisConnectionManager};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type PgPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;
pub type RedisPool = bb8::Pool<RedisConnectionManager>;

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
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub name: Option<String>,
}

impl User {
    pub fn from_details(email: String, name: Option<String>) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
            name: name.map(|n| n.into()),
        }
    }

    pub fn from_details_with_id(id: uuid::Uuid, email: String, name: Option<String>) -> Self {
        User {
            id: id.into(),
            email: email.into(),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema, AsChangeset,
)]
#[schema(example = json!({
    "id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "name": "Trieve Team",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = orgs)]
pub struct Org {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Org {
    pub fn from_details(name: String) -> Self {
        Org {
            id: uuid::Uuid::new_v4(),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }

    pub fn from_details_with_id(id: uuid::Uuid, name: String) -> Self {
        Org {
            id: id.into(),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Deserialize, Serialize)]
#[ExistingTypePath = "crate::data::schema::sql_types::Perm"]
pub enum Perm {
    Subscription,
}

// TODO: Way to not have to update manually?
impl Perm {
    pub const ALL_PERMS: [Self; 1] = [Self::Subscription];
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[diesel(table_name = org_users_perms)]
pub struct OrgUserPerm {
    pub org_user_id: uuid::Uuid,
    pub perm: Option<Perm>,
    pub has: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "f1f1f1f1-f1f1-f1f1-f1f1-f1f1f1f1f1f1",
    "user_id": "8w8w8w8w-8w8w-8w8w-8w8w-8w8w8w8w8w8w",
    "org_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
}))]
#[diesel(table_name = org_users)]
pub struct OrgUserLink {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub org_id: uuid::Uuid,
}
impl OrgUserLink {
    pub fn from_details(user_id: uuid::Uuid, org_id: uuid::Uuid) -> Self {
        OrgUserLink {
            id: uuid::Uuid::new_v4(),
            user_id,
            org_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "b8b8b8b8-b8b8-b8b8-b8b8-b8b8b8b8b8b8",
    "stripe_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "num_users": 4,
    "num_deals": 5,
    "price_per_month": 40,
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name=plans)]
pub struct Plan {
    pub id: uuid::Uuid,
    pub stripe_id: String,
    pub num_users: i32,
    pub num_deals: i32,
    pub price_per_month: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Plan {
    pub fn from_details(
        stripe_id: String,
        num_users: i32,
        num_deals: i32,
        price_per_month: i32,
    ) -> Self {
        Plan {
            id: uuid::Uuid::new_v4(),
            stripe_id,
            num_users,
            num_deals,
            price_per_month,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "afafafaf-afaf-afaf-afaf-afafafafafaf",
    "user_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "name": "my-api-key",
    "blake3_hash": "blake3-hash",
    "created_at": "2021-01-01T00:00:00",
    "updated_at": "2021-01-01T00:00:00",
}))]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub blake3_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl ApiKey {
    pub fn from_details(user_id: uuid::Uuid, name: String, blake3_hash: String) -> Self {
        ApiKey {
            id: uuid::Uuid::new_v4(),
            user_id,
            name,
            blake3_hash,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}
