use std::{default, fmt::Display};

use crate::{effect::PlayerId, player::Player, pokemon::Pokemon, selvec::PointerVec};

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

impl Display for HazardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HazardId::Stealthrock => String::from("Stealth Rock"),
                HazardId::Spikes => String::from("Spikes"),
                HazardId::ToxicSpikes => String::from("Toxic Spikes"),
            }
        )
    }
}

impl HazardId {
    pub fn max_layers(&self) -> u8 {
        match self {
            HazardId::Stealthrock => 1u8,
            HazardId::Spikes => 2u8,
            HazardId::ToxicSpikes => 3u8,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum WeatherId {
    #[default]
    Sand,
    Rain,
    Hail,
}

#[derive(Debug, Default)]
pub struct Game {
    pub players: PointerVec<Player>,
    pub turn_count: i32,
    pub weather: Option<WeatherId>,
    pub log: Vec<Vec<String>>,
}

pub enum MoveSelection {
    Switch(usize),
    Move(usize),
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: PointerVec::from(vec![Player::new(false), Player::new(true)]),
            turn_count: 0,
            weather: None,
            log: vec![vec![]],
        }
    }

    pub fn log(&mut self, message: String) {
        self.log.last_mut().unwrap().push(message);
    }

    pub fn player(&self, target: &PlayerId) -> Option<&Player> {
        match target {
            PlayerId::Player1 => Some(&self.players[0]),
            PlayerId::Player2 => Some(&self.players[1]),
            PlayerId::Active => self.players.active(),
            PlayerId::Inactive => {
                if let Some(data) = self.players.active {
                    Some(&self.players[(data + 1) % 2])
                } else {
                    None
                }
            }
        }
    }

    pub fn player_mut(&mut self, target: &PlayerId) -> Option<&mut Player> {
        match target {
            PlayerId::Player1 => Some(&mut self.players[0]),
            PlayerId::Player2 => Some(&mut self.players[1]),
            PlayerId::Active => self.players.active_mut(),
            PlayerId::Inactive => {
                if let Some(data) = self.players.active {
                    Some(&mut self.players[(data + 1) % 2])
                } else {
                    None
                }
            }
        }
    }

    pub fn active(&self, target: &PlayerId) -> Option<&Pokemon> {
        if let Some(player) = self.player(target) {
            player.roster.active()
        } else {
            None
        }
    }

    pub fn active_mut(&mut self, target: &PlayerId) -> Option<&mut Pokemon> {
        if let Some(player) = self.player_mut(target) {
            player.roster.active_mut()
        } else {
            None
        }
    }

    // pub fn execute_turn(&mut self, )
}
