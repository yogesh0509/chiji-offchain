use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Space::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Space::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Space::Name).string().not_null())
                    .col(ColumnDef::new(Space::About).text())
                    .col(ColumnDef::new(Space::Avatar).string())
                    .col(ColumnDef::new(Space::Symbol).string())
                    .col(ColumnDef::new(Space::TokenAddress).string())
                    .col(ColumnDef::new(Space::GovernanceContractAddress).string())
                    .col(ColumnDef::new(Space::Twitter).string())
                    .col(ColumnDef::new(Space::Discord).string())
                    .col(ColumnDef::new(Space::Terms).text())
                    .col(ColumnDef::new(Space::Admins).json())
                    .col(ColumnDef::new(Space::Authors).json())
                    .col(ColumnDef::new(Space::Users).json())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Space::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Space {
    Table,
    Id,
    Name,
    About,
    Avatar,
    Symbol,
    TokenAddress,
    GovernanceContractAddress,
    Twitter,
    Discord,
    Terms,
    Admins,
    Authors,
    Users
}