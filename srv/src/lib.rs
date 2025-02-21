mod gen_captcha;

use aok::Void;
use axum::{Router, body::Bytes, http::HeaderMap, response::Response, routing::put};
use gen_captcha::GenCaptcha;
mod _hpc;

const BATCH_LIMIT: usize = 64;

#[axum::debug_handler]
pub async fn hpc(headers: HeaderMap, body: Bytes) -> Response {
  hpc::run::<BATCH_LIMIT, _hpc::Hpc, GenCaptcha>(headers, body).await
}

genv::def!(PORT:u16 | 2025);

pub async fn srv() -> Void {
  let port = PORT();
  let router = Router::new();
  let router = router.route("/", put(hpc));
  let router = axum_layer::layer(router);
  hpc::srv(port, router).await
}
