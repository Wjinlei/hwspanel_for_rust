use axum::response::Json;
use serde_json::{json, Value};

use crate::appliaction;

pub async fn api_resources() -> Json<Value> {
    Json(json!({
        "code": 200,
        "data": { "disks": appliaction::resources::get_disks() },
    }))
}

pub async fn api_disks() -> Json<Value> {
    Json(json!({
        "code": 200,
        "data": appliaction::resources::get_disks(),
    }))
}
