// @generated automatically by Diesel CLI.

diesel::table! {
    gallery_images (file_path) {
        file_path -> Text,
        username -> Text,
        size -> Integer,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        format -> Nullable<Text>,
        updated_at -> Text,
    }
}

diesel::table! {
    users (username) {
        username -> Text,
        password -> Text,
        email -> Text,
        user_type -> Integer,
        user_root -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    gallery_images,
    users,
);
