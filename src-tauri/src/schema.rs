// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Integer,
        username -> Text,
        master_password -> Text,
        recovery_code -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        identifier -> Nullable<Text>,
        password -> Nullable<Text>,
        description -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        client_id -> Integer,
    }
}

diesel::joinable!(items -> client (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    client,
    items,
);
