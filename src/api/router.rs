use axum::{routing::get, Router};

use super::{public, resources};

pub fn new_router() -> Router {
    Router::new()
        .route("/", get(public::hello_world))
        .nest("/api", Router::new().nest("/res", resources_router()))
}

fn resources_router() -> Router {
    Router::new()
        .route("/GetDisks", get(resources::api_get_disks))
        .route("/GetNetworks", get(resources::api_get_networks))
        .route("/GetCpu", get(resources::api_get_cpu))
        .route("/GetSystem", get(resources::api_get_system))
        .route("/GetMemory", get(resources::api_get_memory))
        .route("/GetResources", get(resources::api_get_resources))
}
