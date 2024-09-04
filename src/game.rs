use crate::status::Status;
use crate::{effect::Damage, poketype::Type};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::{default, fmt::Display};

use crate::{
    effect::{Effect, PlayerId},
    player::Player,
    pokemon::Pokemon,
    selvec::PointerVec,
    stat::StatId,
};

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
    pub state: GameState,
    pub prev_state: GameState,
    pub prev_prev_state: GameState,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    #[default]
    TurnStart,
    MidTurn,
    TurnEnd,
    AwaitingSwitch,
    Completed(GameResult),
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TurnStart => String::from("Turn Start"),
                Self::MidTurn => String::from("Mid Turn"),
                Self::TurnEnd => String::from("Turn End"),
                Self::AwaitingSwitch => String::from("Awaiting Switch Input"),
                Self::Completed(data) => match data {
                    GameResult::Winner(winner) => format!("Player {} won", winner + 1),
                    GameResult::Tie => String::from("Game tied"),
                    GameResult::Incomplete => panic!("somethin wrongggg"),
                },
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Winner(usize),
    Tie,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveSelection {
    Switch(usize),
    Move(usize),
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: PointerVec::from(vec![Player::new(false), Player::new(true)]),
            log: vec![vec![String::from("Players sent out their starters!")]],
            ..Default::default()
        }
    }

    pub fn log(&mut self, message: String) {
        self.log.last_mut().unwrap().push(message);
    }

    pub fn player(&self, target: &PlayerId) -> &Player {
        match target {
            PlayerId::Player1 => &self.players[0],
            PlayerId::Player2 => &self.players[1],
            PlayerId::Active => self.players.active().unwrap(),
            PlayerId::Inactive => {
                let active = self.players.active.unwrap();
                &self.players[(active + 1) % 2]
            }
        }
    }

    pub fn player_mut(&mut self, target: &PlayerId) -> &mut Player {
        match target {
            PlayerId::Player1 => &mut self.players[0],
            PlayerId::Player2 => &mut self.players[1],
            PlayerId::Active => self.players.active_mut().unwrap(),
            PlayerId::Inactive => {
                let active = self.players.active.unwrap();
                &mut self.players[(active + 1) % 2]
            }
        }
    }

    pub fn execute_turn(&mut self) {
        match self.state {
            GameState::TurnStart => {
                self.log.push(Vec::new());
                self.input_rand_ai();
                self.init_turn_order();
                self.state = GameState::MidTurn;
                self.apply_effects(self.calculate_effects());
            }
            GameState::MidTurn => {
                self.state = GameState::TurnEnd;
                self.apply_effects(self.calculate_effects());
            }
            GameState::TurnEnd => {
                self.state = GameState::TurnStart;
                self.order_turn_by_speed();
                for _ in 0..self.players.data.len() {
                    if let Some(active_mon) = self.player(&PlayerId::Active).roster.active() {
                        let (sand, ice) = (
                            (active_mon.poketype.contains(Type::Rock)
                                | active_mon.poketype.contains(Type::Steel)),
                            active_mon.poketype.contains(Type::Ice),
                        );
                        match (self.weather, sand, ice) {
                            (Some(WeatherId::Sand), false, _) => {
                                self.apply_effects(vec![Effect::Damage(
                                    PlayerId::Active,
                                    Damage::Fractional(1, 12),
                                )])
                            }
                            (Some(WeatherId::Hail), _, true) => {
                                self.apply_effects(vec![Effect::Damage(
                                    PlayerId::Active,
                                    Damage::Fractional(1, 12),
                                )])
                            }
                            _ => {}
                        }
                    }
                    self.invert_active_player();
                }

                match (
                    self.player(&PlayerId::Active).roster.active,
                    self.player(&PlayerId::Inactive).roster.active,
                ) {
                    (None, _) => {
                        self.prev_prev_state = self.prev_state;
                        self.prev_state = self.state;
                        self.state = GameState::AwaitingSwitch;
                        self.players.active = Some(0);
                    }
                    (_, None) => {
                        self.prev_prev_state = self.prev_state;
                        self.prev_state = self.state;
                        self.state = GameState::AwaitingSwitch;
                        self.players.active = Some(1);
                    }
                    _ => {}
                }
            }
            GameState::AwaitingSwitch => {
                self.apply_effects(self.calculate_effects());
                self.state = self.prev_state;
                self.prev_state = self.prev_prev_state;
            }
            GameState::Completed(_) => {
                return;
            }
        }

        let winner = self.check_winner();
        if winner != GameResult::Incomplete {
            self.state = GameState::Completed(winner);
            self.log(String::from("game finished"));
        }

        match self.state {
            GameState::AwaitingSwitch => {
                if let Some(1) = self.players.active {
                    self.input_rand_ai();
                    self.execute_turn();
                }
            }
            GameState::TurnStart => {}
            _ => {
                self.invert_active_player();
                self.execute_turn();
            }
        }
    }

    pub fn input_rand_ai(&mut self) {
        let mut rng = thread_rng();
        let choices = self.list_valid_inputs(&PlayerId::Player2);
        self.player_mut(&PlayerId::Player2)
            .inputs
            .push(*choices.choose(&mut rng).unwrap());
    }

    fn check_winner(&self) -> GameResult {
        match (
            self.list_valid_inputs(&PlayerId::Player1).len(),
            self.list_valid_inputs(&PlayerId::Player2).len(),
        ) {
            (0, 0) => GameResult::Tie,
            (_, 0) => GameResult::Winner(0),
            (0, _) => GameResult::Winner(1),
            _ => GameResult::Incomplete,
        }
    }

    pub fn list_valid_inputs(&self, player: &PlayerId) -> Vec<MoveSelection> {
        let mut out = Vec::new();
        let player = self.player(player);
        let active_idx = player.roster.active;
        for i in 0..player.roster.living().len() {
            if Some(i) != active_idx {
                out.push(MoveSelection::Switch(i));
            }
        }
        if let Some(active) = player.roster.active() {
            for i in 0..active.moves.living().len() {
                out.push(MoveSelection::Move(i));
            }
        }
        out
    }

    fn calculate_effects(&self) -> Vec<Effect> {
        let active_mon = if let Some(mon) = self.player(&PlayerId::Active).roster.active() {
            mon
        } else {
            return Vec::new();
        };

        match self.player(&PlayerId::Active).inputs.last().unwrap() {
            MoveSelection::Switch(idx) => [
                vec![Effect::Switch(*idx)],
                self.calc_hazards(&PlayerId::Active),
            ]
            .concat(),
            MoveSelection::Move(idx) => {
                let _move = &active_mon.moves[*idx];
                let mut out = Vec::new();
                if let (Some(bp), Some(inactive_mon)) = (
                    _move.base_power,
                    self.player(&PlayerId::Inactive).roster.active(),
                ) {
                    let (atk, def, burn, weather, random, stab, eff) = (
                        active_mon.stats[StatId::Atk].curr as f32,
                        inactive_mon.stats[StatId::Def].curr as f32,
                        if active_mon.status.contains_key(&Status::Burn) {
                            0.5f32
                        } else {
                            1.0f32
                        },
                        1.0f32,
                        thread_rng().gen_range(85..101) as f32 / 100.0,
                        if active_mon.poketype.contains(_move.poke_type) {
                            1.5f32
                        } else {
                            1.0f32
                        },
                        _move.poke_type.calc_eff(&inactive_mon.poketype),
                    );
                    out.push(Effect::Damage(
                        PlayerId::Inactive,
                        Damage::Normal(
                            ((((42.0f32 * bp as f32 * atk / def) / 50.0f32) * burn * weather)
                                * stab
                                * eff
                                * random) as i32,
                        ),
                    ))
                }
                match _move.freq {
                    Some(data) => {
                        if thread_rng().gen::<f32>() > data {
                            out.append(&mut _move.effects.clone());
                        }
                    }
                    None => out.append(&mut _move.effects.clone()),
                }
                out
            }
        }
    }

    fn calc_hazards(&self, target: &PlayerId) -> Vec<Effect> {
        let mut out = Vec::new();
        match self.player(target).hazards.stealth_rock.data {
            1 => out.push(Effect::Damage(*target, Damage::Fractional(1, 8))),
            _ => {}
        }
        match self.player(target).hazards.toxic_spikes.data {
            1 => out.push(Effect::InflictStatus(*target, Status::Poison)),
            2 => out.push(Effect::InflictStatus(*target, Status::Toxic)),
            _ => {}
        }
        match self.player(target).hazards.spikes.data {
            1 => out.push(Effect::Damage(*target, Damage::Fractional(1, 8))),
            2 => out.push(Effect::Damage(*target, Damage::Fractional(1, 6))),
            3 => out.push(Effect::Damage(*target, Damage::Fractional(1, 4))),
            _ => {}
        }
        out
    }

    pub fn active_mons_mut(&mut self) -> (Option<&mut Pokemon>, Option<&mut Pokemon>) {
        let players = match self.players.active {
            Some(0) => self.players.data.split_first_mut().unwrap(),
            Some(1) => self.players.data.split_last_mut().unwrap(),
            _ => panic!("no active player"),
        };
        (
            players.0.roster.active_mut(),
            players.1[0].roster.active_mut(),
        )
    }

    fn init_turn_order(&mut self) {
        let active;
        if let (p1, Some(p1_mon), p2, Some(p2_mon)) = (
            self.player(&PlayerId::Player1),
            self.player(&PlayerId::Player1).roster.active(),
            self.player(&PlayerId::Player2),
            self.player(&PlayerId::Player2).roster.active(),
        ) {
            let (input1, input2) = (
                p1.inputs.last().expect("P1 no input"),
                p2.inputs.last().expect("P2 no input"),
            );

            let (move1, move2) = (p1_mon.get_move(input1), p2_mon.get_move(input2));

            let faster = (p1_mon.stats[StatId::Spe].curr > p2_mon.stats[StatId::Spe].curr)
                | (p1_mon.stats[StatId::Spe].curr == p2_mon.stats[StatId::Spe].curr
                    && thread_rng().gen::<bool>());

            let (priority1, priority2) = (
                match move1 {
                    None => 5,
                    Some(sel_move) => sel_move.priority,
                },
                match move2 {
                    None => 5,
                    Some(sel_move) => sel_move.priority,
                },
            );

            active = (priority1 > priority2) | (priority1 == priority2 && faster);
        } else {
            panic!("Turn order initialised with inactive mons or players");
        }
        self.players.active = if active { Some(0) } else { Some(1) };
    }

    fn order_turn_by_speed(&mut self) {
        self.players.active = if let (Some(p1_mon), Some(p2_mon)) = (
            self.player(&PlayerId::Active).roster.active(),
            self.player(&PlayerId::Inactive).roster.active(),
        ) {
            if (p1_mon.stats[StatId::Spe].curr > p2_mon.stats[StatId::Spe].curr)
                | ((p1_mon.stats[StatId::Spe].curr == p2_mon.stats[StatId::Spe].curr)
                    && thread_rng().gen())
            {
                Some(0)
            } else {
                Some(1)
            }
        } else {
            Some(0)
        }
    }

    fn invert_active_player(&mut self) {
        let active = self.players.active.as_mut().unwrap();
        *active = (*active + 1) % 2;
    }
}
