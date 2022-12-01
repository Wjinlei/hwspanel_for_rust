use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

use crate::appliaction;

pub async fn get_resources() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "disks": appliaction::resources::get_disks(),
            "networks": appliaction::resources::get_networks(),
            "cpu": appliaction::resources::get_cpu(),
            "memory": appliaction::resources::get_memory(),
            "system": appliaction::resources::get_system(),
        })),
    )
}

pub async fn get_disks() -> impl IntoResponse {
    (StatusCode::OK, Json(appliaction::resources::get_disks()))
}

pub async fn get_networks() -> impl IntoResponse {
    (StatusCode::OK, Json(appliaction::resources::get_networks()))
}

pub async fn get_cpu() -> impl IntoResponse {
    (StatusCode::OK, Json(appliaction::resources::get_cpu()))
}

pub async fn get_memory() -> impl IntoResponse {
    (StatusCode::OK, Json(appliaction::resources::get_memory()))
}

pub async fn get_system() -> impl IntoResponse {
    (StatusCode::OK, Json(appliaction::resources::get_system()))
}

pub fn new_router() -> Router {
    Router::new()
        .route("/GetDisks", get(get_disks))
        .route("/GetNetworks", get(get_networks))
        .route("/GetCpu", get(get_cpu))
        .route("/GetSystem", get(get_system))
        .route("/GetMemory", get(get_memory))
        .route("/GetResources", get(get_resources))
}
