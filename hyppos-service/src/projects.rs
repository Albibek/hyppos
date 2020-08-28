use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use actix_web::{web, Error as ActixWebError, HttpResponse, Responder};
use serde::Serialize;

use crate::models::Project;
use crate::State;

pub fn find_project_by_id(
    pid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<Project>, diesel::result::Error> {
    use crate::schema::projects::dsl::*;

    let project = projects
        .filter(id.eq(pid))
        .first::<Project>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_project(
    uid: uuid::Uuid,
    ext_id: i64,
    conn: &PgConnection,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::projects::dsl::*;

    let _id = Uuid::new_v4();

    let new_project = Project {
        id: _id,
        user_id: uid.to_owned,
        external_id: ext_id,
        created_at: Utc::now().to_owned(),
    };

    diesel::insert_into(projects)
        .values(&new_project)
        .execute(conn)?;

    Ok(new_user)
}
