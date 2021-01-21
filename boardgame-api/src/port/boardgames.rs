use async_trait::async_trait;

use crate::domain::boardgames::Boardgames;

#[async_trait]
pub trait BoardgamesPort {
  async fn find_all(&self) -> Result<Boardgames, String>;
}