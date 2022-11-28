use axum::{routing::get, Router};

use super::{public, resources};

pub fn get_router() -> Router {
    Router::new().route("/", get(public::hello_world)).nest(
        "/api",
        Router::new()
            .route("/GetResources", get(resources::api_get_resources))
            .route("/GetNetworks", get(resources::api_get_networks))
            .route("/GetDisks", get(resources::api_get_disks))
            .route("/GetCpus", get(resources::api_get_cpus)),
    )
}
