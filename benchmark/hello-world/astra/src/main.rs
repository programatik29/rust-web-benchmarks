use astra::{Body, Response, Server};

fn main() {
    Server::bind("127.0.0.1:3000")
        .serve(|_req, _info| Response::new(Body::new("Hello, World!")))
        .unwrap()
}
