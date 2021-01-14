use tide::Error;
use crate::{domain::boardgames::Boardgames, gateway::boardgames::BoardgamesPort};

pub struct GetBoardgameUsecase<T: BoardgamesPort> {
  pub boardgame_port: T
}

impl<T: BoardgamesPort> GetBoardgameUsecase<T> {
  pub async fn execute(&self) -> Result<Boardgames, Error> {
    Ok(
      Boardgames::new()
    )
  }
}

#[cfg(test)]
mod tests {

  #[async_std::test]
  async fn test_find_boardgame_list() {
      assert_eq!(4, 4);
  }
}