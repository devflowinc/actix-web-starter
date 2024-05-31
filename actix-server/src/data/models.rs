use crate::prefixes::{OrgPrefix, OrgUserPrefix, PrefixedUuid, UserPrefix};

use super::schema::*;
use bb8_redis::{bb8, RedisConnectionManager};
use diesel::expression::ValidGrouping;
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
    pub id: PrefixedUuid<UserPrefix>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub name: Option<String>,
}

impl User {
    pub fn from_details(email: String, name: Option<String>) -> Self {
        User {
            id: PrefixedUuid::create(UserPrefix),
            email: email.into(),
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
            name: name.map(|n| n.into()),
        }
    }

    pub fn from_details_with_id(
        id: PrefixedUuid<UserPrefix>,
        email: String,
        name: Option<String>,
    ) -> Self {
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
    pub id: PrefixedUuid<OrgPrefix>,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Org {
    pub fn from_details(name: String) -> Self {
        Org {
            id: PrefixedUuid::create(OrgPrefix),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }

    pub fn from_details_with_id(id: PrefixedUuid<OrgPrefix>, name: String) -> Self {
        Org {
            id: id.into(),
            name,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "org_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
    "size": 4.0,
    "active": true,
}))]
#[diesel(table_name = deals)]
pub struct Deal {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub org_id: uuid::Uuid,
    pub size: Option<f32>,
    pub active: bool,
}

impl Deal {
    pub fn from_details(
        org_id: uuid::Uuid,
        name: Option<String>,
        size: Option<f32>,
        active: bool,
    ) -> Self {
        Deal {
            id: uuid::Uuid::new_v4(),
            name,
            org_id,
            size,
            active,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UserRole {
    Owner = 2,
    Admin = 1,
    User = 0,
}

impl From<i32> for UserRole {
    fn from(role: i32) -> Self {
        match role {
            2 => UserRole::Owner,
            1 => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Owner => 2,
            UserRole::Admin => 1,
            UserRole::User => 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone, ToSchema)]
#[schema(example = json!({
    "id": "f1f1f1f1-f1f1-f1f1-f1f1-f1f1f1f1f1f1",
    "user_id": "8w8w8w8w-8w8w-8w8w-8w8w-8w8w8w8w8w8w",
    "org_id": "e3e3e3e3-e3e3-e3e3-e3e3-e3e3e3e3e3e3",
}))]
#[diesel(table_name = org_users)]
pub struct OrgUserLink {
    pub id: PrefixedUuid<OrgUserPrefix>,
    pub user_id: PrefixedUuid<UserPrefix>,
    pub org_id: PrefixedUuid<OrgPrefix>,
    pub role: i32,
}
impl OrgUserLink {
    pub fn from_details(
        user_id: PrefixedUuid<UserPrefix>,
        org_id: PrefixedUuid<OrgPrefix>,
        role: UserRole,
    ) -> Self {
        OrgUserLink {
            id: PrefixedUuid::create(OrgUserPrefix),
            user_id,
            org_id,
            role: role.into(),
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

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, ValidGrouping, ToSchema)]
#[diesel(table_name = invitations)]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub organization_id: PrefixedUuid<OrgPrefix>,
    pub used: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub role: i32,
}

// any type that implements Into<String> can be used to create Invitation
impl Invitation {
    pub fn from_details(
        email: String,
        organization_id: PrefixedUuid<OrgPrefix>,
        role: i32,
    ) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            email,
            organization_id,
            used: false,
            created_at: chrono::Utc::now().naive_local(),
            updated_at: chrono::Utc::now().naive_local(),
            role,
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
    pub user_id: PrefixedUuid<UserPrefix>,
    pub name: String,
    pub blake3_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl ApiKey {
    pub fn from_details(
        user_id: PrefixedUuid<UserPrefix>,
        name: String,
        blake3_hash: String,
    ) -> Self {
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
