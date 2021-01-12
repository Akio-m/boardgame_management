use super::{ages::Ages, manufacturer::Manufacturer, name::Name, play_time::PlayTime, players::Players};


pub type Boardgames = Vec<Boardgame>;

#[derive(Debug)]
pub struct Boardgame {
  pub name: Name,
  pub players: Players,
  pub play_time: PlayTime,
  pub ages: Ages,
  pub manufacturere: Manufacturer,
}

impl Boardgame {
    pub fn new(name: Name, players: Players, play_time: PlayTime, ages: Ages, manufacturere: Manufacturer) -> Self {
      Self { name, players, play_time, ages, manufacturere }
    }
}
