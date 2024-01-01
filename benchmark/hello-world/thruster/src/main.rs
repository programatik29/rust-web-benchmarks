use thruster::BasicContext as Context;
use thruster::{
    m, middleware_fn, App, MiddlewareNext, MiddlewareResult, Request, Server, ThrusterServer,
};

#[middleware_fn]
async fn hello(mut ctx: Context, _next: MiddlewareNext<Context>) -> MiddlewareResult<Context> {
    ctx.body("Hello, world!");
    Ok(ctx)
}

fn main() {
    let app = App::<Request, Context, ()>::new_basic().get("/", m![hello]);

    Server::new(app).start("127.0.0.1", 3000);
}
