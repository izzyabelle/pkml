use crate::effect::PlayerId;
use crate::moves::MoveId;
use crate::moves::Priority;
use crate::player::Player;
use crate::pokemon::Pokemon;
use crate::stat::StatId;

use crate::EmptyResult;

type Layers = u8;

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
    pub players: [Player; 2],
    pub turn_count: i32,
    pub weather: Option<WeatherId>,
    pub active_player: usize,
    pub log: Vec<String>,
}

impl Game {
    pub fn set_weather(&mut self, weather: WeatherId) -> bool {
        let success;
        if let Some(weather_) = self.weather {
            success = weather != weather_;
        } else {
            success = true;
        }
        self.weather = Some(weather);
        success
    }

    pub fn init_order(&mut self) -> EmptyResult<()> {
        let speed_1 = self
            .active(&PlayerId::Active)
            .get_stat(&StatId::Spe, &self.weather);
        let speed_2 = self
            .active(&PlayerId::Inactive)
            .get_stat(&StatId::Spe, &self.weather);

        // check if action is switch then determine priority
        let priority_1 = Priority::from(self.player(&PlayerId::Active).active_move());
        let priority_2 = Priority::from(self.player(&PlayerId::Inactive).active_move());

        // move 1 priority is greater
        if priority_1 > priority_2
            || {
                // priority is the same but speed is greater
                priority_1 == priority_2 && speed_1 > speed_2
            }
            || {
                // speed tie
                priority_1 == priority_2 && speed_1 == speed_2 && rand::random()
            }
        {
            self.invert_player().unwrap();
        }
        Ok(())
    }

    pub fn check_prerequisites(&self) -> bool {
        if self.active(&PlayerId::Inactive).hp != 0 {
            return true;
        }

        match self.player(&PlayerId::Active).active_move() {
            MoveId::Roost => self.active(&PlayerId::Active).hp.is_max(),
            MoveId::Refresh => !self.active(&PlayerId::Active).has_nv_status(),
            MoveId::Thunderwave => self.active(&PlayerId::Inactive).has_nv_status(),
            MoveId::Stealthrock => self.player(&PlayerId::Inactive).hazards[0] == 1,
            _ => false,
        }
    }

    pub fn winner_check(&self) -> Option<GameResult> {
        if self.player(&PlayerId::Active).remaining_mons == 0
            && self.player(&PlayerId::Inactive).remaining_mons == 0
        {
            return Some(GameResult::Tie);
        }
        let mut curr = PlayerId::Active;
        for i in 0..2 {
            if self.player(&curr).remaining_mons == 0 {
                return Some(GameResult::Winner((i + 1) % 2));
            }
            curr.invert().unwrap();
        }
        None
    }

    pub fn invert_player(&mut self) -> EmptyResult<()> {
        self.active_player = (self.active_player + 1) % 2;
        Ok(())
    }

    pub fn player(&self, target: &PlayerId) -> &Player {
        match target {
            PlayerId::Active => &self.players[self.active_player],
            PlayerId::Inactive => &self.players[(self.active_player + 1) % 2],
            PlayerId::Player1 => &self.players[0],
            PlayerId::Player2 => &self.players[1],
        }
    }

    pub fn player_mut(&mut self, target: &PlayerId) -> &mut Player {
        match target {
            PlayerId::Active => &mut self.players[self.active_player],
            PlayerId::Inactive => &mut self.players[(self.active_player + 1) % 2],
            PlayerId::Player1 => &mut self.players[0],
            PlayerId::Player2 => &mut self.players[1],
        }
    }

    pub fn active(&self, target: &PlayerId) -> &Pokemon {
        self.player(&target).get_active()
    }

    pub fn active_mut(&mut self, target: &PlayerId) -> &mut Pokemon {
        self.player_mut(&target).get_active_mut()
    }
}
