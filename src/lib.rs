use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use tower_service::Service;
use worker::*;

const BASIC_AUTH_USER: &str = "admin";
const BASIC_AUTH_PASSWORD: &str = "changeme";

async fn root() -> &'static str {
    "Hello Axum!"
}

async fn require_basic_auth(
    auth: Option<TypedHeader<Authorization<Basic>>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let is_authorized = auth
        .map(|TypedHeader(Authorization(credentials))| {
            credentials.username() == BASIC_AUTH_USER
                && credentials.password() == BASIC_AUTH_PASSWORD
        })
        .unwrap_or(false);

    if !is_authorized {
        return (
            StatusCode::UNAUTHORIZED,
            [(
                header::WWW_AUTHENTICATE,
                r#"Basic realm="cf-rust-deploy", charset="UTF-8""#,
            )],
            "Unauthorized",
        )
            .into_response();
    }

    next.run(req).await
}

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .layer(middleware::from_fn(require_basic_auth))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}
