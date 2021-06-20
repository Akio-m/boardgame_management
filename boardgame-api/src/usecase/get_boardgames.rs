use crate::{domain::boardgames::Boardgames, port::boardgames::BoardgamesPort};

pub struct GetBoardgameUsecase<T: BoardgamesPort> {
    pub boardgame_port: T,
}

impl<T: BoardgamesPort> GetBoardgameUsecase<T> {
    pub async fn execute(&self) -> Result<Boardgames, String> {
        self.boardgame_port.find_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::GetBoardgameUsecase;
    use super::*;
    use crate::domain::{
        ages::Ages, boardgames::Boardgame, manufacturer::Manufacturer, name::Name,
        play_time::PlayTime, players::Players,
    };
    use crate::port::boardgames::*;

    #[async_std::test]
    async fn test_find_boardgame_list() {
        let expected: Result<Boardgames, String> = Ok(Boardgames(vec![Boardgame::new(
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

        let mut boardgame_port_mock = MockBoardgamesPort::new();
        boardgame_port_mock
            .expect_find_all()
            .return_const(Ok(Boardgames(vec![Boardgame::new(
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
            )])));

        let target = GetBoardgameUsecase {
            boardgame_port: boardgame_port_mock,
        };

        assert_eq!(target.execute().await, expected);
    }
}
