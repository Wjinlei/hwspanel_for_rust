use axum::{routing::get, Router};

use super::public;

pub fn routers() -> Router {
    Router::new().route("/", get(public::hello_world))
}
