use tide::Server;
use super::systems;

pub fn get_app() -> Server<()> {
  let mut app = tide::new();
  app = add_route(app);
  app
}

pub fn add_route(mut app: Server<()>) -> Server<()> {
  app.at("/v1/systems/ping").get(|_| async move { systems::ping() });
  app
}