use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Proposal::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Proposal::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Proposal::Title).string().not_null())
                    .col(ColumnDef::new(Proposal::Description).text())
                    .col(ColumnDef::new(Proposal::SpaceId).integer().not_null())
                    .col(ColumnDef::new(Proposal::CreatorAddress).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-proposal-space")
                            .from(Proposal::Table, Proposal::SpaceId)
                            .to(Space::Table, Space::Id)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Proposal::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Proposal {
    Table,
    Id,
    Title,
    Description,
    SpaceId,
    CreatorAddress,
    startTime,
    lockPeriod
}

#[derive(Iden)]
enum Space {
    Table,
    Id,
}