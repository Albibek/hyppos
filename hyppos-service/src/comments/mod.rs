use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

pub mod models;
pub mod schema;

pub fn find_comment_by_id(
    cid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Comment>, diesel::result::Error> {
    use crate::comments::schema::comments::dsl::*;

    let comment = comments
        .filter(id.eq(cid))
        .first::<models::Comment>(conn)
        .optional()?;

    Ok(comment)
}

pub fn insert_new_comment(
    comment: &models::NewComment,
    conn: &PgConnection,
) -> Result<models::Comment, diesel::result::Error> {
    use crate::comments::schema::comments::dsl::*;

    let _id = Uuid::new_v4();

    let new_comment = models::Comment {
        id: _id,
        parent_id: match comment.parent_id {
            Some(i) => i.to_owned(),
            None => _id,
        },
        message: comment.message.to_owned(),
        user_id: comment.user_id,
        project_id: comment.project_id,
        hash: comment.hash.to_owned(),
        file_id: comment.file_id,
        line_no: comment.line_no,
        is_deleted: false,

        created_at: Utc::now().to_owned(),
    };

    diesel::insert_into(comments).values(&new_comment).execute(conn)?;

    Ok(new_comment)
}
