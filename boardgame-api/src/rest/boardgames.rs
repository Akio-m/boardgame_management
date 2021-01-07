use serde_json::json;
use tide::{Response, Result, StatusCode};

use crate::usecase::get_boardgames;

pub async fn getBoardgames() -> Result<> {
  // let boardgames = get_boardgames.execute().await;
  let body = json!("a");
  Ok(Response::builder(StatusCode::Ok).body(body).build())
}