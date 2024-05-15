// @generated automatically by Diesel CLI.

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
    org_plans (id) {
        id -> Uuid,
        org_id -> Uuid,
        plan_id -> Uuid,
        next_billing_date -> Timestamp,
        start_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
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
        num_users -> Int4,
        price_per_month -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(org_plans -> orgs (org_id));
diesel::joinable!(org_plans -> plans (plan_id));
diesel::joinable!(org_users -> orgs (org_id));
diesel::joinable!(org_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(api_keys, org_plans, org_users, orgs, plans, users,);
