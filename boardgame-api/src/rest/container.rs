use crate::{driver::boardgames_driver::BoardgamesDbImpl, gateway::boardgames::BoardgamesGateway};

// NOTE: TideはSingletonのShared dataを持つことができなさそうなのでStateを回している
#[derive(Clone)]
pub struct Container {
  pub boardgames_port: BoardgamesGateway<BoardgamesDbImpl>,
}

impl Container {
  pub fn new() -> Self {
    Self {
      boardgames_port: BoardgamesGateway {
        db: BoardgamesDbImpl {
          pool: r2d2::Pool::builder().build(
            diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new("postgres://admin:admin@localhost:5432/boardgame?options=-c search_path%3Dboardgame"))
            .expect("connection作れなかったわ"),
        }
      },
    }
  }
}