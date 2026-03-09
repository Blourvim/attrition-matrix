use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use entity::{intermediate, sdk};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, Statement, Value,
};

use crate::data::selector::get_db;

#[derive(Debug, FromQueryResult)]
pub struct IntermediateAggragate {
    pub sdk_from_id: i64,
    pub sdk_to_id: i64,
    pub app_count: i64,
}

pub struct IntermediateSdk {
    pub id: i64,
    pub name: String,
}

pub struct IntermediateAggragates {
    pub sdk_usages: HashMap<(i64, i64), IntermediateAggragate>,
}

impl IntermediateAggragates {
    pub async fn new(sdk_ids: &Vec<i64>, db: &DatabaseConnection) -> IntermediateAggragates {
        let placeholders: Vec<String> = (1..=sdk_ids.len()).map(|i| format!("${}", i)).collect();
        let in_clause = placeholders.join(", ");

        let sql = format!(
            r#"SELECT
    CASE WHEN from_sdk IN ({in_clause}) THEN from_sdk ELSE 0 END AS sdk_from_id,
    CASE WHEN to_sdk IN ({in_clause}) THEN to_sdk ELSE 0 END AS sdk_to_id,
    COUNT(*) AS app_count
FROM "intermediate"
GROUP BY sdk_from_id, sdk_to_id"#,
        );

        let values: Vec<Value> = sdk_ids.iter().map(|f| Value::BigInt(Some(*f))).collect();
        let statement = Statement::from_sql_and_values(db.get_database_backend(), &sql, values);
        let int_response = IntermediateAggragate::find_by_statement(statement)
            .all(db)
            .await;
        println!("int response{:? }", int_response);
        std::io::stdout().flush().unwrap();
        if let Ok(val) = int_response {
            let mut intermediate_aggragates: IntermediateAggragates = IntermediateAggragates {
                sdk_usages: HashMap::new(),
            };

            val.iter().for_each(|f| {
                intermediate_aggragates.sdk_usages.insert(
                    (f.sdk_from_id, f.sdk_to_id),
                    IntermediateAggragate {
                        app_count: f.app_count,
                        sdk_from_id: f.sdk_from_id,
                        sdk_to_id: f.sdk_to_id,
                    },
                );
            });
            return intermediate_aggragates;
        } else {
            let intermediate_aggragates: IntermediateAggragates = IntermediateAggragates {
                sdk_usages: HashMap::new(),
            };
            println!("no results");
            std::io::stdout().flush().unwrap();
            return intermediate_aggragates;
        }
    }
}

impl IntermediateAggragates {
    pub async fn to_html(&self) -> String {
        let sdk_set: HashSet<i64> = self.sdk_usages.iter().map(|f| f.0.0).collect();

        let db = get_db(crate::data::selector::DbSelector::Successor).await;
        let mut sdks: Vec<entity::sdk::Model> = entity::sdk::Entity::find()
            .filter(entity::sdk::Column::Id.is_in(sdk_set))
            .all(&db)
            .await
            .unwrap();

        let mut html: String = Default::default();
        // ---------insert the none field-----------

        let none_sdk: entity::sdk::Model = sdk::Model {
            id: 0,
            // I think this should be named "other" instead of "none", but I am sticking to the spec
            name: Some("(none)".to_string()),
            slug: Some("none".to_string()),
            url: Some("none".to_string()),
            description: Some("not included in the query".to_string()),
        };
        sdks.push(none_sdk);
        // ---------TAGS-----------
        let tags_start = "<div id=\"sdk-tags\" class=\"tags\">";
        html.push_str(tags_start);

        let tags: String = sdks
            .iter()
            .map(|f| {
                format!(
                    "<span  class=\"tag\" >{}</span>",
                    f.name.as_ref().unwrap_or(&"none".to_string())
                )
            })
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
            let col = format!(
                "<td class=\"cell\">{}</td>",
                sdk.name.as_ref().unwrap_or(&"none".to_string())
            );
            row.push_str(&col);
            sdks.iter().for_each(|to| {
                let value = self.sdk_usages.get_key_value(&(sdk.id, to.id));
                if let Some(value) = value {
                    let col = format!(
                        "<td style=\" background-color:hsl(207, {}%, 50%)\" class=\"cell\">{}</td>",
                        50, value.1.app_count
                    );
                    row.push_str(&col);
                } else {
                    // oops weird handling here, this could be cleaner,
                    // This "value" may not exist when attrition between sdk's is 0
                    // this entry is only created when attrition exists
                    let col = format!(
                        "<td style=\" background-color:hsl(207, {}%, 50%)\" class=\"cell\">{}</td>",
                        0, 0,
                    );
                    row.push_str(&col);
                }
            });
            row.push_str(table_header_end);
            html.push_str(&row);
        });
        html.push_str(table_end);
        html
    }
}
