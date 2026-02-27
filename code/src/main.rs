use dotenv;
use tokio;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // the main objective here is to compare one dataset to the next,
    let baseline_db_url = std::env::var("BASELINE_DB_URL").unwrap();
    Ok(())
}
