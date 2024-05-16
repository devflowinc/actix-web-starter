// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "perm"))]
    pub struct Perm;
}

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Text,
        blake3_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    org_users (id) {
        id -> Uuid,
        user_id -> Uuid,
        org_id -> Uuid,
        role -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Perm;

    org_users_perms (org_user_id) {
        org_user_id -> Uuid,
        perm -> Nullable<Perm>,
        has -> Bool,
    }
}

diesel::table! {
    orgs (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    plans (id) {
        id -> Uuid,
        stripe_id -> Text,
        num_users -> Int4,
        num_deals -> Int4,
        price_per_month -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Uuid,
        stripe_id -> Text,
        org_id -> Uuid,
        plan_id -> Uuid,
        stripe_plan_id -> Text,
        next_billing_date -> Timestamp,
        start_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(org_users -> orgs (org_id));
diesel::joinable!(org_users -> users (user_id));
diesel::joinable!(org_users_perms -> org_users (org_user_id));
diesel::joinable!(subscriptions -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    org_users,
    org_users_perms,
    orgs,
    plans,
    subscriptions,
    users,
);
