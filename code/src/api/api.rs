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
    format!("Hello {}!", 1)
}

#[get("/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}
