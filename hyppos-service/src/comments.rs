use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use actix_web::{web, Error as ActixWebError, HttpResponse, Responder};
use serde::Serialize;

use crate::models::{Comment, NewComment};
use crate::State;

pub fn find_comments_by_user_id(
    uid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<Vec<Comment>>, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    let all_comments = comments
        .filter(user_id.eq(uid))
        .load::<Comment>(conn)
        .optional()?;

    Ok(all_comments)
}

pub fn find_comments_by_file_id(
    fid: String,
    conn: &PgConnection,
) -> Result<Option<Vec<Comment>>, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    let all_comments = comments
        .filter(file_id.eq(fid))
        .load::<Comment>(conn)
        .optional()?;

    Ok(all_comments)
}

pub fn insert_new_comment(
    comment: &NewComment,
    conn: &PgConnection,
) -> Result<Comment, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    let _id = Uuid::new_v4();

    let new_comment = Comment {
        id: _id,
        parent_id: match comment.parent_id {
            Some(i) => i.to_owned(),
            None => _id,
        },
        user_id: comment.user_id.to_owned(),
        project_id: comment.project_id.to_owned(),
        commit_id: comment.commit_id.to_owned(),
        file_id: comment.file_id.to_owned(),
        line_no: comment.line_no,
        message: comment.message.to_owned(),
        created_at: Utc::now().to_owned(),
        is_deleted: false,
    };

    diesel::insert_into(comments)
        .values(&new_comment)
        .execute(conn)?;

    Ok(new_comment)
}

#[derive(Serialize)]
struct InsertResponse {
    status: String,
}

pub(crate) async fn insert_comment(
    state: web::Data<State>,
    new_comment: web::Json<NewComment>,
) -> Result<HttpResponse, ActixWebError> {
    dbg!("HI");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let resp = web::block(move || {
        insert_new_comment(&new_comment, &conn).expect("inserting new comment");
        Ok::<_, ()>(InsertResponse {
            status: "ok".to_string(),
        })
    })
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}
