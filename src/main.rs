use axum::{routing::get, Router};

pub mod api;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(api::public::hello_world));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:6588".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
