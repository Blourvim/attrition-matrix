use actix_web::{HttpResponse, Responder, post, web};

use crate::api::dto::{
    matrix::{AttritionMatrixQuery, AttritionMatrixResponse},
    sdk_search::SdkSearchResponse,
};

async fn get_matrix(attrition_matrix_query: web::Query<AttritionMatrixQuery>) -> impl Responder {
    let query = attrition_matrix_query.into_inner();
    // now per requested sdk, we can calculate and fetch
    let response = AttritionMatrixResponse {};
    HttpResponse::Ok().json(response)
}

async fn search_sdk() -> impl Responder {
    let response: SdkSearchResponse = SdkSearchResponse { sdks: Vec::new() };
    HttpResponse::Ok().json(response)
}

async fn get_example_apps() -> impl Responder {
    let response: SdkSearchResponse = SdkSearchResponse { sdks: Vec::new() };
    HttpResponse::Ok().json(response)
}
pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
        .route("/search_sdk", web::get().to(search_sdk))
        .route("/get_matrix", web::get().to(get_matrix))
        .route("/get_example_apps", web::get().to(get_example_apps))
}
