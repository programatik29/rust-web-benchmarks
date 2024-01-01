use salvo::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get(hello);
    Server::new(TcpListener::new("127.0.0.1:3000").bind().await)
        .serve(router)
        .await
}
