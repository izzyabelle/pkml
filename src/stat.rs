use core::fmt;
use std::default;
use std::ops::Add;
use std::ops::AddAssign;

use crate::pokemon::Pokemon;
use crate::poketype::Type;
use crate::status::Status;
use crate::trigger::Item;
use crate::EmptyResult;
use crate::Result;
use crate::{effect::Damage, game::WeatherId};

#[derive(Default, Debug, Clone, Copy)]
pub struct Hp {
    current: i32,
    max: i32,
}

#[derive(Debug, Default)]
pub struct Stat {
    current: i32,
    base: i32,
    stage: i32,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum StatId {
    Atk,
    Def,
    Spa,
    Spd,
    Spe,
}

impl From<i32> for Stat {
    fn from(value: i32) -> Self {
        Self {
            current: value,
            base: value,
            ..Default::default()
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:>+0width$}", self.current, self.stage, width = 2)
    }
}

impl fmt::Display for StatId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatId::Atk => write!(f, "ATK"),
            StatId::Def => write!(f, "DEF"),
            StatId::Spa => write!(f, "SPA"),
            StatId::Spd => write!(f, "SPD"),
            StatId::Spe => write!(f, "SPE"),
        }
    }
}

impl fmt::Display for Hp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}% ({} / {})",
            (self.current as f32 / self.max as f32 * 100.0) as i32,
            self.current,
            self.max,
        )
    }
}

impl<T> PartialEq<T> for Hp
where
    T: TryInto<i32>,
    T: Copy,
{
    fn eq(&self, other: &T) -> bool {
        let b: Result<i32, _> = (*other).try_into();
        match b {
            Ok(b) => b == self.current,
            Err(_) => false,
        }
    }
}

impl Add<Damage> for Hp {
    type Output = Self;
    fn add(self, rhs: Damage) -> Self {
        let temp = self.current - i32::from(rhs);
        Self {
            current: if temp < 0 { 0 } else { temp },
            max: self.max,
        }
    }
}

impl AddAssign<Damage> for Hp {
    fn add_assign(&mut self, rhs: Damage) {
        *self = *self + rhs;
    }
}

impl From<i32> for Hp {
    fn from(value: i32) -> Self {
        Self {
            current: value,
            max: value,
        }
    }
}

impl Hp {
    pub fn reset(&mut self) -> EmptyResult<()> {
        self.current = self.max;
        Ok(())
    }

    pub fn is_max(&self) -> bool {
        self.current == self.max
    }
}

impl Stat {
    pub fn reset(&mut self) -> EmptyResult<()> {
        self.current = self.base;
        self.stage = 0;
        Ok(())
    }

    /// as stat stages are altered the resultant total stat is calculated
    pub fn alter(&mut self, diff: i32) -> bool {
        // bool to return if any change actually occurred
        let mut success = true;
        self.stage += diff;

        // stage is bounded between -6 and +6
        if self.stage > 6 {
            self.stage = 6;
            success = false;
        } else if self.stage < -6 {
            self.stage = -6;
            success = false;
        }

        // set current stat according to GF's strange formula
        if self.stage < 0 {
            self.current = self.base * (2 / (2 + self.stage))
        } else {
            self.current = self.base * ((2 + self.stage) / 2)
        }

        success
    }
}

impl Pokemon {
    pub fn get_stat(&self, stat: &StatId, weather: &Option<WeatherId>) -> i32 {
        let (current_stat, mult) = (
            self.stats[stat.index()].current as f32,
            match stat {
                StatId::Atk => {
                    if self.has_status(&Status::Burn) {
                        0.5
                    } else {
                        1.0
                    }
                }
                StatId::Spd => {
                    if let (Some(WeatherId::Sand), true) = (weather, self.has_type(&Type::Rock)) {
                        1.5
                    } else {
                        1.0
                    }
                }
                StatId::Spe => {
                    if self.has_status(&Status::Paralyse) {
                        0.25
                    } else if self.item == Some(Item::ChoiceScarf) {
                        1.5
                    } else {
                        1.0
                    }
                }
                _ => 1.0,
            },
        );

        (current_stat * mult) as i32
    }
}
