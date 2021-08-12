use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use mockall::*;
use r2d2::Pool;

#[derive(Queryable, Debug, Clone)]
pub struct Boardgame {
    pub name: String,
    pub name_kana: String,
    pub players_min: Option<i32>,
    pub players_max: Option<i32>,
    pub play_time_min: Option<i32>,
    pub play_time_max: Option<i32>,
    pub ages: Option<i32>,
    pub manufacturer: Option<String>,
}

#[automock]
#[async_trait]
pub trait BoardgamesDb {
    fn find_boardgames(&self) -> Result<Vec<Boardgame>, String>;
}

pub struct BoardgamesDbImpl {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl BoardgamesDb for BoardgamesDbImpl {
    fn find_boardgames(&self) -> Result<Vec<Boardgame>, String> {
        use crate::driver::schema::boardgame::dsl::*;
        let connection = &self.pool.get().unwrap();
        let result = boardgame
            .load::<Boardgame>(connection)
            .expect("Error loading boardgame");
        Ok(result)
    }
}
