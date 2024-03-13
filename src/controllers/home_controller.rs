
use actix_web::{HttpRequest, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    name: String,
}



pub async fn home(req:HttpRequest) -> impl Responder {
    let mut tera = Tera::new("../views/**/*").unwrap();

    //let name = _req.match_info().get("name").unwrap();
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    println!("{:?}",params);
    let name = &params.name;
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    let rendered = tera.render("index.html", &ctx).unwrap();
// handling error with except macros
    HttpResponse::Ok().body(rendered)
}