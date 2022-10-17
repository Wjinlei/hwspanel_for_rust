use axum::response::Json;
use serde_json::{json, Value};

pub async fn hello_world() -> Json<Value> {
    Json(json!({ "data": "Hello World!"}))
}
