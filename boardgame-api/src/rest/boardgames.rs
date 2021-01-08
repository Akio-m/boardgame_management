use serde_json::json;
use serde::{Serialize, Deserialize};
use tide::{Response, Result, StatusCode};

use crate::usecase::get_boardgames;
use crate::domain::boardgames::Boardgames;

pub async fn getBoardgames() -> Result<> {
  let boardgames = get_boardgames::execute().await.unwrap();
  let body = json!(BoardgamesJson::from(boardgames));
  Ok(Response::builder(StatusCode::Ok).body(body).build())
}
#[derive(Serialize, Deserialize, Debug)]
struct BoardgamesJson {}

impl BoardgamesJson {
  fn from(boardgames: Boardgames) -> Self {
    BoardgamesJson {}
  }
}