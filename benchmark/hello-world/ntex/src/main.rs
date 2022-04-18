use ntex::web::{self, middleware, App, HttpRequest};

async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
