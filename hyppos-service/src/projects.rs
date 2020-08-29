use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use actix_session::Session;

use actix_web::{web, Error as ActixWebError, HttpResponse, Responder};
use diesel::result::Error as DieselError;
use serde::Serialize;

use crate::models::{NewProject, NewProjectWithID, Project};
use crate::State;

use crate::{github_types, users};

pub fn find_project_by_id(
    pid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<Project>, diesel::result::Error> {
    use crate::schema::projects::dsl::*;

    let project = projects
        .filter(id.eq(pid))
        .first::<Project>(conn)
        .optional()?;

    Ok(project)
}

pub fn find_projects_by_user_id(
    uid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<Vec<Project>>, diesel::result::Error> {
    use crate::schema::projects::dsl::*;

    let all_projects = projects
        .filter(user_id.eq(uid))
        .load::<Project>(conn)
        .optional()?;

    Ok(all_projects)
}

pub fn insert_new_project(
    project: &NewProjectWithID,
    conn: &PgConnection,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::projects::dsl::*;

    let _id = Uuid::new_v4();

    let new_project = Project {
        id: _id,
        user_id: project.user_id,
        external_id: project.external_id,
        created_at: Utc::now().to_owned(),
        name: project.name.clone(),
    };

    diesel::insert_into(projects)
        .values(&new_project)
        .execute(conn)?;

    Ok(new_project)
}

#[derive(Serialize)]
struct InsertResponse {
    status: String,
}

pub(crate) async fn insert_project(
    session: Session,
    state: web::Data<State>,
    new_project: web::Json<NewProject>,
) -> Result<HttpResponse, ActixWebError> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = if let Some(user) = session.get::<github_types::User>("user").unwrap() {
        user
    } else {
        return Ok(HttpResponse::Forbidden().finish());
    };

    let token = session.get::<String>("token").unwrap().unwrap();

    let repos: Vec<github_types::RepoDetails> = state
        .github
        .for_token(&token)
        //.get_user_repos(&user.login)
        .get_user_repos("Albibek")
        .await
        .unwrap();

    let repo_pos = repos
        .iter()
        .position(|ref r| r.id == new_project.external_id)
        .unwrap();

    let repo_name = repos[repo_pos].name.clone();

    let resp = web::block(move || {
        let db_user = users::find_user_by_ext_id(user.id, &conn).expect("finding user by ID");
        if db_user.is_none() {
            users::insert_new_user(user.id, &conn)?;
        }

        let db_user = users::find_user_by_ext_id(user.id, &conn)
            .expect("finding user by ID")
            .unwrap();
        let new_project = NewProjectWithID {
            external_id: new_project.external_id,
            user_id: db_user.id.to_owned(),
            name: repo_name,
        };
        insert_new_project(&new_project, &conn).expect("inserting new project");
        Ok::<_, DieselError>(InsertResponse {
            status: "ok".to_string(),
        })
    })
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}

pub(crate) async fn get_projects(
    session: Session,
    state: web::Data<State>,
) -> Result<HttpResponse, ActixWebError> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = if let Some(user) = session.get::<github_types::User>("user").unwrap() {
        user
    } else {
        return Ok(HttpResponse::Forbidden().finish());
    };

    let resp = web::block(move || {
        let db_user = users::find_user_by_ext_id(user.id, &conn).expect("finding user by ID");
        //  .unwrap();
        //let projects = find_projects_by_user_id(db_user.id, &conn)
        //.expect("select for projects")
        //.unwrap();

        let projects: Option<Vec<Project>> = match db_user {
            None => Some(vec![]),
            Some(u) => find_projects_by_user_id(u.id, &conn).expect("select for projects"),
        };

        Ok::<_, DieselError>(projects)
    })
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}
