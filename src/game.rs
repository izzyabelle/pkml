use crate::{player::Player, selvec::PointerVec};

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
    pub players: PointerVec<Player>,
    pub turn_count: i32,
    pub weather: Option<WeatherId>,
    pub log: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: PointerVec::from(vec![Player::new(false), Player::new(true)]),
            turn_count: 0,
            weather: None,
            log: vec![String::new()],
        }
    }
}
