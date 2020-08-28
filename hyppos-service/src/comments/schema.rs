table! {
    comments (id) {
        id -> Uuid,
        parent_id -> Uuid,
        user_id -> Uuid,
        project_id -> Uuid,
        hash -> Nullable<Text>,
        file_id -> Uuid,
        line_no -> Nullable<Int8>,
        message -> Text,
        created_at -> Timestamptz,
        is_deleted -> Bool,
    }
}
