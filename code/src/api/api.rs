use actix_web::{HttpResponse, Responder, web};

use crate::api::dto::{
    example_apps,
    matrix::{AttritionMatrixQuery, AttritionMatrixResponse},
    sdk_search::{self, SdkSearchQuery, SdkSearchResponse},
};

async fn get_matrix(attrition_matrix_query: web::Query<AttritionMatrixQuery>) -> impl Responder {
    let query = attrition_matrix_query.into_inner();
    // now per requested sdk, we can calculate and fetch
    let response = AttritionMatrixResponse {};
    HttpResponse::Ok().json(response)
}

async fn search_sdk(search_query: web::Query<SdkSearchQuery>) -> impl Responder {
    let mock_sdks = (0..5)
        .into_iter()
        .map(|i| sdk_search::Sdk {
            name: search_query.search.clone(),
            id: i,
            logo_url: "https://picsum.photos/200".to_string(),
        })
        .collect::<Vec<sdk_search::Sdk>>();

    let html_response = mock_sdks
        .iter()
        .map(|sdk| format!("<option value=\"{}\">{}</option>", sdk.id, sdk.name))
        .collect::<String>();

    HttpResponse::Ok().body(html_response)
}

async fn get_example_apps(
    search_query: web::Query<example_apps::ExampleAppsQuery>,
) -> impl Responder {
    let example_apps: example_apps::ExampleApps = example_apps::ExampleApps::new(5);

    let html_response = example_apps.to_html();
    HttpResponse::Ok().body(html_response)
}
pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
        .route("/sdk_search", web::get().to(search_sdk))
        .route("/matrix", web::get().to(get_matrix))
        .route("/example_apps", web::get().to(get_example_apps))
}
