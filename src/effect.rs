use std::fmt::{Debug, Formatter};

use crate::bounded_i32::BoundedI32;
use crate::game::{Game, HazardId, WeatherId};
use crate::player::HazardBlock;
use crate::stat::StatId;
use crate::status::Status;
use crate::EmptyResult;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
pub enum PlayerId {
    #[default]
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
}

impl Damage {
    pub fn collapse(&self, value: BoundedI32) -> i32 {
        match self {
            Self::Normal(out) => *out,
            Self::Fractional(n, d) => value.max * n / d,
        }
    }
}

impl Game {
    fn apply_effects(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::InflictStatus(target, status) => {
                    if let Some(mon) = self.active_mut(&target) {
                        mon.add_status(status);
                        let mon_name = format!("{}", mon.id);
                        self.log(format!(
                            "{}'s {} was {}",
                            self.player(&target).unwrap(),
                            mon_name,
                            status
                        ));
                    }
                }

                Effect::AlterStat(target, stat, stat_mod) => {
                    if let Some(target_mon) = self.active_mut(&target) {
                        let mod_str = if stat_mod > 0 { "raised" } else { "lowered" };
                        if target_mon.stats[stat].alter(stat_mod) {
                            let mon_name = format!("{}", target_mon.id);
                            self.log(format!(
                                "{}'s {} {} was {}",
                                self.player(&target).unwrap(),
                                mon_name,
                                stat,
                                mod_str,
                            ));
                        }
                    }
                }
                Effect::InflictHazard(target, hazard) => {
                    let target_player = if let Some(data) = self.player_mut(&target) {
                        data
                    } else {
                        continue;
                    };

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
                    let target_player = if let Some(data) = self.player_mut(&target) {
                        data
                    } else {
                        continue;
                    };
                    target_player.hazards = HazardBlock::default();
                    let player_name = format!("{}", target_player);

                    self.log(format!("hazards were cleared from {}'s field", player_name));
                }
                Effect::Damage(target, damage) => {
                    let rem_hp;
                    if let Some(target_mon) = self.active_mut(&target) {
                        let prev = target_mon.hp.data;
                        target_mon.hp.data -= damage.collapse(target_mon.hp);
                        let diff = prev - target_mon.hp.data;
                        let mon_name = format!("{}", target_mon.id);
                        rem_hp = target_mon.hp.data;

                        self.log(format!(
                            "{}'s {} lost {} hp",
                            self.player(&target).unwrap(),
                            mon_name,
                            diff,
                        ));
                    } else {
                        continue;
                    }

                    if rem_hp == 0 {
                        self.player_mut(&target).unwrap().roster.kill();
                        self.log(String::from("They fainted"));
                    }
                }

                Effect::Cure(target) => {
                    if let Some(target_mon) = self.active_mut(&target) {
                        for status in &vec![Status::Burn, Status::Paralyse, Status::Toxic] {
                            target_mon.status.remove(status);
                        }
                        let mon_name = format!("{}", target_mon.id);

                        self.log(format!(
                            "{}'s {} was cured of status",
                            self.player(&target).unwrap(),
                            mon_name,
                        ));
                    }
                }
                Effect::Heal(target, frac) => {
                    if let Some(target_mon) = self.active_mut(&target) {
                        if target_mon.hp.data == 0 {
                            continue;
                        }
                        let prev = target_mon.hp.data;
                        target_mon.hp += target_mon.hp.max / frac;
                        let diff = target_mon.hp - prev;
                        let mon_name = format!("{}", target_mon.id);

                        self.log(format!(
                            "{}'s {} gained {} hp",
                            self.player(&target).unwrap(),
                            mon_name,
                            diff,
                        ));
                    }
                }
                Effect::OHKO(target) => {
                    let mon_name;
                    if let Some(target_mon) = self.active_mut(&target) {
                        target_mon.hp.data = 0;
                        mon_name = format!("{}", target_mon.id);
                    } else {
                        continue;
                    }
                    self.player_mut(&target).unwrap().roster.kill();

                    self.log(format!(
                        "{}'s {} fainted!",
                        self.player(&target).unwrap(),
                        mon_name
                    ));
                }
                Effect::MidSwitch(target) => {
                    let target_player = if let Some(data) = self.player_mut(&target) {
                        data
                    } else {
                        continue;
                    };
                    if target_player.roster.dead == 1 {
                        continue;
                    }
                    // self.get_input().unwrap();
                    // let effects_ = self.process_move();
                    // self.apply_effects(effects).unwrap();
                }
                Effect::SetWeather(weather) => {
                    if self.weather != Some(weather) {
                        self.weather = Some(weather);
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
            }
        }
    }
}
