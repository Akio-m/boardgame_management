use tide::Server;

pub fn get_app() -> Server<()> {
  let mut app = tide::new();
  app.at("/").get(|_| async { Ok("Hello, world!") });
  app
}