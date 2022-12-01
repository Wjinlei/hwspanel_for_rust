use std::sync::RwLock;
use sysinfo::System;
use sysinfo::SystemExt;

use hwspanel_for_rust::api;
use hwspanel_for_rust::appliaction::SYS_INFO;
use hwspanel_for_rust::infrastructure::config;
use hwspanel_for_rust::infrastructure::APP_CONTENT;

#[tokio::main]
async fn main() {
    let router = api::router::get_router();
    let config = config::load_config().await;
    let server = format!("{}:{}", config.server().host(), config.server().port());
    APP_CONTENT.set(config);
    SYS_INFO.set(RwLock::new(System::new_all()));

    axum::Server::bind(&server.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
