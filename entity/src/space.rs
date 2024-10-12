//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "space")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub protocol: String,
    pub title: String,
    pub creator: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, Serialize, Deserialize)]
pub enum Relation {
    #[sea_orm(has_many = "super::proposal::Entity")]
    Proposal,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Creator",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::proposal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Proposal.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
