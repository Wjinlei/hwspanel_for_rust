use axum::{routing::get, Router};

use hwspanel_for_rust::api;

pub fn routers() -> Router {
    Router::new().route("/", get(api::public::hello_world))
}
