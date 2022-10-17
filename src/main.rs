use axum::{routing::get, Router};
use config::config::AppConfig;
use state::Container;
use tokio::fs::read_to_string;

pub mod api;
pub mod config;

#[macro_use]
extern crate getset;

pub static APP_CONTENT: Container![Send + Sync] = <Container![Send + Sync]>::new();

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(api::public::hello_world));

    // Load config
    let config_data = read_to_string("application.yaml").await.unwrap();
    let config = AppConfig::new(config_data.as_str());

    // Save config
    APP_CONTENT.set(config.clone());

    // run it with hyper on localhost:3000
    let address = format!("{}:{}", config.server().host(), config.server().port());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
