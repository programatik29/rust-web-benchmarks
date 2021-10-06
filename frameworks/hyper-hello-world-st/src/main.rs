use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::{convert::Infallible, sync::Arc};
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

        tokio::spawn(async move {
            let _ = Http::new().serve_connection(stream, service_fn(serve)).await;
        });
    }
}

async fn serve(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let resp = Response::new(Body::from("Hello World!"));

    Ok(resp)
}
