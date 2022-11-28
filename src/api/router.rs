use axum::{routing::get, Router};

use super::{public, resources};

pub fn get_router() -> Router {
    Router::new().route("/", get(public::hello_world)).nest(
        "/api",
        Router::new()
            .route("/GetResources", get(resources::api_get_resources))
            .route("/GetDisks", get(resources::api_get_disks))
            .route("/GetNetworks", get(resources::api_get_networks)),
    )
}
