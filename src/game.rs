use crate::{player::Player, selvec::SelVec};

pub enum GameResult {
    Winner(usize),
    Tie,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HazardId {
    Stealthrock,
    Spikes,
    ToxicSpikes,
}

impl HazardId {
    pub fn to_string(&self) -> String {
        match self {
            HazardId::Stealthrock => String::from("stealth rock"),
            HazardId::Spikes => String::from("spikes"),
            HazardId::ToxicSpikes => String::from("toxic spikes"),
        }
    }

    pub fn max_layers(&self) -> u8 {
        match self {
            HazardId::Stealthrock => 1u8,
            HazardId::Spikes => 2u8,
            HazardId::ToxicSpikes => 3u8,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WeatherId {
    Sand,
    Rain,
    Hail,
}

#[derive(Debug, Default)]
pub struct Game {
    pub players: SelVec<Player>,
    pub turn_count: i32,
    pub weather: Option<WeatherId>,
    pub active_player: usize,
    pub log: Vec<String>,
}
