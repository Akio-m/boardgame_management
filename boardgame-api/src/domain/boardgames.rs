use super::{
    ages::Ages, manufacturer::Manufacturer, name::Name, play_time::PlayTime, players::Players,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Boardgames(pub Vec<Boardgame>);

impl From<Vec<Boardgame>> for Boardgames {
    fn from(boardgames: Vec<Boardgame>) -> Boardgames {
        Boardgames(boardgames)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Boardgame {
    pub name: Name,
    pub players: Players,
    pub play_time: PlayTime,
    pub ages: Ages,
    pub manufacturere: Manufacturer,
}

impl Boardgame {
    pub fn new(
        name: Name,
        players: Players,
        play_time: PlayTime,
        ages: Ages,
        manufacturere: Manufacturer,
    ) -> Self {
        Self {
            name,
            players,
            play_time,
            ages,
            manufacturere,
        }
    }
}
