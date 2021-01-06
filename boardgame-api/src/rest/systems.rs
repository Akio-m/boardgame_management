use serde_json::json;
use tide::{Response, Result, StatusCode};

pub fn ping() -> Result<>{
  Ok(Response::builder(StatusCode::Ok).body(json!({ "ping": "pong"})).build())
}