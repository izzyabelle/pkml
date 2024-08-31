use core::fmt;

use crate::EmptyResult;

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

impl Stat {
    pub fn reset(&mut self) -> EmptyResult {
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
