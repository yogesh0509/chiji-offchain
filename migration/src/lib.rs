pub use sea_orm_migration::prelude::*;

mod m20231013_000001_create_user_table;
mod m20231013_000002_create_space_table;
mod m20231013_000003_create_proposal_table;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20231013_000001_create_user_table::Migration),
            Box::new(m20231013_000002_create_space_table::Migration),
            Box::new(m20231013_000003_create_proposal_table::Migration),
        ]
    }
}
