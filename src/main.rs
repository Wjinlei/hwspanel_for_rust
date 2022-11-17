use clap::Parser;
use state::Container;

use hwspanel_for_rust::api;
use hwspanel_for_rust::infrastructure::cli;
use hwspanel_for_rust::infrastructure::config;

pub static APP_CONTENT: Container![Send + Sync] = <Container![Send + Sync]>::new();

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    if cli.start {
        start_webservice().await;
    }
}

async fn start_webservice() {
    // Build routers
    let app = api::routers::routers();

    // Load config
    let config = config::load_config().await;

    // Save config
    APP_CONTENT.set(config.clone());

    let address = format!("{}:{}", config.server().host(), config.server().port());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
