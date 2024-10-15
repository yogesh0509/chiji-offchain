use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the VoteType enum
        let sql = r#"
            CREATE TYPE vote_type AS ENUM ('For', 'Against', 'Abstain');
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await?;

        manager
            .create_table(
                Table::create()
                    .table(Vote::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Vote::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Vote::UserAddress).string().not_null())
                    .col(ColumnDef::new(Vote::ProposalId).integer().not_null())
                    .col(ColumnDef::new(Vote::VoteType).custom(Alias::new("vote_type")).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-vote-proposal")
                            .from(Vote::Table, Vote::ProposalId)
                            .to(Proposal::Table, Proposal::Id)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vote::Table).to_owned())
            .await?;

        // Drop the VoteType enum
        let sql = "DROP TYPE vote_type;";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Vote {
    Table,
    Id,
    UserAddress,
    ProposalId,
    VoteType,
}

#[derive(Iden)]
enum Proposal {
    Table,
    Id,
}