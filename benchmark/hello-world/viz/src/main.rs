#![deny(warnings)]

use std::net::SocketAddr;
use viz::{get, Request, Result, Router, Server, ServiceMaker, Error};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/", get(index));

    Server::bind(&addr)
        .tcp_nodelay(true)
        .serve(ServiceMaker::from(app))
        .await
        .map_err(Error::normal)
}
