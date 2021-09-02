use crate::{
    domain::{
        ages::Ages,
        boardgames::{Boardgame, Boardgames},
        manufacturer::Manufacturer,
        name::Name,
        play_time::PlayTime,
        players::Players,
    },
    driver::boardgames_driver::BoardgamesDb,
    port::boardgames::BoardgamesPort,
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct BoardgamesGateway<T: BoardgamesDb> {
    pub db: T,
}

#[async_trait]
impl<T: BoardgamesDb + Sync + Send> BoardgamesPort for BoardgamesGateway<T> {
    async fn find_all(&self) -> Result<Boardgames, String> {
        let boardgames = self.db.find_boardgames().unwrap();
        Ok(Boardgames(
            boardgames
                .into_iter()
                .map(|it| Boardgame {
                    name: Name {
                        name: it.name,
                        name_kana: it.name_kana,
                    },
                    players: Players {
                        min: it.players_min.unwrap(),
                        max: it.players_max.unwrap(),
                    },
                    play_time: PlayTime {
                        min: it.play_time_min.unwrap(),
                        max: it.play_time_max.unwrap(),
                    },
                    ages: Ages {
                        value: it.ages.unwrap(),
                    },
                    manufacturere: Manufacturer {
                        value: it.manufacturer.unwrap(),
                    },
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{
        ages::Ages,
        boardgames::{Boardgame, Boardgames},
        manufacturer::Manufacturer,
        name::Name,
        play_time::PlayTime,
        players::Players,
    };
    use crate::driver::boardgames_driver::Boardgame as BoardgameRecord;
    use crate::driver::boardgames_driver::*;

    #[async_std::test]
    async fn test_find_all() {
        let mut boardgame_db_mock = MockBoardgamesDb::new();
        boardgame_db_mock
            .expect_find_boardgames()
            .return_const(Ok(vec![BoardgameRecord {
                name: "name1".to_string(),
                name_kana: "name_kana1".to_string(),
                players_min: Some(0),
                players_max: Some(1),
                play_time_min: Some(0),
                play_time_max: Some(30),
                ages: Some(10),
                manufacturer: Some("maker1".to_string()),
            }]));

        let target = BoardgamesGateway {
            db: boardgame_db_mock,
        };

        let expected = Ok(Boardgames(vec![Boardgame::new(
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
        )]));

        let actual = target.find_all().await;
        assert_eq!(actual, expected);
    }
}
