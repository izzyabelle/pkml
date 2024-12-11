use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use rand::{thread_rng, Rng};

use crate::bounded_i32::BoundedI32;
use crate::game::{MoveSelection, WeatherId};
use crate::moves::Move;
use crate::poketype::Poketype;
use crate::preset::PokeId;
use crate::selvec::PointerVec;
use crate::stat::{StatBlock, StatId};
use crate::status::{Status, StatusBlock};
use crate::trigger::{Ability, Item};

#[derive(Debug, Clone, Default)]
pub struct Pokemon {
    pub ability: Ability,
    pub hp: BoundedI32,
    pub id: PokeId,
    pub item: Rc<RefCell<Option<Item>>>,
    pub moves: PointerVec<Move>,
    pub poketype: Rc<RefCell<Poketype>>,
    pub stats: StatBlock,
    pub status: Rc<RefCell<StatusBlock>>,
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
        let status = Rc::new(RefCell::new(StatusBlock::new()));
        let stats = StatBlock::new(
            stats,
            Rc::clone(&poketype),
            Rc::clone(&item),
            weather,
            Rc::clone(&status),
        );
        Self {
            ability,
            hp,
            id,
            item,
            moves,
            poketype,
            stats,
            status,
        }
    }

    pub fn get_move(&self, move_: &MoveSelection) -> Option<&Move> {
        match move_ {
            MoveSelection::Switch(_) => None,
            MoveSelection::Move(idx) => Some(&self.moves[*idx]),
        }
    }
    /// if return.1 skip move, if Some always log message
    pub fn exec_moveskip(&mut self) -> (Option<String>, bool) {
        let mut rand = thread_rng();
        let mut removed_statuses = Vec::new();
        let mut message = None;
        let mut skip_turn = false;
        let mut statusblock = self.status.try_borrow_mut().unwrap();
        for status in &[
            Status::Paralyse,
            Status::Sleep,
            Status::Freeze,
            Status::Confusion,
            Status::Flinch,
        ] {
            if let Some(value) = statusblock.data.get_mut(status) {
                match status {
                    Status::Paralyse => {
                        if rand.gen_range(0..=3) == 0 {
                            message = Some(format!("{} was full para", self.id));
                            skip_turn = true;
                        }
                    }
                    Status::Sleep => {
                        if *value == 0 {
                            Some(format!("{} woke up!", self.id));
                            removed_statuses.push(status);
                        } else {
                            *value -= 1;
                            message = Some(format!("{} was sleeping", self.id));
                            skip_turn = true;
                        }
                    }
                    Status::Freeze => {
                        if rand.gen_range(0..=9) == 0 {
                            message = Some(format!("{} thawed!", self.id));
                            removed_statuses.push(status);
                        } else {
                            message = Some(format!("{} is frozen", self.id));
                            skip_turn = true;
                        }
                    }
                    Status::Confusion => {
                        if *value == 0 {
                            message = Some(format!("{} snapped out of confusion", self.id));
                            removed_statuses.push(status);
                        } else if rand.gen_range(0..=1) == 0 {
                            message = Some(format!("{} hit itself in confusion(todo)", self.id));
                            skip_turn = true;
                            *value -= 1;
                        } else {
                            *value -= 1;
                        }
                    }
                    Status::Flinch => {
                        message = Some(format!("{} flinched", self.id));
                        removed_statuses.push(status);
                    }
                    _ => {}
                }
            }
        }
        if let Ok(mut data) = self.status.try_borrow_mut() {
            for status in removed_statuses {
                data.data.remove(status);
            }
        }
        (message, skip_turn)
    }
}
