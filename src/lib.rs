use worker::*;

fn root_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!")
}

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let router = Router::new();
    router
        .get("/", root_handler)
        .run(_req, _env)
        .await
}
