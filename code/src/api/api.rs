use std::{
    fs,
    path::{self, Path},
};

use actix_web::{
    App, HttpServer, Responder, get,
    web::{self, Html},
};

#[get("/")]
pub async fn index() -> impl Responder {
    // todo: this should fail in startup instead
    let index_content =
        fs::read_to_string("./frontend/src/public/index.html").expect("missing index");
    Html::new(index_content)
}

#[get("/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}
