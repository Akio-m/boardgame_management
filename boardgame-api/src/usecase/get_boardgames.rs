use crate::{domain::boardgames::Boardgames, gateway::boardgames::BoardgamesPort};

pub struct GetBoardgameUsecase<T: BoardgamesPort> {
  pub boardgame_port: T
}

impl<T: BoardgamesPort> GetBoardgameUsecase<T> {
  pub async fn execute(&self) -> Result<Boardgames, String> {
    self.boardgame_port.find_all().await
  }
}

#[cfg(test)]
mod tests {
  use std::sync::{Arc, Mutex};

  use async_trait::async_trait;

  use crate::{domain::{ages::Ages, boardgames::{Boardgame, Boardgames}, manufacturer::Manufacturer, name::Name, play_time::PlayTime, players::Players}, gateway::boardgames::BoardgamesPort};
  use super::GetBoardgameUsecase;


  struct MockBoardgamesPort {
    find_all_calls: Arc<Mutex<u8>>,
    find_all_mock: Result<Boardgames, String>,
  }

  #[async_trait]
  impl BoardgamesPort for MockBoardgamesPort {
    async fn find_all(&self) -> Result<Boardgames, String> {
      let mut calls = self.find_all_calls.lock().unwrap();
      *calls += 1;
      self.find_all_mock.clone()
    }
  }


  #[async_std::test]
  async fn test_find_boardgame_list() {
    let boardgame = Boardgame {
      name: Name { name: "name1".to_string(), name_kana: "name_kana1".to_string() },
      players: Players { min: 0, max: 1 },
      play_time: PlayTime { min: 0, max: 30 },
      ages: Ages { value: 10 },
      manufacturere: Manufacturer { value: "maker1".to_string() }
    };

    let calls = Arc::new(Mutex::new(0));

    let boardgame_port_mock = MockBoardgamesPort {
      find_all_calls: calls.clone(),
      find_all_mock: Ok(Boardgames(vec![boardgame])),
    };

    let target = GetBoardgameUsecase {
      boardgame_port: boardgame_port_mock,
    };

    let expected: Result<Boardgames, String> = Ok(
      Boardgames(vec![
        Boardgame::new(
          Name { name: "name1".to_string(), name_kana: "name_kana1".to_string() },
          Players { min: 0, max: 1 },
          PlayTime { min: 0, max: 30 },
          Ages { value: 10 },
          Manufacturer { value: "maker1".to_string() }
        )
      ]));

    assert_eq!(target.execute().await, expected);

    assert_eq!(*calls.lock().unwrap(), 1);
  }
}