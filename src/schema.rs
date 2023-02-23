// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "citext"))]
    pub struct Citext;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Citext;

    users (id) {
        id -> Oid,
        preferred_username -> Citext,
        name -> Text,
        summary -> Text,
        followers -> Array<Nullable<Text>>,
        following -> Array<Nullable<Text>>,
        public_key -> Text,
        private_key -> Nullable<Text>,
        published -> Timestamptz,
        email -> Text,
    }
}
