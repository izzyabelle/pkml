use std::collections::HashMap;
use std::fmt::Display;

use crate::bounded_i32::BoundedI32;
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
    pub stats: HashMap<StatId, Stat>,
    pub status: HashMap<Status, i8>,
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HP: {} / {} ({}%)\nType: {}\n{}Ability: {}\nAtk: {}\nDef: {}\nSpa: {}\nSpd: {}\nSpe: {}\n",
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
            self.stats[&StatId::Atk],
            self.stats[&StatId::Def],
            self.stats[&StatId::Spa],
            self.stats[&StatId::Spd],
            self.stats[&StatId::Spe],
        )
    }
}
