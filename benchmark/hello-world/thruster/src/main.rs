use thruster::{App, Request, middleware_fn, MiddlewareNext, MiddlewareResult, m, Server, ThrusterServer};
use thruster::BasicContext as Context;

#[middleware_fn]
async fn hello(mut ctx: Context, _next: MiddlewareNext<Context>) -> MiddlewareResult<Context> {
    ctx.body("Hello, world!");
    Ok(ctx)
}

fn main() {
    let app = App::<Request, Context, ()>::new_basic().get("/", m![hello]);

    Server::new(app).start("127.0.0.1", 3000);
}
