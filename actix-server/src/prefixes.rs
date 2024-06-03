use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

use diesel::{deserialize::FromSql, pg::PgValue, serialize::ToSql};
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{openapi::ObjectBuilder, ToSchema};

pub trait Prefix:
    Clone
    + Debug
    + Default
    + Display
    + Serialize
    + for<'de> Deserialize<'de>
    + 'static
    + ToString
    + FromStr
{
}

#[derive(Clone, AsExpression, Debug, Default, FromSqlRow, Copy, PartialEq, Eq)]
#[diesel(sql_type = diesel::sql_types::Uuid)]
pub struct PrefixedUuid<P: Prefix> {
    pub prefix: P,
    pub id: uuid::Uuid,
}

impl<'__s, P: Prefix> ToSchema<'__s> for PrefixedUuid<P> {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema = ObjectBuilder::new()
            .schema_type(utoipa::openapi::SchemaType::String)
            .build();
        return ("PrefixedUuid", schema.into());
    }

    fn aliases() -> Vec<(&'__s str, utoipa::openapi::schema::Schema)> {
        vec![(
            "PrefixedUuid",
            ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .build()
                .into(),
        )]
    }
}

impl<P: Prefix> Display for PrefixedUuid<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.prefix, self.id)
    }
}

// Turn it into a string with prefix-uuid
impl<P: Prefix> Serialize for PrefixedUuid<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        format!("{}-{}", self.prefix.to_string(), self.id).serialize(serializer)
    }
}

impl<P: Prefix> FromStr for PrefixedUuid<P> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        // Combine all parts except the first
        let rest = parts[1..].join("-");

        let parsed_uuid = rest.parse().map_err(|_| ())?;

        Ok(PrefixedUuid {
            prefix: P::default(),
            id: parsed_uuid,
        })
    }
}

// Going off of this: https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
impl<P: Prefix> ToSql<diesel::sql_types::Uuid, diesel::pg::Pg> for PrefixedUuid<P> {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::ToSql::to_sql(
            &self.id as &dyn diesel::serialize::ToSql<diesel::sql_types::Uuid, diesel::pg::Pg>,
            out,
        )
    }
}

impl<P: Prefix> PrefixedUuid<P> {
    // TODO: make the prefix an enum, like in the unkey blog
    pub fn create(prefix: P) -> Self {
        PrefixedUuid {
            prefix,
            id: uuid::Uuid::new_v4(),
        }
    }
}

// Implement Deserialize manually
impl<'de, P> Deserialize<'de> for PrefixedUuid<P>
where
    P: Prefix,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('-').collect();

        // Combine all parts except the first
        let rest = parts[1..]
            .join("-")
            .parse()
            .map_err(serde::de::Error::custom)?;

        Ok(PrefixedUuid {
            prefix: P::default(),
            id: rest,
        })
    }
}

impl<P: Prefix + Default> FromSql<diesel::sql_types::Uuid, diesel::pg::Pg> for PrefixedUuid<P> {
    fn from_sql(raw: PgValue) -> diesel::deserialize::Result<Self> {
        let id = uuid::Uuid::from_slice(raw.as_bytes())?;
        Ok(PrefixedUuid {
            prefix: P::default(),
            id,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy, PartialEq, Eq, ToSchema)]
pub struct OrgPrefix;

impl Display for OrgPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "org")
    }
}

impl FromStr for OrgPrefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "org" {
            Ok(OrgPrefix)
        } else {
            Err(())
        }
    }
}

impl Prefix for OrgPrefix {}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy, ToSchema)]
pub struct OrgUserPrefix;

impl Display for OrgUserPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "orguser")
    }
}

impl FromStr for OrgUserPrefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "orguser" {
            Ok(OrgUserPrefix)
        } else {
            Err(())
        }
    }
}

impl Prefix for OrgUserPrefix {}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy, PartialEq, Eq, ToSchema)]
pub struct UserPrefix;

impl Display for UserPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user")
    }
}

impl FromStr for UserPrefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "user" {
            Ok(UserPrefix)
        } else {
            Err(())
        }
    }
}

impl Prefix for UserPrefix {}
