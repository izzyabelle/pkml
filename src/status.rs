use std::{borrow::Borrow, collections::HashMap, fmt::Display};

use rand::{random, thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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
    pub fn is_nv(&self) -> bool {
        matches!(
            self,
            Status::Paralyse
                | Status::Burn
                | Status::Poison
                | Status::Toxic
                | Status::Sleep
                | Status::Freeze
        )
    }
}

#[derive(Default, Debug)]
pub struct StatusBlock {
    pub data: HashMap<Status, i8>,
    nv: Option<Status>,
}

impl StatusBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, status: &Status) -> bool {
        if status.is_nv() {
            if self.nv.is_some() {
                return false;
            } else {
                self.nv = Some(*status);
            }
        }

        let mut rand = thread_rng();

        if let std::collections::hash_map::Entry::Vacant(e) = self.data.entry(*status) {
            e.insert(match status {
                Status::Sleep => rand.gen_range(1..=3),
                Status::Confusion => rand.gen_range(2..5),
                Status::Drowsy => 2,
                _ => 0,
            });
            true
        } else {
            false
        }
    }

    pub fn clear_nv(&mut self) -> bool {
        if let Some(status) = self.nv {
            self.data.remove(&status);
            self.nv = None;
            true
        } else {
            false
        }
    }

    pub fn increment(&mut self, status: &Status) {
        if let Some(data) = self.data.get_mut(status) {
            match status {
                Status::Toxic => *data += 1,
                Status::Sleep => *data -= 1,
                Status::Confusion => *data -= 1,
                Status::Drowsy => *data -= 1,
                _ => {}
            }
        }
    }
}
