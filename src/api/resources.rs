use axum::response::Json;
use serde_json::{json, Value};

use crate::appliaction;

pub async fn api_get_resources() -> Json<Value> {
    Json(json!({
        "code": 200,
        "data": {
            "disks": appliaction::resources::get_disks(),
            "networks": appliaction::resources::get_networks(),
            "cpus": appliaction::resources::get_cpus(),
            "system": appliaction::resources::get_system(),
            "memory": appliaction::resources::get_memory(),
        },
    }))
}

pub async fn api_get_disks() -> Json<Value> {
    Json(json!({ "code": 200, "data": appliaction::resources::get_disks() }))
}

pub async fn api_get_networks() -> Json<Value> {
    Json(json!({ "code": 200, "data": appliaction::resources::get_networks() }))
}

pub async fn api_get_cpus() -> Json<Value> {
    Json(json!({ "code": 200, "data": appliaction::resources::get_cpus() }))
}

pub async fn api_get_system() -> Json<Value> {
    Json(json!({ "code": 200, "data": appliaction::resources::get_system() }))
}

pub async fn api_get_memory() -> Json<Value> {
    Json(json!({ "code": 200, "data": appliaction::resources::get_memory() }))
}
