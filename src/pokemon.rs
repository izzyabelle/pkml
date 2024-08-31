use std::collections::HashMap;

use crate::bounded_i32::BoundedI32;
use crate::moves::Move;
use crate::poketype::Poketype;
use crate::preset::PokeId;
use crate::selvec::SelVec;
use crate::stat::{Stat, StatId};
use crate::status::Status;
use crate::trigger::{Ability, Item};

#[derive(Debug, Default)]
pub struct Pokemon {
    pub ability: Ability,
    pub hp: BoundedI32,
    pub id: PokeId,
    pub item: Option<Item>,
    pub moves: SelVec<Move>,
    pub poketype: Poketype,
    pub stats: HashMap<StatId, Stat>,
    pub status: HashMap<Status, i8>,
}
