pub mod cli;
pub mod routers;

use clap::Parser;
use cli::cli::Cli;
use state::Container;

pub static APP_CONTENT: Container![Send + Sync] = <Container![Send + Sync]>::new();

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.start {
        start_webservice().await;
    }
}

async fn start_webservice() {
    // Build routers
    let app = routers::api::routers();

    // Load config
    let config = hwspanel_for_rust::load_config().await;

    // Save config
    APP_CONTENT.set(config.clone());

    let address = format!("{}:{}", config.server().host(), config.server().port());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
