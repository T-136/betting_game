use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // manager.create_index(sea_query::Index::create());
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::PlayerId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::Secret).integer().not_null())
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Bet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bet::BetId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bet::Name).string().not_null())
                    .col(ColumnDef::new(Bet::PlayerId).uuid().not_null())
                    .col(ColumnDef::new(Bet::Odds).string().not_null())
                    .col(ColumnDef::new(Bet::Stake).string().not_null())
                    .col(ColumnDef::new(Bet::Settled).string().not_null())
                    .col(ColumnDef::new(Bet::Description).string().not_null())
                    .to_owned(),
            ).await?;
        manager
            .create_table(
                Table::create()
                    .table(Jointable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Jointable::ConnKey)
                            .uuid()
                            .primary_key()
                            .not_null()
                    )
                    .col(ColumnDef::new(Jointable::PlayerId).uuid().not_null())
                    .col(ColumnDef::new(Jointable::BetId).uuid().not_null())
                    .to_owned(),
            ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
            .name("playerid_fkey")
            .from(Jointable::Table, Jointable::PlayerId)
            .to(Player::Table, Player::PlayerId)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned(),
        ).await?;
        manager.create_foreign_key(
            ForeignKey::create()
            .name("betid_fkey")
            .from(Jointable::Table, Jointable::BetId)
            .to(Bet::Table, Bet::BetId)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned(),
        ).await?;
        manager.create_foreign_key(
            ForeignKey::create()
            .name("playerid_fkey")
            .from(Bet::Table, Bet::PlayerId)
            .to(Player::Table, Player::PlayerId)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bet::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Jointable::Table).to_owned())
            .await
    }
}


#[derive(Iden)]
enum Player {
    Table,
    Name,
    Secret, // better typ
    PlayerId,
}

#[derive(Iden)]
enum Bet {
    Table,
    Name,
    BetId,
    Odds,
    Stake,
    PlayerId,
    Settled,
    Description,
}

#[derive(Iden)]
enum Jointable {
    Table,
    PlayerId,
    BetId,
    ConnKey,
}