// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Integer,
        username -> Text,
        master_password -> Text,
        recovery_code -> Text,
        app_lock -> Integer,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        title -> Text,
        identifier -> Nullable<Text>,
        password -> Nullable<Text>,
        description -> Nullable<Text>,
        link -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        created_at -> BigInt,
        last_modified -> BigInt,
        client_id -> Integer,
    }
}

diesel::joinable!(items -> client (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    client,
    items,
);
