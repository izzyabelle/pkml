use core::fmt;

use crate::{
    bounded_i32::BoundedI32,
    game::WeatherId,
    poketype::{Poketype, Type},
    trigger::Item,
    EmptyResult,
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Stat {
    id: StatId,
    pub curr: i32,
    base: i32,
    stage: BoundedI32,
    weather: Option<WeatherId>,
    poketype: Poketype,
    item: Option<Item>,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum StatId {
    #[default]
    Atk,
    Def,
    Spa,
    Spd,
    Spe,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:>+0width$}", self.curr, self.stage, width = 2)
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
    pub fn new(value: i32, id: StatId, poketype: Poketype, item: Option<Item>) -> Self {
        Self {
            id,
            curr: value,
            base: value,
            stage: BoundedI32 {
                data: 0,
                min: -6,
                max: 6,
            },
            weather: None,
            poketype,
            item,
        }
    }
    pub fn reset(&mut self) {
        self.curr = self.base;
        self.stage.data = 0;
    }

    pub fn update_stats(&mut self) {
        match (self.weather, self.id) {
            (Some(WeatherId::Sand), StatId::Spd) => {
                if self.poketype.contains(Type::Rock) {
                    self.curr = (self.curr * 15) / 10
                }
            }
            (Some(WeatherId::Hail), StatId::Def) => {
                if self.poketype.contains(Type::Ice) {
                    self.curr = (self.curr * 15) / 10
                }
            }
            _ => {}
        }

        match (self.item, self.id) {
            (Some(Item::ChoiceScarf), StatId::Spe) => self.curr = (self.curr * 15) / 10,
            _ => {}
        }
    }

    pub fn set_weather(&mut self, weather: Option<WeatherId>) {
        self.weather = weather;
        self.update_stats();
    }

    /// as stat stages are altered the resultant total stat is calculated
    pub fn alter(&mut self, diff: i32) -> bool {
        // bool to return if any change actually occurred
        let success = if (self.stage.data == -6) | (self.stage.data == 6) {
            false
        } else {
            true
        };
        self.stage += diff;

        // set current stat according to GF's strange formula
        if self.stage.data < 0 {
            self.curr = self.base * (2 / (2 + self.stage.data))
        } else {
            self.curr = self.base * ((2 + self.stage.data) / 2)
        }

        self.update_stats();

        success
    }
}
