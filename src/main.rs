use axum::{
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use sea_orm::error;
use std::net::SocketAddr;
use serde_json::{self, to_string};
use uuid::Uuid;

mod crud;
mod entity;

use crate::entity::player;
use crate::entity::bet;
use crud::{write_new_player, write_new_bet, find_all_owned_bets, write_participant_to_bet, find_all_participats, BetInput, ParticipationInput};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // let db = Database::connect(opt)

    let app = Router::new()
        .route("/", get(root))
        .route("/create_new_player", post(create_new_player))
        .route("/create_bet", post(create_bet))
        .route("/get_owned_bets", post(get_owned_bets))
        .route("/create_participation", post(create_participation))
        .route("/get_all_participants", post(get_all_participants));


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_new_player(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    payload: String 
) -> impl IntoResponse { 
    println!("{:?}", payload);
    let written = write_new_player(payload).await;
    let res = serde_json::to_string(&written).unwrap() ;
    
    (StatusCode::CREATED, res)
}



async fn create_bet(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<BetInput>,
) -> impl IntoResponse {
    let written = write_new_bet(payload).await;
    let res = serde_json::to_string(&written).unwrap();
    (StatusCode::CREATED, res)
}

async fn create_participation(
    Json(payload): Json<ParticipationInput>,
) -> impl IntoResponse {
    let input = write_participant_to_bet(payload.player_id, payload.bet_id).await;
    (StatusCode::OK, serde_json::to_string(&input).unwrap())
}

async fn get_owned_bets(  
    player_id: String
) -> impl IntoResponse {  
    println!("{:?}", player_id);
    if let Ok(player_uuid) = Uuid::parse_str(&player_id) {
        let bets = find_all_owned_bets(player_uuid).await;
        (StatusCode::OK, serde_json::to_string(&bets).unwrap())
    } else {
        (StatusCode::BAD_REQUEST, "player id not of type Uuid".to_string())
    }
}

async fn get_all_participants(  
    bet_id: String
) -> impl IntoResponse {  
    println!("{:?}", bet_id);
    if let Ok(bet_uuid) = Uuid::parse_str(&bet_id) {
        let bets = find_all_participats(bet_uuid).await;
        (StatusCode::OK, serde_json::to_string(&bets).unwrap())
    } else {
        (StatusCode::BAD_REQUEST, "player id not of type Uuid".to_string())
    }
}

// {
//     "Name": "crazy bet",
//     "player_id": "c7482762-59de-40db-98b5-53613a8e8c1d",
//     "odds": "5",
//     "stake": "100",
//     "settled": "false",
//     "description": "very good bet"
// }