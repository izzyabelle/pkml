use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::bounded_i32::BoundedI32;
use crate::game::{MoveSelection, WeatherId};
use crate::moves::Move;
use crate::poketype::Poketype;
use crate::preset::PokeId;
use crate::selvec::PointerVec;
use crate::stat::{Stat, StatBlock, StatId};
use crate::status::Status;
use crate::trigger::{Ability, Item};

#[derive(Debug, Default)]
pub struct Pokemon {
    pub ability: Ability,
    pub hp: BoundedI32,
    pub id: PokeId,
    pub item: Rc<RefCell<Option<Item>>>,
    pub moves: PointerVec<Move>,
    pub poketype: Rc<RefCell<Poketype>>,
    pub stats: StatBlock,
    pub status: HashMap<Status, i8>,
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
            self.poketype.borrow(),
            if let Some(item) = *self.item.borrow() {
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
    pub fn new(
        ability: Ability,
        hp: BoundedI32,
        id: PokeId,
        item: Option<Item>,
        moves: PointerVec<Move>,
        poketype: Poketype,
        stats: [i32; 5],
        weather: Rc<RefCell<Option<WeatherId>>>,
    ) -> Self {
        let item = Rc::new(RefCell::new(item));
        let poketype = Rc::new(RefCell::new(poketype));
        let stats = StatBlock::new(stats, Rc::clone(&poketype), Rc::clone(&item), weather);
        Self {
            ability,
            hp,
            id,
            item,
            moves,
            poketype,
            stats,
            status: HashMap::new(),
        }
    }

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
