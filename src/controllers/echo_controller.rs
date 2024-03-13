


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}