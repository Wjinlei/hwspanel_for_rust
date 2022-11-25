use axum::{routing::get, Router};

use super::{public, resources};

pub fn get_routers() -> Router {
    Router::new().route("/", get(public::hello_world)).nest(
        "/api",
        Router::new()
            .route("/GetResources", get(resources::api_resources))
            .route("/GetDisk", get(resources::api_disks)),
    )
}
