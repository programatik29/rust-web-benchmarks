use axum::{handler::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/", get(|| async { "Hello, world!" }));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
