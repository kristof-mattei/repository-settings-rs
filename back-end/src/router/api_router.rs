use axum::extract::Json;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use tracing::{event, Level};

use crate::github;
use crate::state::ApplicationState;

pub(crate) fn build_api_router(state: ApplicationState) -> Router {
    Router::new()
        .with_state(state)
        .route("/", post(handler_200))
}

#[allow(clippy::unused_async)]
async fn handler_200(
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    match github::handle_webhook(headers, payload).await {
        Ok(()) => (StatusCode::OK, "handled"),
        Err(err) => {
            event!(Level::ERROR, ?err, "error processing webhook");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "you might want to check the fan",
            )
        },
    }
}
