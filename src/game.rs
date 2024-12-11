use crate::bounded_i32::BoundedI32;
use crate::moves::Mtype;
use crate::player::HazardBlock;
use crate::poketype::Type;
use crate::selvec::PlayerId;
use crate::status::Status;
use crate::trigger::Item;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::{player::Player, pokemon::Pokemon, selvec::PointerVec, stat::StatId};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HazardId {
    StealthRock,
    Spikes,
    ToxicSpikes,
}

impl Display for HazardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HazardId::StealthRock => String::from("Stealth Rock"),
                HazardId::Spikes => String::from("Spikes"),
                HazardId::ToxicSpikes => String::from("Toxic Spikes"),
            }
        )
    }
}

impl HazardId {
    pub fn max_layers(&self) -> u8 {
        match self {
            HazardId::StealthRock => 1,
            HazardId::Spikes => 2,
            HazardId::ToxicSpikes => 3,
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
    pub weather: Rc<RefCell<Option<WeatherId>>>,
    pub log: Vec<Vec<String>>,
    pub state: GameState,
    pub prev_state: Vec<GameState>,
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
                    GameResult::Incomplete => panic!("somethin wronggg"),
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

fn write_log(log: &mut [Vec<String>], message: String) {
    log.last_mut()
        .expect("attempted to write to empty log")
        .push(message);
}

impl Game {
    pub fn new() -> Self {
        let weather = Rc::new(RefCell::new(None));
        Self {
            players: PointerVec::from(vec![
                Player::new(false, Rc::clone(&weather)),
                Player::new(true, Rc::clone(&weather)),
            ]),
            log: vec![vec![String::from("Players sent out their starters!")]],
            weather,
            ..Default::default()
        }
    }

    pub fn execute_turn(&mut self) {
        match self.state {
            GameState::TurnStart => {
                self.log.push(Vec::new());
                self.input_rand_ai();
                self.init_turn_order();
                self.state = GameState::MidTurn;
                self.execute_move();
            }
            GameState::MidTurn => {
                self.state = GameState::TurnEnd;
                if self.players[PlayerId::Active].has_active() {
                    self.execute_move();
                }
            }
            GameState::TurnEnd => {
                self.state = GameState::TurnStart;
                self.order_turn_by_speed();
                for _ in 0..self.players.data.len() {
                    self.apply_eot_effects();
                    self.invert_active_player();
                }

                if self.players[0].roster.active().is_none() {
                    self.prev_state.push(self.state);
                    self.state = GameState::AwaitingSwitch;
                    self.players.active = Some(0);
                }
                if self.players[1].roster.active().is_none() {
                    self.prev_state.push(self.state);
                    self.state = GameState::AwaitingSwitch;
                    self.players.active = Some(1);
                }
            }
            GameState::AwaitingSwitch => {
                self.state = self
                    .prev_state
                    .pop()
                    .expect("gamestate reverted with no prev");
                self.execute_move();
                self.invert_active_player();
            }
            GameState::Completed(_) => {
                return;
            }
        }

        let winner = self.check_winner();
        if winner != GameResult::Incomplete {
            self.state = GameState::Completed(winner);
            write_log(&mut self.log, String::from("game finished"));
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

    fn apply_eot_effects(&mut self) {
        let mut effects = Vec::new();
        if let Some(active_mon) = self.players[PlayerId::Active].roster.active() {
            // weather effects
            let (weather, sand, ice) = (
                *self.weather.borrow(),
                (active_mon.poketype.borrow().contains(Type::Rock)
                    | active_mon.poketype.borrow().contains(Type::Steel)),
                active_mon.poketype.borrow().contains(Type::Ice),
            );
            match (weather, sand, ice) {
                (Some(WeatherId::Sand), false, _) => effects.push(vec![Effect::Damage(
                    PlayerId::Active,
                    Damage::Fractional(1, 12),
                )]),
                (Some(WeatherId::Hail), _, true) => effects.push(vec![Effect::Damage(
                    PlayerId::Active,
                    Damage::Fractional(1, 12),
                )]),
                _ => {}
            }

            // eot item triggers
            if let Some(item) = *active_mon.item.borrow() {
                match item {
                    Item::Leftovers => effects.push(vec![Effect::Heal(PlayerId::Active, 16)]),
                    Item::ToxicOrb => {
                        effects.push(vec![Effect::InflictStatus(PlayerId::Active, Status::Toxic)])
                    }
                    _ => {}
                }
            }

            // increment counters
            for status in &[Status::Toxic, Status::Drowsy, Status::Confusion] {
                active_mon
                    .status
                    .try_borrow_mut()
                    .expect("refcell error")
                    .increment(status);
            }

            active_mon
                .status
                .try_borrow_mut()
                .expect("refcell error")
                .data
                .remove(&Status::Flinch);

            // calc status effects
            for status in &[Status::Burn, Status::Poison, Status::Toxic, Status::Drowsy] {
                if let Some(value) = active_mon.status.borrow().data.get(status) {
                    match status {
                        Status::Burn | Status::Poison => {
                            effects.push(vec![Effect::Damage(
                                PlayerId::Active,
                                Damage::Fractional(1, 8),
                            )]);
                        }
                        Status::Toxic => {
                            effects.push(vec![Effect::Damage(
                                PlayerId::Active,
                                Damage::Fractional(*value as i32, 16),
                            )]);
                        }
                        Status::Drowsy if *value == 0 => {
                            effects
                                .push(vec![Effect::InflictStatus(PlayerId::Active, Status::Sleep)]);
                            active_mon
                                .status
                                .try_borrow_mut()
                                .expect("deref error")
                                .data
                                .remove(&Status::Drowsy);
                        }
                        _ => {}
                    }
                }
            }
        }
        self.apply_effects(effects.concat());
    }

    pub fn input_rand_ai(&mut self) {
        let mut rng = thread_rng();
        let choices = self.players[1].list_valid_inputs(&self.state);
        self.players[1]
            .inputs
            .push(*choices.choose(&mut rng).unwrap());
    }

    fn check_winner(&self) -> GameResult {
        // check if either player's entire roster is dead
        match (self.players[0].roster.dead, self.players[1].roster.dead) {
            (0, 0) => GameResult::Tie,
            (_, 0) => GameResult::Winner(0),
            (0, _) => GameResult::Winner(1),
            _ => GameResult::Incomplete,
        }
    }

    /// executes the last move input by the active player
    fn execute_move(&mut self) {
        let player = &self.players[PlayerId::Active];
        match *player
            .inputs
            .last()
            .expect("empty input vec upon executing move")
        {
            MoveSelection::Switch(idx) => {
                // write different log message depending on whether mon is being withdrawn
                write_log(
                    &mut self.log,
                    if let Some(mon) = player.roster.active() {
                        format!("{} withdraws {}", player, mon.id)
                    } else {
                        format!("{} selects new mon", player)
                    },
                );

                let mut effects = vec![Effect::Switch(idx)];
                effects.extend(self.calc_switch(PlayerId::Active));
                self.apply_effects(effects);
            }
            MoveSelection::Move(idx) => {
                // check if pokemon can move
                if let Some(message) = self.exec_moveskip() {
                    write_log(&mut self.log, message);
                    return;
                }

                let player = &self.players[PlayerId::Active];

                let active_mon = player
                    .roster
                    .active()
                    .expect("move used with no active mon");

                write_log(
                    &mut self.log,
                    format!(
                        "{}'s {} used {}",
                        player, active_mon.id, active_mon.moves[idx].id
                    ),
                );

                self.apply_effects(self.calc_move());
            }
        }
        write_log(&mut self.log, String::new());
    }

    /// if return Some move is skipped with message. moveskip statuses mutated
    fn exec_moveskip(&mut self) -> Option<String> {
        let mut rand = thread_rng();
        let active_mon = &mut self.players[PlayerId::Active]
            .roster
            .active()
            .expect("no active mon");
        let mut removed_statuses = Vec::new();
        let mut message = None;
        let mut statusblock = active_mon.status.try_borrow_mut().unwrap();
        for status in &[
            Status::Paralyse,
            Status::Sleep,
            Status::Freeze,
            Status::Confusion,
            Status::Flinch,
        ] {
            if let Some(value) = statusblock.data.get_mut(status) {
                match status {
                    Status::Paralyse => {
                        if rand.gen_range(0..=3) == 0 {
                            message = Some(format!("{} was full para", active_mon.id));
                        }
                    }
                    Status::Sleep => {
                        if *value == 0 {
                            write_log(&mut self.log, format!("{} woke up!", active_mon.id));
                            removed_statuses.push(status);
                        } else {
                            *value -= 1;
                            message = Some(format!("{} was sleeping", active_mon.id));
                        }
                    }
                    Status::Freeze => {
                        if rand.gen_range(0..=9) == 0 {
                            write_log(&mut self.log, format!("{} thawed!", active_mon.id));
                            removed_statuses.push(status);
                        } else {
                            message = Some(format!("{} is frozen", active_mon.id));
                        }
                    }
                    Status::Confusion => {
                        if *value == 0 {
                            write_log(
                                &mut self.log,
                                format!("{} snapped out of confusion", active_mon.id),
                            );
                            removed_statuses.push(status);
                        } else if rand.gen_range(0..=1) == 0 {
                            message =
                                Some(format!("{} hit itself in confusion(todo)", active_mon.id));
                            *value -= 1;
                        } else {
                            *value -= 1;
                        }
                    }
                    Status::Flinch => {
                        message = Some(format!("{} flinched", active_mon.id));
                        removed_statuses.push(status);
                    }
                    _ => {}
                }
            }
        }
        if let Ok(mut data) = active_mon.status.try_borrow_mut() {
            for status in removed_statuses {
                data.data.remove(status);
            }
        }
        message
    }

    fn calc_move(&self) -> Vec<Effect> {
        let active_player = &self.players[PlayerId::Active];
        let active_mon = &active_player
            .roster
            .active()
            .expect("move used with no active mon");
        let selected_move =
            if let MoveSelection::Move(idx) = active_player.inputs.last().expect("no inputs") {
                &active_mon.moves[*idx]
            } else {
                panic!("move calculatd with switch input");
            };

        let mut out = Vec::new();
        if let (Some(bp), Some(inactive_mon)) = (
            selected_move.base_power,
            self.players[PlayerId::Inactive].roster.active(),
        ) {
            let (atk, def, burn, weather, random, stab, eff) = (
                if selected_move.damage_type == Mtype::Physical {
                    active_mon.stats[StatId::Atk].curr() as f32
                } else {
                    active_mon.stats[StatId::Spa].curr() as f32
                },
                if selected_move.damage_type == Mtype::Physical {
                    inactive_mon.stats[StatId::Def].curr() as f32
                } else {
                    inactive_mon.stats[StatId::Spd].curr() as f32
                },
                if active_mon.status.borrow().data.contains_key(&Status::Burn) {
                    0.5f32
                } else {
                    1.0f32
                },
                1.0f32,
                thread_rng().gen_range(85..=100) as f32 / 100.0,
                if active_mon
                    .poketype
                    .borrow()
                    .contains(selected_move.poke_type)
                {
                    1.5f32
                } else {
                    1.0f32
                },
                selected_move
                    .poke_type
                    .calc_eff(&inactive_mon.poketype.borrow()),
            );
            out.push(Effect::Damage(
                PlayerId::Inactive,
                Damage::Normal(
                    (((42.0f32 * bp as f32 * (atk / def)) / 50.0f32 * burn * weather)
                        * stab
                        * eff
                        * random) as i32,
                ),
            ))
        }
        match selected_move.freq {
            Some(data) => {
                if thread_rng().gen::<f32>() > data {
                    out.append(&mut selected_move.effects.clone());
                }
            }
            None => out.append(&mut selected_move.effects.clone()),
        }
        out
    }

    fn calc_switch(&self, target: PlayerId) -> Vec<Effect> {
        let mut out = Vec::new();
        if self.players[target].hazards.stealth_rock.data == 1 {
            out.push(Effect::Damage(target, Damage::Fractional(1, 8)))
        }
        match self.players[target].hazards.toxic_spikes.data {
            1 => out.push(Effect::InflictStatus(target, Status::Poison)),
            2 => out.push(Effect::InflictStatus(target, Status::Toxic)),
            _ => {}
        }
        match self.players[target].hazards.spikes.data {
            1 => out.push(Effect::Damage(target, Damage::Fractional(1, 8))),
            2 => out.push(Effect::Damage(target, Damage::Fractional(1, 6))),
            3 => out.push(Effect::Damage(target, Damage::Fractional(1, 4))),
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
            &self.players[0],
            self.players[0].roster.active(),
            &self.players[1],
            self.players[1].roster.active(),
        ) {
            let (input1, input2) = (
                p1.inputs.last().expect("P1 no input"),
                p2.inputs.last().expect("P2 no input"),
            );

            let (move1, move2) = (p1_mon.get_move(input1), p2_mon.get_move(input2));

            let faster = (p1_mon.stats[StatId::Spe].curr() > p2_mon.stats[StatId::Spe].curr())
                | (p1_mon.stats[StatId::Spe].curr() == p2_mon.stats[StatId::Spe].curr()
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
            self.players[PlayerId::Active].roster.active(),
            self.players[PlayerId::Inactive].roster.active(),
        ) {
            if (p1_mon.stats[StatId::Spe].curr() > p2_mon.stats[StatId::Spe].curr())
                | ((p1_mon.stats[StatId::Spe].curr() == p2_mon.stats[StatId::Spe].curr())
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
        let active = self.players.active.as_mut().expect("no active player");
        *active = (*active + 1) % 2;
    }

    fn apply_effects(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::InflictStatus(target, status) => {
                    if let Some(mon) = self.players[target].roster.active_mut() {
                        let mut success = false;
                        if let Ok(mut status_ref) = mon.status.try_borrow_mut() {
                            success = status_ref.add(&status);
                        }

                        if success {
                            let mon_name = mon.id.to_string();
                            write_log(
                                &mut self.log,
                                format!("{}'s {} was {}", self.players[target], mon_name, status),
                            );
                        }
                    }
                }

                Effect::AlterStat(target, stat, stat_mod) => {
                    if let Some(target_mon) = self.players[target].roster.active_mut() {
                        let mod_str = if stat_mod > 0 { "raised" } else { "lowered" };
                        if target_mon.stats[stat].alter(stat_mod) {
                            let mon_name = target_mon.id.to_string();
                            write_log(
                                &mut self.log,
                                format!(
                                    "{}'s {} {} was {}",
                                    self.players[target], mon_name, stat, mod_str,
                                ),
                            );
                        }
                    }
                }

                Effect::InflictHazard(target, hazard) => {
                    let target_player = &mut self.players[target];
                    if !target_player.hazards[hazard].is_max() {
                        target_player.hazards[hazard] += 1;
                        let player_name = target_player.to_string();
                        let hazard_name = target_player.hazards[hazard].to_string();
                        write_log(
                            &mut self.log,
                            format!("{} was placed on {}'s field", hazard_name, player_name),
                        );
                    }
                }

                Effect::ClearHazard(target) => {
                    let target_player = &mut self.players[target];
                    target_player.hazards = HazardBlock::default();
                    let player_name = target_player.to_string();
                    write_log(
                        &mut self.log,
                        format!("hazards were cleared from {}'s field", player_name),
                    );
                }

                Effect::Damage(target, damage) => {
                    if let Some(target_mon) = self.players[target].roster.active_mut() {
                        let prev_hp = target_mon.hp.data;
                        target_mon.hp -= damage.collapse(target_mon.hp);
                        let diff = prev_hp - target_mon.hp.data;
                        let mon_name = target_mon.id.to_string();
                        let rem_hp = target_mon.hp.data;

                        write_log(
                            &mut self.log,
                            format!("{}'s {} lost {} hp", self.players[target], mon_name, diff),
                        );

                        if rem_hp == 0 {
                            self.players[target].roster.kill();
                            write_log(&mut self.log, String::from("They fainted"));
                        }
                    }
                }

                Effect::Cure(target) => {
                    if let Some(target_mon) = self.players[target].roster.active_mut() {
                        if let Ok(mut status_ref) = target_mon.status.try_borrow_mut() {
                            status_ref.clear_nv();
                        }
                        let mon_name = target_mon.id.to_string();
                        write_log(
                            &mut self.log,
                            format!(
                                "{}'s {} was cured of status",
                                self.players[target], mon_name
                            ),
                        );
                    }
                }

                Effect::Heal(target, frac) => {
                    if let Some(target_mon) = self.players[target].roster.active_mut() {
                        if target_mon.hp.data > 0 {
                            let prev_hp = target_mon.hp.data;
                            target_mon.hp += target_mon.hp.max / frac;
                            let diff = target_mon.hp.data - prev_hp;
                            let mon_name = target_mon.id.to_string();
                            write_log(
                                &mut self.log,
                                format!(
                                    "{}'s {} gained {} hp",
                                    self.players[target], mon_name, diff
                                ),
                            );
                        }
                    }
                }

                Effect::OHKO(target) => {
                    if let Some(target_mon) = self.players[target].roster.active_mut() {
                        target_mon.hp.data = 0;
                        let mon_name = target_mon.id.to_string();
                        self.players[target].roster.kill();
                        write_log(
                            &mut self.log,
                            format!("{}'s {} fainted!", self.players[target], mon_name),
                        );
                    }
                }

                Effect::MidSwitch(target) => {
                    let target_player = &mut self.players[target];
                    if target_player.roster.dead != 1 {
                        self.prev_state.push(self.state);
                        self.state = GameState::AwaitingSwitch;
                        write_log(
                            &mut self.log,
                            format!("{} selects a pokemon to switch to", self.players[target]),
                        );
                    }
                }

                Effect::SetWeather(weather) => {
                    if *self.weather.borrow() != Some(weather) {
                        if let Ok(mut weather_ref) = self.weather.try_borrow_mut() {
                            *weather_ref = Some(weather);
                            let weather_message = match weather {
                                WeatherId::Sand => "A sandstorm kicked up!",
                                WeatherId::Hail => "Hail starts",
                                WeatherId::Rain => "Rain starts",
                            };
                            write_log(&mut self.log, weather_message.to_string());
                        }
                    }
                }

                Effect::Switch(idx) => {
                    self.players[PlayerId::Active].roster.active = Some(idx);
                    let active_player = &self.players[PlayerId::Active];
                    let mon_name = &active_player.roster[idx].id;
                    write_log(
                        &mut self.log,
                        format!("{} sends out {}", active_player, mon_name),
                    );
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Damage {
    Normal(i32),
    // fractional damage represented as a fraction, value 1 over value 2
    Fractional(i32, i32),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Effect {
    AlterStat(PlayerId, StatId, i32),
    ClearHazard(PlayerId),
    Cure(PlayerId),
    Damage(PlayerId, Damage),
    Heal(PlayerId, i32),
    InflictHazard(PlayerId, HazardId),
    InflictStatus(PlayerId, Status),
    MidSwitch(PlayerId),
    OHKO(PlayerId),
    SetWeather(WeatherId),
    Switch(usize),
}

impl Damage {
    pub fn collapse(&self, value: BoundedI32) -> i32 {
        match self {
            Self::Normal(out) => *out,
            Self::Fractional(n, d) => value.max * n / d,
        }
    }
}
