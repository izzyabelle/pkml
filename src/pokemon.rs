use std::collections::HashMap;

use crate::moves::{Move, MoveId};
use crate::poketype::{Poketype, Type};
use crate::preset::PokeId;
use crate::selvec::SelVec;
use crate::stat::{Hp, Stat, StatId};
use crate::status::Status;
use crate::trigger::{Ability, Item};
use crate::EmptyResult;

#[derive(Debug, Default)]
pub struct Pokemon {
    pub ability: Ability,
    pub hp: Hp,
    pub id: PokeId,
    pub item: Option<Item>,
    pub moves: SelVec<Move>,
    pub poketype: Poketype,
    pub stats: HashMap<StatId, Stat>,
    pub status: HashMap<Status, i8>,
}

impl Pokemon {
    pub fn use_move(&mut self, index: usize) -> MoveId {
        if self.remaining_moves == 0 {
            return MoveId::Struggle;
        }
        if self.moves[index].1 == 0 {
            panic!("unusable move used!");
        }
        self.moves[index].1 -= 1;
        if self.moves[index].1 == 0 {
            self.remaining_moves -= 1;
        }
        self.moves[index].0
    }

    pub fn reset_stages(&mut self) -> EmptyResult<()> {
        for i in 0..5 {
            self.stats[i].reset();
        }
        Ok(())
    }

    pub fn dec_pp(&mut self, index: usize) -> EmptyResult<()> {
        self.moves[index].1 -= 1;
        Ok(())
    }

    pub fn is_grounded(&self) -> bool {
        !(self.has_type(&Type::Flying) || self.has_ability(&Ability::Levitate))
    }

    pub fn can_move(&self) -> bool {
        todo!("");
    }
}
