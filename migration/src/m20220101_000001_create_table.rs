use sea_orm_migration::prelude::*;



#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_index(sea_query::Index::create());
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::PlayerId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::Bets).integer())
                    .col(ColumnDef::new(Player::Secret).integer().not_null())
                    .to_owned(),
            )
            .await?;

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
                    .col(ColumnDef::new(Bet::Owner).string().not_null())
                    .col(ColumnDef::new(Bet::Odds).string().not_null())
                    .col(ColumnDef::new(Bet::Stake).string().not_null())
                    .col(ColumnDef::new(Bet::ParticipantsTableId).uuid().not_null())
                    .col(ColumnDef::new(Bet::Settled).string().not_null())
                    .col(ColumnDef::new(Bet::Description).string().not_null())
                    .to_owned(),
            ).await?;
        manager
            .create_table(
                Table::create()
                    .table(Participants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Participants::ParticipantsId)
                            .uuid()
                            .not_null()

                    )
                    .col(
                        ColumnDef::new(Participants::ParticipantsTableId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            ).await?;
        manager
            .create_table(
                Table::create()
                    .table(Bets::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(Bets::BetsId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Bets::BetId)
                            .uuid()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await?;
        manager.create_foreign_key(
            ForeignKey::create()
            .name("testname")
            .from(Player::Table, Player::Bets)
            .to(Bets::Table, Bets::BetsId)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned(),
        ).await?;
        manager.create_foreign_key(
            ForeignKey::create()
            .name("testname")
            .from(Bets::Table, Bets::BetId)
            .to(Bet::Table, Bet::BetId)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned(),
        ).await?;
        manager.create_foreign_key(
            ForeignKey::create()
            .name("testname")
            .from(Bet::Table, Bet::ParticipantsTableId)
            .to(Participants::Table, Participants::ParticipantsTableId)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned(),
        ).await?;
        manager.create_foreign_key(
            ForeignKey::create()
            .name("testname")
            .from(Participants::Table, Participants::ParticipantsId)
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
            .drop_table(Table::drop().table(Bets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Participants::Table).to_owned())
            .await
    }
}


#[derive(Iden)]
enum Player {
    Table,
    Name,
    Bets,
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
    ParticipantsTableId,
    Owner,
    Settled,
    Description,
}

#[derive(Iden)]
enum Participants {
    Table,
    ParticipantsTableId,
    ParticipantsId,
}

#[derive(Iden)]
enum Bets {
    Table,
    BetsId,
    BetId,
}