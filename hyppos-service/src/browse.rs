use actix_web::{HttpResponse, Responder};

pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
