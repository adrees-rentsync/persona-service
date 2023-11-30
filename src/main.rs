mod consts;
mod persona;
mod types;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower_http::trace::{self, TraceLayer};
use serde_json::{Map, Value};
use tracing_subscriber;
use tracing::Level;

use crate::types::{PersonaUrlPayload, RcaResponse, UserId};

async fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn get_inquiry_url(Json(input): Json<PersonaUrlPayload>) -> impl IntoResponse {
    println!("{:?}", input);

    let engine = persona::engine::get_persona_engine().await;
    let uid = UserId::from(input);

    let persona_inquiry = match engine.create_inquiry(uid.into()).await {
        Result::Ok(inq) => inq.data,
        Result::Err(_) => {
            let mut meta = Map::new();
            meta.insert("success".to_string(), Value::Bool(false));

            let response = RcaResponse {
                meta: meta,
                data: Map::new(),
            };

            return (StatusCode::SERVICE_UNAVAILABLE, Json(response));
        }
    };

    let url = engine.get_inquiry_url(persona_inquiry.id, None);

    let mut meta = Map::new();
    meta.insert("success".to_string(), Value::Bool(true));

    let mut data = Map::new();
    data.insert("url".to_string(), Value::String(url));

    let response = RcaResponse {
        meta: meta,
        data: data,
    };

    return (StatusCode::OK, Json(response));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/get-inquiry-url", post(get_inquiry_url))
        .route("/health-check", get(health_check));
        
    let port = 8000;

    let bind = format!("0.0.0.0:{}", port);

    axum::Server::bind(&bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
