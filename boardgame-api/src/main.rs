use boardgame_api::rest::app::get_app;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let app = get_app();

    app.listen("127.0.0.1:21001").await?;
    Ok(())
}