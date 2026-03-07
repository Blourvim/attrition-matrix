use std::collections::{HashMap, HashSet};

use entity::intermediate;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::data::selector::get_db;

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

    // todo: this is for the "none" calculation implement this later since calculations are  slightly different
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

    pub async fn to_html(&self) -> String {
        let sdk_set: HashSet<i64> = self.sdk_usages.iter().map(|f| f.0.0).collect();

        let db = get_db(crate::data::selector::DbSelector::Successor).await;
        let sdks = entity::sdk::Entity::find()
            .filter(entity::sdk::Column::Id.is_in(sdk_set))
            .all(&db)
            .await
            .unwrap();
        let mut html: String = Default::default();

        // ---------TAGS-----------
        let tags_start = "<div id=\"sdk-tags\" class=\"tags\">";
        html.push_str(tags_start);

        let tags: String = sdks
            .iter()
            .map(|f| format!("<span  class=\"tag\" >{}</span>", f.name.as_ref().unwrap()))
            .collect();
        html.push_str(&tags);

        let tags_end = "</div>";
        html.push_str(tags_end);

        // ---------Table-----------

        let table_start = "<table id=\"matrix\" border=\"1\" cellpadding=\"0\" cellspacing=\"0\" style=\"border-collapse: collapse\" width=\"60%\">";
        let table_header_start = "<tr class=\"row\">";
        let table_header_end = "</tr>";
        let table_end = "</table>";

        html.push_str(table_start);
        html.push_str(table_header_start);

        // do an empty one
        let empty_col = "<td class=\"cell\"></td>";
        html.push_str(empty_col);

        sdks.iter().for_each(|f| {
            let col = format!("<td class=\"cell\">{}</td>", f.name.as_ref().unwrap());
            html.push_str(&col);
        });

        html.push_str(table_header_end);

        let _ = &sdks.iter().enumerate().for_each(|(index, sdk)| {
            let mut row: String = Default::default();

            row.push_str("<tr class=\"row\">");
            let col = format!("<td class=\"cell\">{}</td>", sdk.name.as_ref().unwrap());
            row.push_str(&col);
            sdks.iter().for_each(|to| {
                let value = self.sdk_usages.get_key_value(&(sdk.id, to.id));
                if let Some(value) = value {
                    let col = format!("<td class=\"cell\">{}</td>", value.1.app_count);
                    row.push_str(&col);
                } else {
                    // oops weird handling here, this could be cleaner,
                    // This "value" may not exist when attrition between sdk's is 0
                    // this entry is only created when attrition exists
                    let col = format!("<td class=\"cell\">{}</td>", 0);
                    row.push_str(&col);
                }
            });
            row.push_str("</tr>");
            html.push_str(&row);
        });
        let table_end = "</table>";
        html.push_str(table_end);
        html
    }
}
