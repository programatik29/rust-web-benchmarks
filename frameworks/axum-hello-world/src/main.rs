use tokio::net::TcpListener;

use hyper::server::conn::Http;
use hyper::{Body, Response};

use axum::prelude::*;
use axum::routing::nest;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    let app = nest(
        "/",
        get(|| async { Response::new(Body::from("Hello, World!")) }),
    );

    loop {
        let (stream, _addr) = listener.accept().await?;

        let app = app.clone();

        tokio::spawn(async move {
            let fut = Http::new().serve_connection(stream, app);

            match fut.await {
                Ok(()) => (),
                Err(_) => (),
            }
        });
    }
}
