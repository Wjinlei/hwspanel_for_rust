use clap::Parser;

use hwspanel_for_rust::api;
use hwspanel_for_rust::infrastructure::cli;
use hwspanel_for_rust::infrastructure::config;
use hwspanel_for_rust::APP_CONTENT;

#[tokio::main]
async fn main() {
    let params = cli::Cli::parse();

    if params.start {
        let router = api::routers::get_routers();
        let config = config::load_config().await;
        let server = format!("{}:{}", config.server().host(), config.server().port());
        APP_CONTENT.set(config);

        axum::Server::bind(&server.parse().unwrap())
            .serve(router.into_make_service())
            .await
            .unwrap();
    }
}
