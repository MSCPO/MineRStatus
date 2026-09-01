//! Axum application construction and HTTP handlers.
//!
//! The router built here is shared between the local development binary
//! (`src/main.rs`) and the Vercel serverless entrypoint (`api/axum.rs`).

use axum::{
    Json, Router,
    extract::{FromRequestParts, Query, State},
    http::{Method, StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
};
use base64::Engine;
use serde::Deserialize;
use serde_json::json;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::{IntoParams, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::status::{AppState, ErrorResponse, MotdResponse, PlayersResponse, StatusResponse};

#[derive(Debug, Deserialize, IntoParams)]
struct QueryParams {
    ip: String,
    cache: Option<bool>,
}

impl QueryParams {
    fn use_cache(&self) -> bool {
        self.cache.unwrap_or(true)
    }
}

/// Custom extractor that mirrors FastAPI's 422 validation error format.
///
/// When `ip` (or another field) fails to deserialize from the query string,
/// axum's default `Query` extractor would return a plain-text 400. This
/// wrapper instead responds with `422 Unprocessable Entity` and a structured
/// JSON body matching FastAPI / Pydantic's `detail` array.
#[derive(Debug)]
struct ValidatedQuery(QueryParams);

impl<S> FromRequestParts<S> for ValidatedQuery
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match Query::<QueryParams>::from_request_parts(parts, state).await {
            Ok(Query(params)) => Ok(ValidatedQuery(params)),
            Err(rejection) => Err(validation_error_response(&rejection)),
        }
    }
}

/// Build a FastAPI-style 422 response for a query deserialization rejection.
fn validation_error_response(
    rejection: &axum::extract::rejection::QueryRejection,
) -> Response {
    let details = match rejection {
        axum::extract::rejection::QueryRejection::FailedToDeserializeQueryString(err) => {
            json!([{
                "type": "missing",
                "loc": ["query", "ip"],
                "msg": err.body_text(),
                "input": null,
            }])
        }
        _ => {
            json!([{
                "type": "invalid_request",
                "loc": ["query"],
                "msg": rejection.body_text(),
                "input": null,
            }])
        }
    };

    (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({ "detail": details }))).into_response()
}

impl std::ops::Deref for ValidatedQuery {
    type Target = QueryParams;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/",
    tag = "Minecraft Status",
    responses(
        (
            status = 200,
            description = "Server status (or an error message when unreachable)",
            body = StatusResponse,
        ),
    ),
)]
async fn status_unclassified(
    State(state): State<AppState>,
    ValidatedQuery(params): ValidatedQuery,
) -> Response {
    match crate::status::query_unclassified(&state, &params.ip, params.use_cache()).await {
        Ok(resp) => Json(resp).into_response(),
        Err(err) => Json(ErrorResponse { error: err.0 }).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/java",
    tag = "Minecraft Status",
    responses(
        (
            status = 200,
            description = "Server status, or the icon PNG when ip ends with /icon",
            body = StatusResponse,
        ),
        (
            status = 404,
            description = "Server unreachable or no icon available",
            body = ErrorResponse,
        ),
    ),
)]
async fn status_java(
    State(state): State<AppState>,
    ValidatedQuery(params): ValidatedQuery,
) -> Response {
    let use_cache = params.use_cache();
    if params.ip.ends_with("/icon") {
        let host = &params.ip[..params.ip.len() - "/icon".len()];
        return get_java_icon(&state, host, use_cache).await;
    }
    match crate::status::query_java(&state, &params.ip, use_cache).await {
        Ok(resp) => Json(resp).into_response(),
        Err(err) => Json(ErrorResponse { error: err.0 }).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/bedrock",
    tag = "Minecraft Status",
    responses(
        (
            status = 200,
            description = "Server status (or an error message when unreachable)",
            body = StatusResponse,
        ),
    ),
)]
async fn status_bedrock(
    State(state): State<AppState>,
    ValidatedQuery(params): ValidatedQuery,
) -> Response {
    match crate::status::query_bedrock(&state, &params.ip, params.use_cache()).await {
        Ok(resp) => Json(resp).into_response(),
        Err(err) => Json(ErrorResponse { error: err.0 }).into_response(),
    }
}

/// Serve the Java server favicon as a PNG image (HTTP 404 when unavailable).
async fn get_java_icon(state: &AppState, host: &str, use_cache: bool) -> Response {
    let resp = match crate::status::query_java(state, host, use_cache).await {
        Ok(resp) => resp,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse { error: err.0 }),
            )
                .into_response();
        }
    };

    let Some(icon) = resp.icon else {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Server has no icon".to_string(),
            }),
        )
            .into_response();
    };

    let encoded = icon.split(',').last().unwrap_or(&icon);
    let bytes = match base64::engine::general_purpose::STANDARD.decode(encoded.trim()) {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Invalid icon data".to_string(),
                }),
            )
                .into_response();
        }
    };

    (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], bytes).into_response()
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service health check and version"),
    ),
)]
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

// ---------------------------------------------------------------------------
// OpenAPI / Swagger
// ---------------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(
        status_unclassified,
        status_java,
        status_bedrock,
        health,
    ),
    components(
        schemas(StatusResponse, PlayersResponse, MotdResponse, ErrorResponse)
    ),
    info(
        title = "MineRStatus API",
        description = "A lightweight Minecraft server status query API (Rust).",
        version = env!("CARGO_PKG_VERSION"),
    ),
)]
struct ApiDoc;

// ---------------------------------------------------------------------------
// Application
// ---------------------------------------------------------------------------

/// Build the fully configured axum router for the given application state.
pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([header::ACCEPT, header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(status_unclassified))
        .route("/java", get(status_java))
        .route("/java/", get(status_java))
        .route("/bedrock", get(status_bedrock))
        .route("/bedrock/", get(status_bedrock))
        .route("/health", get(health))
        .with_state(state);

    app.merge(
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
    .layer(cors)
    .layer(TraceLayer::new_for_http())
}