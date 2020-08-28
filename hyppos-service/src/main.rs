mod auth;
#[allow(dead_code)]
mod github;
#[allow(dead_code)]
mod github_types;

mod comments;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use actix_session::{CookieSession, Session};

use crate::auth::AuthState;

pub(crate) async fn index(session: Session) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello, {}",
        session
            .get("token")
            .unwrap_or(None)
            .unwrap_or("mr. anonymous".to_string())
    ))
}

#[derive(Clone)]
struct State {
    auth: AuthState,
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

        Self {
            auth: auth::configure(),
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
            .service(fs::Files::new("/static", "../static"))
            .route("/auth/login", web::get().to(auth::login))
            .route("/auth/callback", web::get().to(auth::callback))
            .route("/auth", web::post().to(auth::index))
            .route("/favicon.ico", web::get().to(index))
            .route("/", web::get().to(index))
            .service(
                // this must be at the end of all routes
                web::scope("/")
                    .wrap(auth::AuthCheck)
                    .route("/comments", web::post().to(comments::insert_comment))
                    .route("/auth/logout", web::get().to(auth::logout)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
