use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct FlightRecord(Vec<(String, String)>);

#[derive(Debug, Serialize, Deserialize)]
struct FlightPath(Vec<String>);

async fn calculate_flight_path(
    State(_state): State<Arc<Mutex<AppState>>>,
    Json(flights): Json<FlightRecord>,
) -> Result<Json<FlightPath>, StatusCode> {
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
    let state = Arc::new(Mutex::new(AppState {}));
    let app = Router::new()
        .route("/calculate", post(calculate_flight_path))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}