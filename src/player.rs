use std::cell::RefCell;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::bounded_i32::BoundedI32;
use crate::game::{HazardId, MoveSelection, WeatherId};
use crate::pokemon::Pokemon;
use crate::preset::PokeId;
use crate::selvec::PointerVec;

#[derive(Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub ai: bool,
    pub hazards: HazardBlock,
    pub roster: PointerVec<Pokemon>,
    pub inputs: Vec<MoveSelection>,
}

#[derive(Debug)]
pub struct HazardBlock {
    pub stealth_rock: BoundedI32,
    pub toxic_spikes: BoundedI32,
    pub spikes: BoundedI32,
}

impl Default for HazardBlock {
    fn default() -> Self {
        Self {
            stealth_rock: BoundedI32 {
                data: 0,
                min: 0,
                max: 1,
            },
            toxic_spikes: BoundedI32 {
                data: 0,
                min: 0,
                max: 2,
            },
            spikes: BoundedI32 {
                data: 0,
                min: 0,
                max: 3,
            },
        }
    }
}

impl Index<HazardId> for HazardBlock {
    type Output = BoundedI32;
    fn index(&self, index: HazardId) -> &Self::Output {
        match index {
            HazardId::Stealthrock => &self.stealth_rock,
            HazardId::Spikes => &self.spikes,
            HazardId::ToxicSpikes => &self.toxic_spikes,
        }
    }
}

impl IndexMut<HazardId> for HazardBlock {
    fn index_mut(&mut self, index: HazardId) -> &mut Self::Output {
        match index {
            HazardId::Stealthrock => &mut self.stealth_rock,
            HazardId::Spikes => &mut self.spikes,
            HazardId::ToxicSpikes => &mut self.toxic_spikes,
        }
    }
}

impl Player {
    pub fn new(ai: bool, weather: Rc<RefCell<Option<WeatherId>>>) -> Self {
        Self {
            name: "test",
            ai,
            hazards: HazardBlock::default(),
            roster: PointerVec::from(vec![
                Pokemon::preset(PokeId::Jirachi, Rc::clone(&weather)),
                Pokemon::preset(PokeId::Tyranitar, Rc::clone(&weather)),
                Pokemon::preset(PokeId::Heatran, Rc::clone(&weather)),
                Pokemon::preset(PokeId::Breloom, Rc::clone(&weather)),
                Pokemon::preset(PokeId::Zapdos, Rc::clone(&weather)),
                Pokemon::preset(PokeId::Starmie, weather),
            ]),
            inputs: Vec::new(),
        }
    }

    pub fn has_active(&self) -> bool {
        self.roster.active.is_some()
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(if self.ai { "Bot" } else { "Human" }))
    }
}
