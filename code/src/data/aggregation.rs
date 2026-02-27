use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement};

#[derive(Debug)]
pub struct SdkUsageCount {
    pub sdk_id: i64,
    pub sdk_name: String,
    pub app_count: i64,
}
#[derive(Debug)]
pub struct SdkUsages {
    pub sdk_usages: Vec<SdkUsageCount>,
}

impl SdkUsages {
    pub async fn new(db: &DatabaseConnection, db_backend_type: DatabaseBackend) -> SdkUsages {
        // I should use the sea-orm builder here instead of the raw sql TODO: refactor
        let statement = Statement::from_string(
            db_backend_type,
            "SELECT 
                COALESCE(s.id, 0) AS sdk_id,
                COALESCE(s.name, 'none') AS sdk_name,
                COUNT(DISTINCT a.id) AS app_count
            FROM app a
            LEFT JOIN app_sdk a_sdk 
                ON a_sdk.app_id = a.id 
                AND a_sdk.installed = 1
            LEFT JOIN sdk s 
                ON s.id = a_sdk.sdk_id
            GROUP BY s.id, s.name
            ORDER BY app_count DESC;",
        );
        let sdk_usage_count = db.query_all_raw(statement).await.unwrap();

        let sdk_usages: Vec<SdkUsageCount> = sdk_usage_count
            .into_iter()
            .map(|row| SdkUsageCount {
                sdk_id: row.try_get("sdk_id", "").unwrap(),
                sdk_name: row.try_get("sdk_name", "").unwrap(),
                app_count: row.try_get("app_count", "").unwrap(),
            })
            .collect();
        SdkUsages {
            sdk_usages: sdk_usages,
        }
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    // Import outer scope to access functions
    use super::*;

    #[tokio::test]
    async fn test_sdk_usages() {
        dotenv::dotenv().ok();
        let db_url = std::env::var("BASELINE_DB_URL").expect("db url not found");
        let db = Database::connect(db_url).await.unwrap();
        let sdk_usages = SdkUsages::new(&db, DatabaseBackend::Sqlite).await;
        assert_ne!(sdk_usages.sdk_usages.len(), 0);
        let paypal_sdk = sdk_usages
            .sdk_usages
            .iter()
            .find(|sdk| sdk.sdk_name == "PayPal");

        assert_eq!(paypal_sdk.is_some(), true);
        assert!(paypal_sdk.is_some());
        assert_eq!(paypal_sdk.unwrap().sdk_id, 33);
        assert_eq!(paypal_sdk.unwrap().app_count, 84);
    }
}
