use sea_orm::{DatabaseBackend, DatabaseConnection};

#[derive(Debug)]
pub struct SdkUsageCount {
    pub sdk_to_id: i64,
    pub sdk_from_id: String,
    pub count: i64,
}
#[derive(Debug)]
pub struct SdkUsages {
    pub sdk_usages: Vec<SdkUsageCount>,
}

impl SdkUsages {
    pub async fn new(db: &DatabaseConnection, db_backend_type: DatabaseBackend) -> SdkUsages {}
}
