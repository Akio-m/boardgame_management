use tide::Server;

use super::{boardgames, systems};

pub fn get_app() -> Server<()> {
  let mut app = tide::new();
  app = add_route(app);
  app
}

pub fn add_route(mut app: Server<()>) -> Server<()> {
  app.at("/v1/systems/ping").get(|_| async move { systems::ping().await });
  app.at("/v1/boardgams").get(|_| async move { boardgames::getBoardgames().await });
  app
}