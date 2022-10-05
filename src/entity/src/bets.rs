//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub bets_id: Uuid,
    pub bet_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bet::Entity",
        from = "Column::BetId",
        to = "super::bet::Column::BetId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Bet,
}

impl Related<super::bet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}