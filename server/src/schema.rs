// @generated automatically by Diesel CLI.

diesel::table! {
    users (username) {
        username -> Text,
        password -> Text,
        email -> Text,
    }
}
