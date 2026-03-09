use std::io::Write;

use actix_web::{HttpResponse, Responder, web};
use sea_orm::DatabaseConnection;

use crate::{
    api::dto::{
        example_apps,
        matrix::AttritionMatrixQuery,
        sdk_search::{SdkSearchQuery, SdkSearchResponse},
    },
    data::selector::{DbSelector, get_db},
    diff_engine::intermediate::IntermediateAggragates,
};

async fn get_matrix(
    attrition_matrix_query: AttritionMatrixQuery,
    conn: web::Data<DatabaseConnection>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let query = attrition_matrix_query;
    print!("{:?}", query.sdks);
    std::io::stdout().flush().unwrap();

    let intermediate_aggragates = IntermediateAggragates::new(&query.sdks, &conn);

    let html = intermediate_aggragates.await.to_html().await;
    return Ok(HttpResponse::Ok().body(html));
}

async fn search_sdk(search_query: web::Query<SdkSearchQuery>) -> impl Responder {
    let conn = get_db(DbSelector::Baseline).await;
    let search = search_query.into_inner().search;
    let html_response = SdkSearchResponse::new(search, &conn).await.to_html();
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
