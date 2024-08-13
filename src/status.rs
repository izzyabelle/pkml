use std::collections::HashMap;
use std::fmt::Display;

use crate::effect::{Damage, Effect, PlayerId};
use crate::pokemon::Pokemon;
use crate::stat::IdIndex;
use crate::trigger::Ability;
use crate::Result;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    Paralyse,
    Burn,
    Poison,
    Toxic,
    Sleep,
    Freeze,
    Flinch,
    Confusion,
    Drowsy,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Paralyse => String::from("PAR"),
                Status::Burn => String::from("BRN"),
                Status::Poison => String::from("PSN"),
                Status::Toxic => String::from("TOX"),
                Status::Sleep => String::from("SLP"),
                Status::Freeze => String::from("FRZ"),
                Status::Flinch => String::from("flinched"),
                Status::Confusion => String::from("confused"),
                Status::Drowsy => String::from("drowsy"),
            }
        )
    }
}

impl Status {
    pub fn initialise(&mut self) -> Option<i32> {
        let mut rng = thread_rng();
        match self {
            Status::Toxic(active_turns) => {
                *active_turns = 0;
                None
            }
            Status::Sleep(length) => {
                *length = rng.gen_range(1..5);
                None
            }
            _ => None,
        }
    }

    pub fn trigger(&mut self) -> i8 {
        match self {
            Status::Sleep(curr) => {
                *curr -= 1;
                *curr
            }
            _ => 0,
        }
    }
}

impl Pokemon {
    pub fn has_nv_status(&self) -> bool {
        if let Some(_) = self.status[0] {
            true
        } else {
            false
        }
    }

    pub fn reset_nv(&mut self) -> Result<()> {
        self.reset_stages();
        for i in 1..4 {
            self.status[i] = None;
        }
        Ok(())
    }

    pub fn has_status(&self, status: &Status) -> bool {
        if let Some(mon_status) = self.status[status.index()] {
            std::mem::discriminant(&mon_status) == std::mem::discriminant(&status)
        } else {
            false
        }
    }

    pub fn trigger_status(&mut self, statuses: Vec<Status>) -> Vec<Effect> {
        let mut ans = Vec::new();
        for status in statuses {
            if !self.has_status(&status) {
                continue;
            }
            match status {
                Status::Paralyse => {
                    if rand::thread_rng().gen::<f32>() <= 0.25 {
                        ans.push(Effect::MissTurn(PlayerId::Active));
                    }
                }
                Status::Burn => {
                    ans.push(Effect::Damage(
                        PlayerId::Active,
                        Damage::Fractional(1.0, 8.0),
                    ));
                }
                Status::Poison => {
                    ans.push(Effect::Damage(
                        PlayerId::Active,
                        Damage::Fractional(1.0, 8.0),
                    ));
                }
                Status::Toxic(_) => {
                    if let Status::Toxic(curr) = self.status[0].as_mut().unwrap() {
                        if let Ability::Poisonheal = self.ability {
                            ans.push(Effect::Damage(
                                PlayerId::Active,
                                Damage::Fractional(-1.0 as f32, 8.0),
                            ))
                        } else {
                            *curr += 1;
                            ans.push(Effect::Damage(
                                PlayerId::Active,
                                Damage::Fractional(*curr as f32, 16.0),
                            ));
                        }
                    }
                }
                Status::Sleep(_) => {
                    let mut flag = false;
                    if let Status::Sleep(curr) = self.status[0].as_mut().unwrap() {
                        *curr -= 1;
                        if *curr == -1 {
                            flag = true;
                        }
                    }
                    if flag {
                        self.status[0] = None;
                    } else {
                        ans.push(Effect::MissTurn(PlayerId::Active));
                    }
                }
                Status::Freeze => {
                    if rand::thread_rng().gen::<f32>() <= 0.1 {
                        self.status[0] = None;
                    } else {
                        ans.push(Effect::MissTurn(PlayerId::Active));
                    }
                }
                Status::Flinch => {
                    ans.push(Effect::MissTurn(PlayerId::Active));
                }
                _ => {
                    continue;
                }
            }
        }
        ans
    }

    pub fn add_status(&mut self, status: &mut Status) -> bool {
        if !self.has_status(status) && self.status[status.index()] == None && self.is_alive() {
            status.initialise();
            self.status[status.index()] = Some(*status);
            true
        } else {
            false
        }
    }

    pub fn remove_status(&mut self, status: Status) -> Option<i32> {
        self.status[status.index()] = None;
        None
    }

    pub fn refresh_status(&mut self) -> bool {
        let success = if let Some(_) = self.status[0] {
            true
        } else {
            false
        };
        self.status[0] = None;
        success
    }

    pub fn clear_status(&mut self) -> bool {
        let mut success = false;
        for i in 0..4 {
            if let Some(_) = self.status[i] {
                success = true;
            }
        }
        for i in 0..4 {
            self.status[i] = None;
        }
        success
    }

    pub fn nv_status_str(&self) -> String {
        if !self.is_alive() {
            return String::from("FNT");
        }
        if let Some(status) = self.status[0] {
            match status {
                Status::Burn => String::from("BRN"),
                Status::Paralyse => String::from("PAR"),
                Status::Poison => String::from("PSN"),
                Status::Toxic(_) => String::from("TOX"),
                Status::Sleep(_) => String::from("SLP"),
                Status::Freeze => String::from("FRZ"),
                _ => String::from("   "),
            }
        } else {
            String::from("   ")
        }
    }
}
