use entity::intermidiate;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn fetch_intermidiate_layer(
    skds: Vec<i64>,
    db: &DatabaseConnection,
) -> Result<Vec<intermidiate::Model>, Box<dyn std::error::Error>> {
    let response: Vec<intermidiate::Model> = intermidiate::Entity::find()
        .filter(intermidiate::Column::FromSdk.is_in(skds.clone()))
        .filter(intermidiate::Column::ToSdk.is_in(skds.clone()))
        .all(db)
        .await?;

    // todo: this is for the "none" calculation implement this later since calculations are done slightly,
    let none_response: Vec<intermidiate::Model> = intermidiate::Entity::find()
        .filter(intermidiate::Column::FromSdk.is_in(skds.clone()))
        .filter(intermidiate::Column::ToSdk.is_not_in(skds.clone()))
        .all(db)
        .await?;

    Ok(response)
}
