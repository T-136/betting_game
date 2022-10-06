use dotenv;
use sea_orm::{Database, ActiveModelTrait, ActiveValue};
use sea_orm::entity::prelude::DatabaseConnection;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::entity::player;

// use super::entity::player::Entity as Entity_Player ; 
// use super::entity::bet::Entity as Bet;


pub async fn write_new_player(player: Json<Player>)  {
    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    println!("env: {}", database_url);
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    
    let rnd = rand::random::<i32>();
    let uuid = rand::random::<i32>();
    let active_player = player::ActiveModel {
        player_id: ActiveValue::set(player.player_id),
        name: ActiveValue::set(player.name.clone()),
        bets: ActiveValue::set(Some(uuid)),
        secret: ActiveValue::set(rnd),
    };

    let new_player: player::Model = active_player.insert(&db).await.unwrap();
    println!("fsd: {:?}", new_player);

}




// #[derive(Deserialize, Serialize)]
// struct Player {
//     name: String,
//     bets: Vec<i32>,
//     secret: Option<i32>, // better typ
//     player_id: i32,
// }


// the input to our `create_user` handler
#[derive(Deserialize, Serialize)]
pub struct Player {
    name: String,
    bets: Vec<i32>,
    secret: Option<i32>, // better typ
    player_id: i32,
}