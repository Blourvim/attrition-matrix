use std::collections::HashMap;

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use dotenv;
use migration::MigratorTrait;
use product_eng_interview::{
    api::api::api_scope,
    data::selector::{DbSelector, get_db},
    diff_engine::engine::CurnAndRetentionMap,
};
use sea_orm::{ColumnTrait, ConnectOptions, Database, EntityTrait, PaginatorTrait, QueryFilter};
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // the main objective here is to compare one dataset to the next,
    // TODO: implement a database selector of the sorts here to better manage datasets from various conections
    let inter_db = get_db(DbSelector::Intermediate).await;
    let baseline_db = get_db(DbSelector::Baseline).await;
    let successor_db = get_db(DbSelector::Successor).await;

    //populate inter_db
    migration::Migrator::up(&inter_db, None)
        .await
        .expect("failed to migrate");

    let mut paginated_apps = entity::app::Entity::find().paginate(&baseline_db, 1);

    // todo: this is bad due to the many roundtrips to db, but it is memory efficent
    // it might be better to find a middle ground or work around the hardware limitations,
    // for the initial version I would like to just make it work, then optimize
    while let Some(apps) = paginated_apps.fetch_and_next().await? {
        // in here we call a smaller subset of the database a single app in this instance
        // todo this should be executed in parallel
        let ids: Vec<i64> = apps.iter().map(|f| f.id).collect();
        let baseline_app_sdks = entity::app_sdk::Entity::find()
            .filter(entity::app_sdk::Column::AppId.is_in(ids.clone()))
            .all(&baseline_db)
            .await?;
        let successor_app_sdks = entity::app_sdk::Entity::find()
            .filter(entity::app_sdk::Column::AppId.is_in(ids.clone()))
            .all(&successor_db)
            .await?;

        let baseline_hasmap_app_sdks: HashMap<i64, &entity::app_sdk::Model> =
            baseline_app_sdks.iter().map(|f| (f.sdk_id, f)).collect();
        let successor_hasmap_app_sdks: HashMap<i64, &entity::app_sdk::Model> =
            successor_app_sdks.iter().map(|f| (f.sdk_id, f)).collect();

        let baseline_hasmap_app_sdks_response =
            CurnAndRetentionMap::calculate_churns_and_retentions(
                baseline_hasmap_app_sdks,
                successor_hasmap_app_sdks,
                &inter_db,
            )
            .await;
    }

    HttpServer::new(move || {
        App::new()
            .service(api_scope())
            .service(
                Files::new("/", "./frontend/src/public")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .app_data(web::Data::new(inter_db.clone()))
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await?;
    Ok(())
}
