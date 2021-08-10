use actix_web::{web, App, HttpServer};

async fn index() -> &'static str {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
