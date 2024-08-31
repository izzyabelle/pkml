use crate::bounded_i32::BoundedI32;
use crate::game::{HazardId, WeatherId};
use crate::stat::StatId;
use crate::status::Status;
use crate::EmptyResult;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum PlayerId {
    Player1,
    Player2,
    Active,
    Inactive,
}

impl PlayerId {
    pub fn invert(&mut self) -> EmptyResult {
        match self {
            PlayerId::Player1 => *self = PlayerId::Player2,
            PlayerId::Player2 => *self = PlayerId::Player1,
            PlayerId::Active => *self = PlayerId::Inactive,
            PlayerId::Inactive => *self = PlayerId::Active,
        }
        Ok(())
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
    AlterStat(PlayerId, StatId, i8),
    ClearHazard(PlayerId),
    Cure(PlayerId),
    Damage(PlayerId, Damage),
    Heal(PlayerId),
    InflictHazard(PlayerId, HazardId),
    InflictStatus(PlayerId, Status),
    MidSwitch(PlayerId),
    MissTurn(PlayerId),
    OHKO(PlayerId),
    SetWeather(WeatherId),
    Switch(PlayerId, usize),
}

impl Damage {
    pub fn collapse(&self, value: BoundedI32) -> i32 {
        match self {
            Self::Normal(out) => *out,
            Self::Fractional(n, d) => value.max * n / d,
        }
    }
}

/*
impl Game {
    fn apply_effects(&mut self, effects: Vec<Effect>) -> EmptyResult {
        for effect in effects {
            match effect {
                Effect::InflictStatus(target, status) => {
                    if self.active_mut(&target).add_status(&mut status) {
                        self.log
                            .last_mut()
                            .expect("write to empty log")
                            .push_str(&format!(
                                "{}'s {} was {}",
                                self.player(&target).name(),
                                self.active(&target).name(),
                                status.to_string(),
                            ));
                    }
                }

                Effect::AlterStat(target, stat, stat_mod) => {
                    let mod_str = if stat_mod > 0 { "raised" } else { "lowered" };
                    if self.active_mut(&target).stats[stat.index()].alter(stat_mod) {
                        self.log
                            .last_mut()
                            .expect("attempted to write to empty game log vec")
                            .push_str(&format!(
                                "{}'s {} had its {} {}",
                                self.player(&target).name(),
                                self.active(&target).name(),
                                stat.to_string(),
                                mod_str,
                            ));
                    }
                }
                Effect::InflictHazard(target, hazard) => {
                    self.player_mut(&target).inc_hazard(hazard).unwrap();

                    self.log
                        .last_mut()
                        .expect("attempted to write to empty game log vec")
                        .push_str(&format!(
                            "{} was placed on {}'s field",
                            hazard.to_string(),
                            self.player(&target).name(),
                        ));
                }
                Effect::ClearHazard(target) => {
                    self.player_mut(&target).clear_hazards().unwrap();

                    self.log
                        .last_mut()
                        .expect("attempted to write to empty game log vec")
                        .push_str(&format!(
                            "hazards were cleared from {}'s field",
                            self.player(&target).name(),
                        ));
                }
                Effect::Damage(target, damage) => {
                    let result = self.active_mut(&target).hp.damage(damage);
                    let dam_str = if result.0 > 0 { "gained" } else { "lost" };
                    if result.0 != 0 {
                        self.log
                            .last_mut()
                            .expect("attempted to write to empty game log vec")
                            .push_str(&format!(
                                "{}'s {} {} {} hp{}",
                                self.player(&target).name(),
                                self.active(&target).name(),
                                dam_str,
                                self.active(&target).hp.percent(&result.0.abs()),
                                if result.1 == 0 {
                                    self.player_mut(&&target).remaining_mons -= 1;
                                    ", it died"
                                } else {
                                    ""
                                }
                            ));
                    }
                }
                Effect::Cure(target) => {
                    if self.active_mut(&target).refresh_status() {
                        self.log
                            .last_mut()
                            .expect("attempted to write to empty game log vec")
                            .push_str(&format!(
                                "{}'s {} was cured of status",
                                self.player(&target).name(),
                                self.active(&target).name(),
                            ));
                    }
                }
                Effect::Heal(target) => {
                    let result = self
                        .active_mut(&target)
                        .hp
                        .damage(Damage::Fractional(-1, 2));

                    self.log
                        .last_mut()
                        .expect("attempted to write to empty game log vec")
                        .push_str(&format!(
                            "{}'s {} gained {} hp",
                            self.player(&target).name(),
                            self.active(&target).name(),
                            self.active(&target).hp.percent(&result.0),
                        ));
                }
                Effect::Switch(target, target_mon) => {
                    self.player_mut(&target).release_lock();
                    self.active_mut(&target).reset_nv();

                    self.player_mut(&target)
                        .set_active_index(target_mon)
                        .unwrap();

                    self.log
                        .last_mut()
                        .expect("attempted to write to empty game log vec")
                        .push_str(&format!(
                            "{} switched to {}",
                            self.player(&target).name(),
                            self.active(&target).name(),
                        ));
                }
                Effect::OHKO(target) => {
                    if self.active_mut(&target).faint() {
                        self.log
                            .last_mut()
                            .expect("attempted to write to empty game log vec")
                            .push_str(&format!("{} fainted!", self.active(&target).name()));
                        self.player_mut(&target).remaining_mons -= 1;
                    }
                }
                Effect::MissTurn(target) => {
                    if self.active_mut(&target).miss_turn() {
                        self.log
                            .last_mut()
                            .expect("attempted to write to empty game log vec")
                            .push_str(&format!(
                                "{}'s {} missed its turn",
                                self.player(&target).name(),
                                self.active(&target).name(),
                            ));
                    }
                }
                Effect::MidSwitch(target) => {
                    if self.player(&target).remaining_mons == 1 {
                        continue;
                    }
                    // self.get_input().unwrap();
                    // let effects_ = self.process_move();
                    self.apply_effects(effects).unwrap();
                }
                Effect::SetWeather(weather) => {
                    if self.set_weather(weather) {
                        match weather {
                            WeatherId::Sand => {
                                self.log
                                    .last_mut()
                                    .expect("attempted to write to empty game log vec")
                                    .push_str(&format!("A sandstorm kicked up!"));
                            }
                            WeatherId::Hail => {
                                self.log
                                    .last_mut()
                                    .expect("attempted to write to empty game log vec")
                                    .push_str(&format!("Hail starts"));
                            }
                            WeatherId::Rain => {
                                self.log
                                    .last_mut()
                                    .expect("attempted to write to empty game log vec")
                                    .push_str(&format!("Rain starts"));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
*/
