use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "intermediate")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub from_sdk: i64,
    pub to_sdk: i64,
}
//todo: to support the example app per sdk functionality, I should include the app_id here or perhaps on another table, tbd
impl ActiveModelBehavior for ActiveModel {}
