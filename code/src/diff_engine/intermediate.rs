use std::collections::HashMap;

use entity::intermediate;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct IntermidiateAggragates {
    pub sdk_usages: HashMap<(i64, i64), SdkUsageCount>,
}

#[derive(Debug)]
pub struct SdkUsageCount {
    pub sdk_from_id: i64,
    pub sdk_to_id: i64,
    pub app_count: i64,
}

pub async fn fetch_intermidiate_layer(
    skds: Vec<i64>,
    db: &DatabaseConnection,
) -> Result<Vec<intermediate::Model>, Box<dyn std::error::Error>> {
    let response: Vec<intermediate::Model> = intermediate::Entity::find()
        .filter(intermediate::Column::FromSdk.is_in(skds.clone()))
        .filter(intermediate::Column::ToSdk.is_in(skds.clone()))
        .order_by_id(sea_orm::Order::Desc)
        .all(db)
        .await?;

    // todo: this is for the "none" calculation implement this later since calculations are done slightly,
    let none_response: Vec<intermediate::Model> = intermediate::Entity::find()
        .filter(intermediate::Column::FromSdk.is_in(skds.clone()))
        .filter(intermediate::Column::ToSdk.is_not_in(skds.clone()))
        .all(db)
        .await?;

    Ok(response)
}

impl IntermidiateAggragates {
    pub fn new(mut self, data: Vec<intermediate::Model>) -> Self {
        for element in data {
            let entry = self.sdk_usages.entry((element.from_sdk, element.to_sdk));

            match entry {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.into_mut().app_count += 1;
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(SdkUsageCount {
                        sdk_from_id: element.from_sdk,
                        sdk_to_id: element.to_sdk,
                        app_count: 1,
                    });
                }
            }
        }
        return self;
    }
}
