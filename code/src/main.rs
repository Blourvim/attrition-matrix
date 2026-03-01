use actix_files::Files;
use actix_web::{App, HttpServer, web};
use dotenv;
use product_eng_interview::api::api::api_scope;
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // the main objective here is to compare one dataset to the next,
    // TODO: implement a database selector of the sorts here to better manage datasets from various conections
    let baseline_db_url = std::env::var("BASELINE_DB_URL").unwrap();
    let successor_db_url = std::env::var("SUCCESSOR_DB_URL").unwrap();

    // in the first db, get the first app
    // in the second db, get the same app
    // compare both on which sdk's they use
    // if the successor has sdk's which are 0 turned to 1, that sdk gets +1 points
    // it is also possible that an app has multiple sdk's installed, and may have more installed without actually losing any market share

    HttpServer::new(|| {
        App::new().service(api_scope()).service(
            Files::new("/", "./frontend/src/public")
                .prefer_utf8(true)
                .index_file("index.html"),
        )
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await?;
    Ok(())
}
