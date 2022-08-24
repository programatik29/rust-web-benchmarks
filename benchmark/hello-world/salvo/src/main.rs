use salvo::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get(hello);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await
}
