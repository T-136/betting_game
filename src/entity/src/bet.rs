//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bet")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bet_id: Uuid,
    pub name: String,
    pub owner: String,
    pub odds: String,
    pub stake: String,
    pub participants: String,
    pub settled: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bets::Entity")]
    Bets,
    #[sea_orm(has_many = "super::player::Entity")]
    Player,
}

impl Related<super::bets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bets.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
