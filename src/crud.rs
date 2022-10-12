use dotenv;
use sea_orm::{Database, ActiveModelTrait, ActiveValue, EntityTrait, QueryFilter, ColumnTrait};
use sea_orm::entity::prelude::DatabaseConnection;
use axum::Json;
use uuid::Uuid;
use serde::Deserialize;



use crate::entity::{player, jointable};
use crate::entity::bet;

// use super::entity::player::Entity as Entity_Player ; 
// use super::entity::bet::Entity as Bet;

async fn get_db() -> DatabaseConnection{
    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    println!("env: {}", database_url);
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    db
}

pub async fn write_new_player(player_name: String) -> player::Model {
    let db: DatabaseConnection = get_db().await;
    
    let rnd = rand::random::<i32>();
    let player_id = Uuid::new_v4();
    let active_player = player::ActiveModel {
        player_id: ActiveValue::set(player_id),
        name: ActiveValue::set(player_name),
        secret: ActiveValue::set(rnd),
    };

    let new_player: player::Model = active_player.insert(&db).await.unwrap();
    println!("fsd: {:?}", new_player);
    new_player

}

pub async fn write_new_bet(bet: BetInput) -> bet::Model {
    let db: DatabaseConnection = get_db().await;
    let bet_id = Uuid::new_v4();
    let active_bet = bet::ActiveModel {
        bet_id: ActiveValue::set(bet_id),
        player_id: ActiveValue::set(bet.player_id),
        name: ActiveValue::set(bet.name.clone()),
        odds: ActiveValue::set(bet.odds.clone()),
        stake: ActiveValue::set(bet.stake.clone()),
        settled: ActiveValue::set(bet.settled.clone()),
        description: ActiveValue::set(bet.description.clone()),
    };

    let new_bet: bet::Model = active_bet.insert(&db).await.unwrap();
    println!("fsd: {:?}", new_bet);
    new_bet
}

pub async fn write_participant_to_bet(player_id: Uuid, bet_id: Uuid) -> jointable::Model {
    let db: DatabaseConnection = get_db().await;
    let conn_key = Uuid::new_v4();
    let active_participation = jointable::ActiveModel{
        player_id: ActiveValue::Set(player_id),
        bet_id: ActiveValue::Set(bet_id),
        conn_key: ActiveValue::Set(conn_key),
    };
    let new_joitable: jointable::Model = active_participation.insert(&db).await.unwrap();
    println!("added to jointable: {:?}", new_joitable);
    new_joitable
}

pub async fn find_all_owned_bets(db: &DatabaseConnection, player_id: Uuid) -> Vec<bet::Model> {
    let stuff = bet::Entity::find().filter(bet::Column::PlayerId.eq(player_id))
        .all(db).await.unwrap();
    stuff
}

pub async fn find_all_participats(bet_id: Uuid) -> Vec<jointable::Model> {
    let db: DatabaseConnection = get_db().await;
    let stuff = jointable::Entity::find().filter(jointable::Column::BetId.eq(bet_id))
        .all(&db).await.unwrap();
    stuff
}

#[derive(Deserialize)]
pub struct BetInput{
    name: String,
    odds: String,
    stake: String,
    player_id: Uuid,
    settled: String,
    description: String,
}

#[derive(Deserialize)]
pub struct ParticipationInput {
    pub player_id: Uuid,
    pub bet_id: Uuid,
}


#[cfg(test)]
mod tests {
    use sea_orm::{
        entity::prelude::*, entity::*, tests_cfg::*, 
        DatabaseBackend, MockDatabase, Transaction,
    };
    use super::*;
    use crate::entity::bet;

    #[async_std::test]
    async fn test_find_all_owned_bets() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![bet::Model {
                bet_id: uuid::uuid!("c4a3e328-7a80-4764-8376-88029a611633"),
                player_id: uuid::uuid!("0ae5bf71-c6b0-4229-8c1c-03e9e7d6c3b3"),
                name: "test 1".to_string(),
                odds: "0.5".to_string(),
                stake: "500".to_string(),
                settled: "false".to_string(),
                description: "such awesome bet".to_string(),
            }]
        ]).into_connection();
        
        assert_eq!(
            find_all_owned_bets(&db, uuid::uuid!("0ae5bf71-c6b0-4229-8c1c-03e9e7d6c3b3")).await,
            vec![bet::Model {
                bet_id: uuid::uuid!("c4a3e328-7a80-4764-8376-88029a611633"),
                player_id: uuid::uuid!("0ae5bf71-c6b0-4229-8c1c-03e9e7d6c3b3"),
                name: "test 1".to_string(),
                odds: "0.5".to_string(),
                stake: "500".to_string(),
                settled: "false".to_string(),
                description: "such awesome bet".to_string(),
            }]
        );

        Ok(())
    }
}