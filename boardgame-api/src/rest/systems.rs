use serde_json::json;
use tide::{Response, Result, StatusCode};

pub async fn ping() -> Result {
    Ok(Response::builder(StatusCode::Ok)
        .body(json!({ "ping": "pong"}))
        .build())
}
