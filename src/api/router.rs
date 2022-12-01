use axum::{routing::get, Router};

use super::{public, resources};

pub fn new_router() -> Router {
    Router::new()
        .route("/", get(public::hello_world))
        .nest("/api", Router::new().nest("/res", resources::new_router()))
}
