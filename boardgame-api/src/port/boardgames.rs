use crate::domain::boardgames::Boardgames;
use async_trait::async_trait;

use mockall::*;

#[automock]
#[async_trait]
pub trait BoardgamesPort: Send + Sync {
    async fn find_all(&self) -> Result<Boardgames, String>;
}
