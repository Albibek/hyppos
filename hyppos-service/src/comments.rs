use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use actix_session::Session;
use actix_web::{web, Error as ActixWebError, HttpResponse, Responder};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::models::{Comment, NewComment, NewCommentWithID};
use crate::State;
use crate::{github_types, projects, users};

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
    comment: &NewCommentWithID,
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

#[derive(Deserialize)]
pub struct CommentsQuery {
    file_id: String,
}

pub(crate) async fn get_comments(
    state: web::Data<State>,
    web::Query(query): web::Query<CommentsQuery>,
) -> Result<HttpResponse, ActixWebError> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let resp = web::block(move || {
        let comments = find_comments_by_file_id(query.file_id, &conn)?;
        Ok::<_, DieselError>(comments)
    })
    .await?;

    match resp {
        Some(comments) => Ok(HttpResponse::Ok().json(comments)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Serialize)]
struct InsertResponse {
    status: String,
}

pub(crate) async fn insert_comment(
    session: Session,
    state: web::Data<State>,
    new_comment: web::Json<NewComment>,
) -> Result<HttpResponse, ActixWebError> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = match session.get::<github_types::User>("user")? {
        Some(user) => user,
        None => return Ok(HttpResponse::Forbidden().finish()),
    };

    let resp = web::block(move || {
        let db_user = users::find_user_by_ext_id(user.id, &conn)?;
        if db_user.is_none() {
            users::insert_new_user(user.id, &conn)?;
        }

        let db_user = users::find_user_by_ext_id(user.id, &conn)?.unwrap();
        let new_comment = NewCommentWithID {
            user_id: db_user.id,
            parent_id: new_comment.parent_id,
            project_id: new_comment.project_id,
            commit_id: new_comment.commit_id.to_owned(),
            file_id: new_comment.file_id.to_owned(),
            line_no: new_comment.line_no,
            message: new_comment.message.to_owned(),
        };
        insert_new_comment(&new_comment, &conn)?;
        Ok::<_, DieselError>(InsertResponse {
            status: "ok".to_string(),
        })
    })
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}
