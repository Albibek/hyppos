table! {
    comments (id) {
        id -> Uuid,
        parent_id -> Uuid,
        user_id -> Uuid,
        project_id -> Uuid,
        commit_id -> Text,
        file_id -> Text,
        line_no -> Nullable<Int8>,
        message -> Text,
        created_at -> Timestamptz,
        is_deleted -> Bool,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        user_id -> Uuid,
        external_id -> Int8,
        created_at -> Timestamptz,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        external_id -> Int8,
        created_at -> Timestamptz,
    }
}

joinable!(comments -> projects (project_id));
joinable!(comments -> users (user_id));
joinable!(projects -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, projects, users,);
