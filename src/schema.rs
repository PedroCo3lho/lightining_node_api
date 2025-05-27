// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Int4,
        #[max_length = 66]
        public_key -> Varchar,
        alias -> Text,
        capacity -> Numeric,
        first_seen -> Timestamp,
        updated_at -> Timestamp,
    }
}
