// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Integer,
        username -> Text,
        master_password -> Text,
        recovery_code -> Text,
    }
}
