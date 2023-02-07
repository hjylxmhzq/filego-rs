// @generated automatically by Diesel CLI.

diesel::table! {
    users (username) {
        username -> Text,
        password -> Text,
        email -> Text,
        user_type -> Integer,
        user_root -> Text,
    }
}
