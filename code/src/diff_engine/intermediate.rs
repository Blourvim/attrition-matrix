use std::collections::{HashMap, HashSet};

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
    pub fn new(data: &Vec<intermediate::Model>) -> Self {
        let mut intermidiate_aggragates = IntermidiateAggragates {
            sdk_usages: HashMap::new(),
        };

        for element in data {
            let entry = intermidiate_aggragates
                .sdk_usages
                .entry((element.from_sdk, element.to_sdk));

            match entry {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().app_count += 1;
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
        return intermidiate_aggragates;
    }

    pub fn to_html(&self) -> String {
        // hashset here as a simple way to remove duplicates without itertools

        // todo: actually these two sets are duplicates, I could rework this into a two pointer system
        let to_sdk_set: HashSet<i64> = self.sdk_usages.iter().map(|f| f.0.1).collect();

        let from_sdk_set: HashSet<i64> = self.sdk_usages.iter().map(|f| f.0.0).collect();
        let mut html: String = Default::default();
        let _ = &to_sdk_set.iter().for_each(|to| {
            let mut row: String = Default::default();
            row.push_str("<tr>");
            from_sdk_set.iter().for_each(|from| {
                let value = self
                    .sdk_usages
                    .get_key_value(&(from.clone(), to.clone()))
                    .unwrap();
                let col = format!("<td>{}</td>", value.1.app_count);
                row.push_str(&col);
            });
            html.push_str(&row);
            row.push_str("</tr>");
        });

        html
    }
}
