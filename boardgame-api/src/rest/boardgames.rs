use diesel;
use r2d2;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::{Response, Result, StatusCode};

use crate::domain::boardgames::Boardgames;
use crate::driver::boardgames_driver::BoardgamesDbImpl;
use crate::{
    domain::boardgames::Boardgame, gateway::boardgames::BoardgamesGateway,
    usecase::get_boardgames::GetBoardgameUsecase,
};

pub async fn get_boardgames() -> Result {
    let usecase = GetBoardgameUsecase {
        boardgame_port: BoardgamesGateway {
            db: BoardgamesDbImpl {
                pool: r2d2::Pool::builder()
                .build(diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new("postgres://admin:admin@localhost:5432/boardgame?options=-c search_path%3Dboardgame")).expect("connection作れなかったわ"),
            }
        },
    };
    let boardgames = usecase.execute().await.unwrap();
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
            boardgames: boardgames
                .0
                .into_iter()
                .map(|v| BoardgameJson::from(v))
                .collect(),
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
    manufacturer: String,
}

impl BoardgameJson {
    fn from(boardgame: Boardgame) -> Self {
        Self {
            name: boardgame.name.name,
            name_kana: boardgame.name.name_kana,
            players_min: if boardgame.players.min == 0 {
                "".to_string()
            } else {
                boardgame.players.min.to_string()
            },
            players_max: if boardgame.players.max == i32::MAX {
                "".to_string()
            } else {
                boardgame.players.max.to_string()
            },
            play_time_min: if boardgame.play_time.min == 0 {
                "".to_string()
            } else {
                boardgame.play_time.min.to_string()
            },
            play_time_max: if boardgame.play_time.max == i32::MAX {
                "".to_string()
            } else {
                boardgame.play_time.max.to_string()
            },
            ages: if boardgame.ages.value == 0 {
                "".to_string()
            } else {
                boardgame.ages.value.to_string() + "+"
            },
            manufacturer: boardgame.manufacturere.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BoardgameJson, BoardgamesJson};
    use crate::domain::{
        ages::Ages,
        boardgames::{Boardgame, Boardgames},
        manufacturer::Manufacturer,
        name::Name,
        play_time::PlayTime,
        players::Players,
    };
    use std::vec;

    #[test]
    fn test_boardgame_to_json() {
        let target = Boardgames(vec![
            Boardgame::new(
                Name {
                    name: "name1".to_string(),
                    name_kana: "name_kana1".to_string(),
                },
                Players { min: 0, max: 1 },
                PlayTime { min: 0, max: 30 },
                Ages { value: 10 },
                Manufacturer {
                    value: "maker1".to_string(),
                },
            ),
            Boardgame::new(
                Name {
                    name: "name2".to_string(),
                    name_kana: "name_kana2".to_string(),
                },
                Players {
                    min: 2,
                    max: i32::MAX,
                },
                PlayTime {
                    min: 60,
                    max: i32::MAX,
                },
                Ages { value: 0 },
                Manufacturer {
                    value: "maker2".to_string(),
                },
            ),
        ]);

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
                },
            ],
        };
        let actual = BoardgamesJson::from(target);
        assert_eq!(actual, expected);
    }
}
