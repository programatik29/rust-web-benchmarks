use axum::{handler::get, Router};
use hyper::server::conn::Http;
use std::sync::Arc;
use tokio::{net::TcpListener, runtime};

type AnyError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let listener = Arc::new(TcpListener::bind("127.0.0.1:3000").await?);

    for _ in 0..num_cpus::get() - 1 {
        let listener = listener.clone();

        std::thread::spawn(move || {
            runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(run_instance(listener))
                .unwrap();
        });
    }

    run_instance(listener).await?;

    Ok(())
}

async fn run_instance(listener: Arc<TcpListener>) -> Result<(), AnyError> {
    loop {
        let (stream, _addr) = listener.accept().await?;

        let app = Router::new().nest("/", get(handler));

        tokio::spawn(async move {
            let _ = Http::new().serve_connection(stream, app).await;
        });
    }
}

async fn handler() -> &'static str {
    "Hello, world!"
}
