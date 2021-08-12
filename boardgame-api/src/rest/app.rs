use tide::security::{CorsMiddleware, Origin};
use tide::Server;

use super::{boardgames, systems};

pub fn get_app() -> Server<()> {
    let mut app = tide::new();
    let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));
    app.with(cors);
    app = add_route(app);
    app
}

pub fn add_route(mut app: Server<()>) -> Server<()> {
    app.at("/v1/systems/ping")
        .get(|_| async move { systems::ping().await });
    app.at("/v1/boardgames")
        .get(|_| async move { boardgames::get_boardgames().await });
    app
}
