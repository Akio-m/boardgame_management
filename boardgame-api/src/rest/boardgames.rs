use serde_json::json;
use serde::{Serialize, Deserialize};
use tide::{Response, Result, StatusCode};

use crate::{domain::boardgames::Boardgame, usecase::get_boardgames};
use crate::domain::boardgames::Boardgames;

pub async fn get_boardgames() -> Result<> {
  let boardgames = get_boardgames::execute().await.unwrap();
  let body = json!(BoardgamesJson::from(boardgames));
  Ok(Response::builder(StatusCode::Ok).body(body).build())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BoardgamesJson {
  boardgames: Vec<BoardgameJson>,
}

impl BoardgamesJson {
  fn from(boardgames: Boardgames) -> Self {
    BoardgamesJson {
        boardgames: boardgames.into_iter()
        .map(|v| BoardgameJson::from(v)).collect(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BoardgameJson {
  name: String,
  name_kana: String,
  players_min: String,
  players_max: String,
  play_time_min: String,
  play_time_max: String,
  ages: String,
  manufacturer: String
}

impl BoardgameJson {
  fn from(boardgame: Boardgame) -> Self {
    Self {
        name: boardgame.name.name,
        name_kana: boardgame.name.name_kana,
        players_min: if boardgame.players.min == u32::MIN { "".to_string() } else { boardgame.players.min.to_string() },
        players_max: if boardgame.players.max == u32::MAX { "".to_string() } else { boardgame.players.max.to_string() },
        play_time_min: if boardgame.play_time.min == u32::MIN { "".to_string() } else { boardgame.play_time.min.to_string() },
        play_time_max: if boardgame.play_time.max == u32::MAX { "".to_string() } else { boardgame.play_time.max.to_string() },
        ages: if boardgame.ages.value == u32::MIN { "".to_string() } else { boardgame.ages.value.to_string() },
        manufacturer: boardgame.manufacturere.value,
    }
  }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::domain::{ages::Ages, boardgames::{Boardgame, Boardgames}, manufacturer::Manufacturer, name::Name, play_time::PlayTime, players::Players};
    use super::{BoardgameJson, BoardgamesJson};

    #[test]
    fn boardgames_to_json() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn boardgame_to_json() {

      let target: Boardgames = vec![
        Boardgame {
          name: Name { name: "name1".to_string(), name_kana: "name_kana1".to_string() },
          players: Players { min: u32::MIN, max: 1 },
          play_time: PlayTime { min: u32::MIN, max: 30},
          ages: Ages { value: 10 },
          manufacturere: Manufacturer { value: "maker1".to_string() },
        },
        Boardgame {
          name: Name { name: "name2".to_string(), name_kana: "name_kana2".to_string() },
          players: Players { min: 2, max: u32::MAX },
          play_time: PlayTime { min: 60, max: u32::MAX },
          ages: Ages { value: u32::MIN },
          manufacturere: Manufacturer { value: "maker2".to_string() },
        }
      ];

      let expected = BoardgamesJson {
        boardgames: vec![
          BoardgameJson {
            name: "name1".to_string(),
            name_kana: "name_kana1".to_string(),
            players_min: "".to_string(),
            players_max: "1".to_string(),
            play_time_min: "".to_string(),
            play_time_max: "30".to_string(),
            ages: "10+".to_string(),
            manufacturer: "maker1".to_string(),
          },
          BoardgameJson {
            name: "name2".to_string(),
            name_kana: "name_kana2".to_string(),
            players_min: "2".to_string(),
            players_max: "".to_string(),
            play_time_min: "60".to_string(),
            play_time_max: "".to_string(),
            ages: "".to_string(),
            manufacturer: "maker2".to_string(),
          }
        ]
      };
      let actual = BoardgamesJson::from(target);
      assert_eq!(actual, expected);
    }
}