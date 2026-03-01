use actix_web::{HttpResponse, Responder, web};

use crate::api::dto::{
    example_apps,
    matrix::{AttritionMatrixQuery, AttritionMatrixResponse},
    sdk_search::{SdkSearchQuery, SdkSearchResponse},
};

async fn get_matrix(attrition_matrix_query: web::Query<AttritionMatrixQuery>) -> impl Responder {
    let query = attrition_matrix_query.into_inner();
    // now per requested sdk, we can calculate and fetch
    let response: AttritionMatrixResponse = AttritionMatrixResponse::new(5);
    HttpResponse::Ok().body(response.to_html())
}

async fn search_sdk(search_query: web::Query<SdkSearchQuery>) -> impl Responder {
    let html_response = SdkSearchResponse::new(search_query.into_inner().search).to_html();
    HttpResponse::Ok().body(html_response)
}

async fn get_example_apps(
    example_apps_query: web::Query<example_apps::ExampleAppsQuery>,
) -> impl Responder {
    let query = example_apps_query.into_inner();

    let example_apps: example_apps::ExampleApps = example_apps::ExampleApps::new(query.sdk_id);

    let html_response = example_apps.to_html();
    HttpResponse::Ok().body(html_response)
}
pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
        .route("/sdk_search", web::get().to(search_sdk))
        .route("/matrix", web::get().to(get_matrix))
        .route("/example_apps", web::get().to(get_example_apps))
}
