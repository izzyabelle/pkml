use std::fmt::Display;
use std::ops::{Index, IndexMut};

use crate::bounded_i32::BoundedI32;
use crate::game::{HazardId, MoveSelection};
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
    pub fn new(ai: bool) -> Self {
        Self {
            name: "test",
            ai,
            hazards: HazardBlock::default(),
            roster: PointerVec::from(vec![
                Pokemon::from(PokeId::Jirachi),
                Pokemon::from(PokeId::Tyranitar),
                Pokemon::from(PokeId::Heatran),
                Pokemon::from(PokeId::Breloom),
                Pokemon::from(PokeId::Zapdos),
                Pokemon::from(PokeId::Starmie),
            ]),
            inputs: Vec::new(),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(if self.ai { "Bot" } else { "Human" }))
    }
}
