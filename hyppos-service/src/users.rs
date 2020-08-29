use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use actix_web::{web, Error as ActixWebError, HttpResponse, Responder};
use serde::Serialize;

use crate::models::User;
use crate::State;

use crate::projects;

pub fn find_user_by_id(
    uid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users.filter(id.eq(uid)).first::<User>(conn).optional()?;

    Ok(user)
}

pub fn find_user_by_ext_id(
    eid: i64,
    conn: &PgConnection,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(external_id.eq(eid))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(ext_id: i64, conn: &PgConnection) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let _id = Uuid::new_v4();

    let new_user = User {
        id: _id,
        external_id: ext_id,
        created_at: Utc::now().to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub(crate) async fn get_projects(
    state: web::Data<State>,
    path: web::Path<(Uuid,)>,
) -> Result<HttpResponse, ActixWebError> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let resp = web::block(move || {
        let projects = projects::find_projects_by_user_id(path.0, &conn).expect("select for projects");
        Ok::<_, ()>(projects)
    })
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}