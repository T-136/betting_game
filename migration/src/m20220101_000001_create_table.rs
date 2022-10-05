use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::Bets).json())
                    .col(ColumnDef::new(Player::Secret).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Bet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bet::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bet::Name).string().not_null())
                    .col(ColumnDef::new(Bet::Owner).string().not_null())
                    .col(ColumnDef::new(Bet::Odds).string().not_null())
                    .col(ColumnDef::new(Bet::Stake).string().not_null())
                    .col(ColumnDef::new(Bet::Participants).string().not_null())
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
                        ColumnDef::new(Participants::Id)
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
                        ColumnDef::new(Bets::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            ).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}


#[derive(Iden)]
enum Player {
    Table,
    Name,
    Bets,
    Secret, // better typ
    Id,
}

#[derive(Iden)]
enum Bet {
    Table,
    Name,
    Id,
    Odds,
    Stake,
    Participants,
    Owner,
    Settled,
    Description,
}

#[derive(Iden)]
enum Participants {
    Table,
    Id,
}


#[derive(Iden)]
enum Bets {
    Table,
    Id
}