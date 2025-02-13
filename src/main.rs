use axum::{
    extract::{State, TypedHeader},
    routing::post,
    Json, Router,
    response::{IntoResponse},
};
use headers::Authorization;
use hyper::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const SECRET: &[u8] = b"e9c8f8e7d6a5b4c3a2f1e0d9c8b7a6f5";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct FlightRecord(Vec<(String, String)>);

#[derive(Debug, Serialize, Deserialize)]
struct FlightPath(Vec<String>);

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

async fn calculate_flight_path(
    State(_state): State<Arc<Mutex<AppState>>>,
    TypedHeader(auth_header): TypedHeader<Authorization<headers::authorization::Bearer>>,
    Json(flights): Json<FlightRecord>,
) -> Result<Json<FlightPath>, (StatusCode, Json<ErrorResponse>)> {

    info!("Received token: {}", auth_header.token());

    if auth_header.token() != "e9c8f8e7d6a5b4c3a2f1e0d9c8b7a6f5" {
        let error = ErrorResponse {
            message: "Unauthorized: Invalid token".to_string(),
        };
        return Err((StatusCode::UNAUTHORIZED, Json(error)));
    }

    let path = sort_flight_path(flights.0);
    Ok(Json(FlightPath(path)))
}

fn sort_flight_path(flights: Vec<(String, String)>) -> Vec<String> {
    let mut graph: HashMap<String, String> = HashMap::new();
    let mut reverse_graph: HashMap<String, String> = HashMap::new();

    for (src, dst) in &flights {
        graph.insert(src.clone(), dst.clone());
        reverse_graph.insert(dst.clone(), src.clone());
    }

    let start = flights
        .iter()
        .map(|(src, _)| src)
        .find(|src| !reverse_graph.contains_key(*src))
        .unwrap()
        .clone();

    let mut path = vec![start.clone()];
    let mut current = start;
    while let Some(next) = graph.get(&current) {
        path.push(next.clone());
        current = next.clone();
    }

    path
}

struct AppState {}

#[tokio::main]
async fn main() {
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let state = Arc::new(Mutex::new(AppState {}));
    let app = Router::new()
        .route("/calculate", post(calculate_flight_path))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}