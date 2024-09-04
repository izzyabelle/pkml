use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

use crate::bounded_i32::BoundedI32;
use crate::game::{MoveSelection, WeatherId};
use crate::moves::Move;
use crate::poketype::Poketype;
use crate::preset::PokeId;
use crate::selvec::PointerVec;
use crate::stat::{Stat, StatId};
use crate::status::Status;
use crate::trigger::{Ability, Item};

#[derive(Debug, Default)]
pub struct Pokemon {
    pub ability: Ability,
    pub hp: BoundedI32,
    pub id: PokeId,
    pub item: Option<Item>,
    pub moves: PointerVec<Move>,
    pub poketype: Poketype,
    pub stats: StatBlock,
    pub status: HashMap<Status, i8>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct StatBlock {
    atk: Stat,
    def: Stat,
    spa: Stat,
    spd: Stat,
    spe: Stat,
    weather: WeatherId,
}

impl StatBlock {
    pub fn new(values: [i32; 5], poketype: Poketype, item: Option<Item>) -> Self {
        Self {
            atk: Stat::new(values[0], StatId::Atk, poketype, item),
            def: Stat::new(values[1], StatId::Def, poketype, item),
            spa: Stat::new(values[2], StatId::Spa, poketype, item),
            spd: Stat::new(values[3], StatId::Spd, poketype, item),
            spe: Stat::new(values[4], StatId::Spe, poketype, item),
            ..Default::default()
        }
    }
}

impl Index<StatId> for StatBlock {
    type Output = Stat;
    fn index(&self, index: StatId) -> &Self::Output {
        match index {
            StatId::Atk => &self.atk,
            StatId::Def => &self.def,
            StatId::Spa => &self.spa,
            StatId::Spd => &self.spd,
            StatId::Spe => &self.spe,
        }
    }
}

impl IndexMut<StatId> for StatBlock {
    fn index_mut(&mut self, index: StatId) -> &mut Stat {
        match index {
            StatId::Atk => &mut self.atk,
            StatId::Def => &mut self.def,
            StatId::Spa => &mut self.spa,
            StatId::Spd => &mut self.spd,
            StatId::Spe => &mut self.spe,
        }
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HP: {} / {} ({}%)\n\
            Type: {}\n\
            {}Ability: {}\n\
            Atk: {}\n\
            Def: {}\n\
            Spa: {}\n\
            Spd: {}\n\
            Spe: {}\n",
            self.hp.data,
            self.hp.max,
            self.hp.data * 100 / self.hp.max,
            self.poketype,
            if let Some(item) = self.item {
                format!("Item: {}\n", item)
            } else {
                String::new()
            },
            self.ability,
            self.stats[StatId::Atk],
            self.stats[StatId::Def],
            self.stats[StatId::Spa],
            self.stats[StatId::Spd],
            self.stats[StatId::Spe],
        )
    }
}

impl Pokemon {
    pub fn add_status(&mut self, status: Status) -> bool {
        if let Some(_) = self.status.get(&status) {
            false
        } else {
            self.status.insert(status, 1);
            true
        }
    }

    pub fn get_move(&self, move_: &MoveSelection) -> Option<&Move> {
        match move_ {
            MoveSelection::Switch(_) => None,
            MoveSelection::Move(idx) => Some(&self.moves[*idx]),
        }
    }
}
