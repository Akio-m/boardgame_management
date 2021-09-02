use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Server};

use super::container::Container;
use super::{boardgames, systems};

pub fn get_app() -> Server<Container> {
    let mut app = tide::with_state(Container::new());
    let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));
    app.with(cors);
    app = add_route(app);
    app
}

pub fn add_route(mut app: Server<Container>) -> Server<Container> {
    app.at("/v1/systems/ping")
        .get(|_| async move { systems::ping().await });
    app.at("/v1/boardgames")
        .get(|req: Request<Container>| async move { boardgames::get_boardgames(&req.state().boardgames_port).await });
    app
}
