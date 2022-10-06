use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::registry::Data;
use std::net::SocketAddr;
use rand::Rng;

mod crud;
mod entity;

use crud::Player;
use crud::write_new_player;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // let db = Database::connect(opt)

    let app = Router::new()
        .route("/", get(root))
        .route("/create_new_player", post(create_new_player))
        .route("/create_bet", post(create_bet));

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
    Json(payload): Json<Player>,
)  { //-> impl IntoResponse
    // let rnd = rand::random::<u64>();
    // let user = Player {
    //     player_id: payload.player_id,
    //     name: payload.name,
    //     bets: payload.bets,
    //     secret: Some(rnd),
    // };

    let written = crud::write_new_player(Json(payload)).await;
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    // (StatusCode::CREATED, Json(written.player_id))
}

async fn create_bet(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<Bet>,
) -> impl IntoResponse {
    let user = Bet {
        bet_id: payload.bet_id,
        name: payload.name,
        username: payload.username,
        odds: payload.odds,
        amount: payload.amount,
        participants: payload.participants,
        owner: payload.owner,
        settled: payload.settled,
        description: payload.description,
    };
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}



// the output to our `create_user` handler
#[derive(Serialize, Deserialize)]
struct Bet {
    name: String,
    bet_id: u64,
    username: String,
    odds: f64,
    amount: f64,
    participants: Vec<String>,
    owner: u64,
    settled: bool,
    description: String,
}
