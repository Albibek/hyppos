mod auth;
mod browse;

use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use actix_session::{CookieSession, Session};

use crate::auth::{auth_handler, AuthState};
pub(crate) async fn index(session: Session) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello, {}",
        session
            .get("token")
            .unwrap_or(None)
            .unwrap_or("mr. anonymous".to_string())
    ))
}

use actix::Actor;

#[derive(Clone)]
struct State {
    auth: AuthState,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            auth: auth::configure(),
        }
    }
}

static BASE_URL: &str = "http://127.0.0.1:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //  std::env::set_var(
    //"RUST_LOG",
    //"actix_example=info,actix_web=info,actix_http=info,actix_service=info",
    //  );
    env_logger::init();

    let state = State::new();
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(auth::AuthCheck)
            .wrap(
                CookieSession::private(&[0; 32])
                    .secure(false)
                    .max_age(60)
                    .name("session"),
            )
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "../static"))
            .route("/auth/login", web::get().to(auth::login))
            .route("/auth/login", web::post().to(auth::login))
            .route("/auth/logout", web::post().to(auth::logout))
            .route("/auth/callback", web::get().to(auth::callback))
            .route("/auth", web::post().to(auth::index))
            .route("/comments", web::get().to(index))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
