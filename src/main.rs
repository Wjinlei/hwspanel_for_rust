use sysinfo::System;
use sysinfo::SystemExt;

use tracing::info;

use hwspanel_for_rust::api;
use hwspanel_for_rust::appliaction::SYS_INFO;
use hwspanel_for_rust::infrastructure::config;
use hwspanel_for_rust::infrastructure::APP_CONTENT;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = api::router::new_router();
    let config = config::load_config().await;
    let server = format!("{}:{}", config.server().host(), config.server().port());

    // State management.
    APP_CONTENT.set(config);
    SYS_INFO.set(std::sync::RwLock::new(System::new_all()));

    info!("Listening on {}", server);

    axum::Server::bind(&server.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
