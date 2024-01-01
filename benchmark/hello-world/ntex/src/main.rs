use std::io;

use ntex::{
    http::{header::HeaderValue, HttpService, Response},
    server::Server,
    time::Seconds,
    util::Ready,
};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    Server::build()
        .bind("hello-world", "127.0.0.1:3000", |_| {
            HttpService::build()
                .headers_read_rate(Seconds(1), Seconds(3), 128)
                .disconnect_timeout(Seconds(1))
                .finish(|_req| {
                    let mut res = Response::Ok();
                    res.header("x-head", HeaderValue::from_static("dummy value!"));
                    Ready::Ok::<_, io::Error>(res.body("Hello, world!"))
                })
        })?
        .run()
        .await
}
