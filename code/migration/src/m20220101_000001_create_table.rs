use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // Defining the table schema
        manager
            .create_table(
                Table::create()
                    .table("intermediate")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("from_sdk"))
                    .col(integer("to_sdk"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let _ = manager
            .drop_table(Table::drop().table("intermediate").to_owned())
            .await;
        Ok(())
    }
}
