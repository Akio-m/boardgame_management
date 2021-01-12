use tide::Error;

use crate::domain::boardgames::Boardgames;

pub async fn execute() -> Result<Boardgames, Error> {
  Ok(
    Boardgames::new()
  )
}

#[cfg(test)]
mod tests {

  #[test]
  fn find_boardgame_list() {
      assert_eq!(4, 4);
  }
}