use tide::Error;

use crate::domain::boardgames::Boardgames;

pub async fn execute() -> Result<Boardgames, Error> {
  Ok(Boardgames{})
}