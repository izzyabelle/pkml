use crate::bounded_i32::BoundedI32;
use crate::player::HazardBlock;
use crate::poketype::Type;
use crate::status::Status;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::{player::Player, pokemon::Pokemon, selvec::PointerVec, stat::StatId};

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
                self.execute_move();
            }
            GameState::MidTurn => {
                self.state = GameState::TurnEnd;
                if self.player(&PlayerId::Active).has_active() {
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

                if self.player(&PlayerId::Player1).roster.active().is_none() {
                    self.prev_state.push(self.state);
                    self.state = GameState::AwaitingSwitch;
                    self.players.active = Some(0);
                }
                if self.player(&PlayerId::Player2).roster.active().is_none() {
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

    fn apply_eot_effects(&mut self) {
        let mut effects = Vec::new();
        if let Some(active_mon) = self.player(&PlayerId::Active).roster.active() {
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
        let player_obj = self.player(player);
        let active_idx = player_obj.roster.active;
        for i in 0..player_obj.roster.living().len() {
            if Some(i) != active_idx {
                out.push(MoveSelection::Switch(i));
            }
        }
        if let Some(active) = player_obj.roster.active() {
            if self.state != GameState::AwaitingSwitch || self.abs_active_player() != *player {
                for i in 0..active.moves.living().len() {
                    out.push(MoveSelection::Move(i));
                }
            }
        }
        out
    }

    fn abs_active_player(&self) -> PlayerId {
        if self.players.active.unwrap() == 0 {
            PlayerId::Player1
        } else {
            PlayerId::Player2
        }
    }

    fn execute_move(&mut self) {
        if let MoveSelection::Switch(_) = self.player(&PlayerId::Active).inputs.last().unwrap() {
            let effects = self.calculate_input_effects();
            self.log(effects.1);
            self.apply_effects(effects.0);
            self.log(String::new());
            return;
        }

        let mut rand = thread_rng();
        let active_mon = self.player(&PlayerId::Active).roster.active().unwrap();
        let mut log_messages = Vec::new();
        let mut miss_turn = false;
        let mut removed_statuses = Vec::new();

        for status in &[
            Status::Paralyse,
            Status::Sleep,
            Status::Freeze,
            Status::Confusion,
            Status::Flinch,
        ] {
            if let Some(value) = active_mon
                .status
                .try_borrow_mut()
                .expect("refcell error")
                .data
                .get_mut(status)
            {
                match status {
                    Status::Paralyse => {
                        if rand.gen_range(0..=3) == 0 {
                            log_messages.push(format!("{} was full para", active_mon.id));
                            miss_turn = true;
                            break;
                        }
                    }
                    Status::Sleep => {
                        if *value == 0 {
                            log_messages.push(format!("{} woke up!", active_mon.id));
                            removed_statuses.push(status);
                        } else {
                            *value -= 1;
                            log_messages.push(format!("{} was sleeping", active_mon.id));
                            miss_turn = true;
                            break;
                        }
                    }
                    Status::Freeze => {
                        if rand.gen_range(0..=9) == 0 {
                            log_messages.push(format!("{} thawed!", active_mon.id));
                            removed_statuses.push(status);
                        } else {
                            log_messages.push(format!("{} is frozen", active_mon.id));
                            miss_turn = true;
                            break;
                        }
                    }
                    Status::Confusion => {
                        if *value == 0 {
                            log_messages
                                .push(format!("{} snapped out of confusion", active_mon.id));
                            removed_statuses.push(status);
                        } else if rand.gen_range(0..=1) == 0 {
                            log_messages
                                .push(format!("{} hit itself in confusion(todo)", active_mon.id));
                            *value -= 1;
                            miss_turn = true;
                            break;
                        } else {
                            *value -= 1;
                        }
                    }
                    Status::Flinch => {
                        log_messages.push(format!("{} flinched", active_mon.id));
                        removed_statuses.push(status);
                        miss_turn = true;
                        break;
                    }
                    _ => {}
                }
            }
        }

        for status in &removed_statuses {
            active_mon
                .status
                .try_borrow_mut()
                .expect("refcell error")
                .data
                .remove(status);
        }

        for message in log_messages {
            self.log(message);
        }

        if miss_turn {
            return;
        }

        let effects = self.calculate_input_effects();
        self.log(effects.1);
        self.apply_effects(effects.0);
        self.log(String::new());
    }

    fn calculate_input_effects(&self) -> (Vec<Effect>, String) {
        let player = self.player(&PlayerId::Active);
        match player.inputs.last().unwrap() {
            MoveSelection::Switch(idx) => (
                [
                    vec![Effect::Switch(*idx)],
                    self.calc_hazards(&PlayerId::Active),
                ]
                .concat(),
                if let Some(mon) = player.roster.active() {
                    format!("{} withdraws {}", player, mon.id)
                } else {
                    format!("{} selects new mon", player)
                },
            ),
            MoveSelection::Move(idx) => {
                let player = self.player(&PlayerId::Active);
                let active_mon = player
                    .roster
                    .active()
                    .expect("Move used with no active mon");
                let selected_move = &active_mon.moves[*idx];
                let mut out = Vec::new();
                if let (Some(bp), Some(inactive_mon)) = (
                    selected_move.base_power,
                    self.player(&PlayerId::Inactive).roster.active(),
                ) {
                    let (atk, def, burn, weather, random, stab, eff) = (
                        active_mon.stats[StatId::Atk].curr() as f32,
                        inactive_mon.stats[StatId::Def].curr() as f32,
                        if active_mon.status.borrow().data.get(&Status::Burn).is_some() {
                            0.5f32
                        } else {
                            1.0f32
                        },
                        1.0f32,
                        thread_rng().gen_range(85..101) as f32 / 100.0,
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
                            ((((42.0f32 * bp as f32 * atk / def) / 50.0f32) * burn * weather)
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
                (
                    out,
                    format!("{}'s {} used {}", player, active_mon.id, selected_move.id),
                )
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
            self.player(&PlayerId::Active).roster.active(),
            self.player(&PlayerId::Inactive).roster.active(),
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
        let active = self.players.active.as_mut().unwrap();
        *active = (*active + 1) % 2;
    }

    fn apply_effects(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::InflictStatus(target, status) => {
                    if let Some(mon) = self.player_mut(&target).roster.active_mut() {
                        mon.status
                            .try_borrow_mut()
                            .expect("refcell error")
                            .add(&status);
                        let mon_name = format!("{}", mon.id);
                        self.log(format!(
                            "{}'s {} was {}",
                            self.player(&target),
                            mon_name,
                            status
                        ));
                    }
                }

                Effect::AlterStat(target, stat, stat_mod) => {
                    if let Some(target_mon) = self.player_mut(&target).roster.active_mut() {
                        let mod_str = if stat_mod > 0 { "raised" } else { "lowered" };
                        if target_mon.stats[stat].alter(stat_mod) {
                            let mon_name = format!("{}", target_mon.id);
                            self.log(format!(
                                "{}'s {} {} was {}",
                                self.player(&target),
                                mon_name,
                                stat,
                                mod_str,
                            ));
                        }
                    }
                }
                Effect::InflictHazard(target, hazard) => {
                    let target_player = self.player_mut(&target);

                    if target_player.hazards[hazard].is_max() {
                        continue;
                    }

                    target_player.hazards[hazard] += 1;

                    let (player_name, hazard_name) = (
                        format!("{}", target_player),
                        format!("{}", target_player.hazards[hazard]),
                    );

                    self.log(format!(
                        "{} was placed on {}'s field",
                        hazard_name, player_name,
                    ));
                }
                Effect::ClearHazard(target) => {
                    let target_player = self.player_mut(&target);
                    target_player.hazards = HazardBlock::default();
                    let player_name = format!("{}", target_player);

                    self.log(format!("hazards were cleared from {}'s field", player_name));
                }
                Effect::Damage(target, damage) => {
                    let rem_hp;
                    if let Some(target_mon) = self.player_mut(&target).roster.active_mut() {
                        let prev = target_mon.hp.data;
                        target_mon.hp -= damage.collapse(target_mon.hp);
                        let diff = prev - target_mon.hp.data;
                        let mon_name = format!("{}", target_mon.id);
                        rem_hp = target_mon.hp.data;

                        self.log(format!(
                            "{}'s {} lost {} hp",
                            self.player(&target),
                            mon_name,
                            diff,
                        ));
                    } else {
                        continue;
                    }

                    if rem_hp == 0 {
                        self.player_mut(&target).roster.kill();
                        self.log(String::from("They fainted"));
                    }
                }

                Effect::Cure(target) => {
                    if let Some(target_mon) = self.player_mut(&target).roster.active_mut() {
                        for status in &vec![Status::Burn, Status::Paralyse, Status::Toxic] {
                            target_mon
                                .status
                                .try_borrow_mut()
                                .expect("refcell error")
                                .clear_nv();
                        }
                        let mon_name = format!("{}", target_mon.id);

                        self.log(format!(
                            "{}'s {} was cured of status",
                            self.player(&target),
                            mon_name,
                        ));
                    }
                }
                Effect::Heal(target, frac) => {
                    if let Some(target_mon) = self.player_mut(&target).roster.active_mut() {
                        if target_mon.hp.data == 0 {
                            continue;
                        }
                        let prev = target_mon.hp.data;
                        target_mon.hp += target_mon.hp.max / frac;
                        let diff = target_mon.hp - prev;
                        let mon_name = format!("{}", target_mon.id);

                        self.log(format!(
                            "{}'s {} gained {} hp",
                            self.player(&target),
                            mon_name,
                            diff,
                        ));
                    }
                }
                Effect::OHKO(target) => {
                    let mon_name;
                    if let Some(target_mon) = self.player_mut(&target).roster.active_mut() {
                        target_mon.hp.data = 0;
                        mon_name = format!("{}", target_mon.id);
                    } else {
                        continue;
                    }
                    self.player_mut(&target).roster.kill();

                    self.log(format!("{}'s {} fainted!", self.player(&target), mon_name));
                }
                Effect::MidSwitch(target) => {
                    let target_player = self.player_mut(&target);
                    if target_player.roster.dead == 1 {
                        continue;
                    }
                    self.prev_state.push(self.state);
                    self.state = GameState::AwaitingSwitch;
                    self.log(format!(
                        "{} selects a pokemon to switch to",
                        self.player(&target)
                    ));
                }
                Effect::SetWeather(weather) => {
                    if *self.weather.borrow() != Some(weather) {
                        *self
                            .weather
                            .try_borrow_mut()
                            .expect("Weather refcell error") = Some(weather);
                        match weather {
                            WeatherId::Sand => {
                                self.log
                                    .last_mut()
                                    .expect("attempted to write to empty game log vec")
                                    .push(format!("A sandstorm kicked up!"));
                            }
                            WeatherId::Hail => {
                                self.log
                                    .last_mut()
                                    .expect("attempted to write to empty game log vec")
                                    .push(format!("Hail starts"));
                            }
                            WeatherId::Rain => {
                                self.log
                                    .last_mut()
                                    .expect("attempted to write to empty game log vec")
                                    .push(format!("Rain starts"));
                            }
                        }
                    }
                }
                Effect::Switch(idx) => {
                    self.player_mut(&PlayerId::Active).roster.active = Some(idx);
                    self.log(format!(
                        "{} sends out {}",
                        self.player(&PlayerId::Active),
                        self.player(&PlayerId::Active)
                            .roster
                            .active()
                            .expect("switch failed")
                            .id
                    ))
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
pub enum PlayerId {
    #[default]
    Player1,
    Player2,
    Active,
    Inactive,
}

impl PlayerId {
    pub fn invert(&mut self) {
        match self {
            PlayerId::Player1 => *self = PlayerId::Player2,
            PlayerId::Player2 => *self = PlayerId::Player1,
            PlayerId::Active => *self = PlayerId::Inactive,
            PlayerId::Inactive => *self = PlayerId::Active,
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
