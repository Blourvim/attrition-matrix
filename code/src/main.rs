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
    let intermediate_db_url = std::env::var("INTERMEDIATE_DB_URL").unwrap();

    let mut opt = ConnectOptions::new(intermediate_db_url);
    let int_db = Database::connect(opt).await?;

    HttpServer::new(|| {
        App::new()
            .service(api_scope())
            .service(
                Files::new("/", "./frontend/src/public")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .app_data(web::Data::new(int_db))
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await?;
    Ok(())
}
