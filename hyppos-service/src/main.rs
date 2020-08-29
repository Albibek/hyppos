mod auth;
mod gh_proxy;
#[allow(dead_code)]
mod github;
#[allow(dead_code)]
mod github_types;

mod comments;
mod models;
mod projects;
mod schema;
mod users;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use actix_session::{CookieSession, Session};

use actix_cors::Cors;

use crate::auth::AuthState;
use crate::github::GithubClient;

pub(crate) async fn index(session: Session, state: web::Data<State>) -> impl Responder {
    let user = session.get::<github_types::User>("user").unwrap();
    if let Some(user) = user {
        HttpResponse::Ok().body(format!("Hello, {}", user.login))
    } else {
        HttpResponse::Ok().body("Hello, mr. anonymous")
    }
}

#[derive(Clone)]
struct State {
    auth: AuthState,
    github: GithubClient,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl State {
    pub(crate) fn new() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url).unwrap();
        // set up database connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let gh_url = url::Url::parse("https://api.github.com").unwrap();
        Self {
            auth: auth::configure(),
            github: GithubClient::with_baseurl(gh_url),
            pool,
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let state = State::new();
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(
                CookieSession::private(&[0; 32])
                    .secure(false)
                    .max_age(60)
                    .name("session"),
            )
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    //.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    //.allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .service(fs::Files::new("/static", "../static"))
            .route("/auth/login", web::get().to(auth::login))
            .route("/auth/callback", web::get().to(auth::callback))
            .route("/auth", web::post().to(auth::index))
            .route("/favicon.ico", web::get().to(index))
            .route("/", web::get().to(index))
            .route("/comments", web::get().to(comments::get_comments))
            .route("/gh/repos", web::get().to(gh_proxy::get_repos))
            .route("/gh/repos/{name}", web::get().to(gh_proxy::list_repo_branches))
            .route("/gh/repos/{name}/branch/{branch}", web::get().to(gh_proxy::list_repo_contents))
            .route("/gh/repos/{name}/dirs/{hash}", web::get().to(gh_proxy::list_directory))
            .route("/gh/repos/{name}/files/{hash}", web::get().to(gh_proxy::get_file))
            .service(
                // this must be at the end of all routes
                web::scope("/")
                    .wrap(auth::AuthCheck)
                    .route("/comments", web::post().to(comments::insert_comment))
                    .route(
                        "/user/{user_id}/projects",
                        web::get().to(users::get_projects),
                    )
                    .route("/projects", web::get().to(projects::get_projects))
                    .route("/projects", web::post().to(projects::insert_project))
                    .route("/auth/logout", web::get().to(auth::logout)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
