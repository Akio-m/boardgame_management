use async_trait::async_trait;
use crate::{domain::boardgames::Boardgames, port::boardgames::BoardgamesPort};

pub struct BoardgamesGateway;

#[async_trait]
impl BoardgamesPort for BoardgamesGateway {
  async fn find_all(&self) -> Result<Boardgames, String> {
    todo!()
  }
}