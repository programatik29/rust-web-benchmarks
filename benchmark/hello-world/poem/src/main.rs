use poem::{
    get, handler, listener::TcpListener, Route, Server,
};

#[handler]
fn hello() -> String {
    format!("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await
}
