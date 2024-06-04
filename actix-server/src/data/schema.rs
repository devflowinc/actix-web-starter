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
    contacts (id) {
        id -> Uuid,
        org_id -> Uuid,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    deals (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        org_id -> Uuid,
        size -> Nullable<Float4>,
        active -> Bool,
    }
}

diesel::table! {
    invitations (id) {
        id -> Uuid,
        #[max_length = 100]
        email -> Varchar,
        organization_id -> Uuid,
        used -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        role -> Int4,
    }
}

diesel::table! {
    notes (id) {
        id -> Uuid,
        title -> Text,
        body -> Text,
        org_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    links (id) {
        id -> Uuid,
        link -> Text,
        org_id -> Uuid,
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
diesel::joinable!(contacts -> orgs (org_id));
diesel::joinable!(deals -> orgs (org_id));
diesel::joinable!(invitations -> orgs (organization_id));
diesel::joinable!(notes -> orgs (org_id));
diesel::joinable!(links -> orgs (org_id));
diesel::joinable!(org_users -> orgs (org_id));
diesel::joinable!(org_users -> users (user_id));
diesel::joinable!(subscriptions -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    contacts,
    deals,
    invitations,
    notes,
    links,
    org_users,
    orgs,
    plans,
    subscriptions,
    users,
);
