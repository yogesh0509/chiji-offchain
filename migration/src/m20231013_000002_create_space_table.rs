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
                    .col(ColumnDef::new(Space::Protocol).string().not_null())
                    .col(ColumnDef::new(Space::Title).string().not_null())
                    .col(ColumnDef::new(Space::Creator).integer().not_null())
                    .col(ColumnDef::new(Space::Description).text())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-space-creator")
                            .from(Space::Table, Space::Creator)
                            .to(User::Table, User::Id)
                    )
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
    Protocol,
    Title,
    Creator,
    Description,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}