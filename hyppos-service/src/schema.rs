table! {
    comments (id) {
        id -> Text,
        parent_id -> Nullable<Text>,
        message -> Text,
        created_at -> Timestamptz,
        is_deleted -> Bool,
    }
}
