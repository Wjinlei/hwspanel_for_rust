use axum::{routing::get, Router};

use super::{public, resources};

pub fn get_router() -> Router {
    Router::new().route("/", get(public::hello_world)).nest(
        "/api",
        Router::new()
            .route("/hello", get(public::hello_world))
            .nest("/res", get_res_router()),
    )
}

fn get_res_router() -> Router {
    Router::new()
        .route("/GetResources", get(resources::api_get_resources))
        .route("/GetDisks", get(resources::api_get_disks))
        .route("/GetNetworks", get(resources::api_get_networks))
        .route("/GetCpu", get(resources::api_get_cpu))
        .route("/GetSystem", get(resources::api_get_system))
        .route("/GetMemory", get(resources::api_get_memory))
}
