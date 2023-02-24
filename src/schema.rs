// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Oid,
        preferred_username -> Text,
        name -> Text,
        summary -> Text,
        followers -> Array<Text>,
        following -> Array<Text>,
        public_key -> Text,
        private_key -> Nullable<Text>,
        published -> Timestamptz,
        email -> Text,
    }
}
