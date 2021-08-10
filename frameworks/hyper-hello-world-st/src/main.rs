use std::convert::Infallible;
use std::sync::Arc;

use tokio::runtime;
use tokio::net::TcpListener;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};

type AnyError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let listener = Arc::new(TcpListener::bind("127.0.0.1:3000").await?);

    for _ in 0..num_cpus::get()-1 {
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

        tokio::spawn(async move {
            let fut = Http::new().serve_connection(stream, service_fn(serve));

            match fut.await {
                Ok(()) => (),
                Err(_) => (),
            }
        });
    }
}

async fn serve(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let resp = Response::new(Body::from("Hello World!"));

    Ok(resp)
}
