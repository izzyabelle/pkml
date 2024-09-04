use std::collections::HashMap;
use std::fmt::Display;

use crate::bounded_i32::BoundedI32;
use crate::moves::{Move, MoveId};
use crate::pokemon::{Pokemon, StatBlock};
use crate::poketype::{Poketype, Type};
use crate::selvec::PointerVec;
use crate::stat::{Stat, StatId};
use crate::trigger::Ability;
use crate::trigger::Item;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum PokeId {
    #[default]
    Jirachi,
    Zapdos,
    Starmie,
    Heatran,
    Tyranitar,
    Breloom,
}

impl Display for PokeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PokeId::Jirachi => "Jirachi",
                PokeId::Zapdos => "Zapdos",
                PokeId::Starmie => "Starmie",
                PokeId::Heatran => "Heatran",
                PokeId::Tyranitar => "Tyranitar",
                PokeId::Breloom => "Breloom",
            }
        )
    }
}

impl From<PokeId> for Pokemon {
    fn from(value: PokeId) -> Self {
        match value {
            PokeId::Jirachi => Self {
                ability: Ability::SereneGrace,
                hp: BoundedI32::new(404, 0, 404).unwrap(),
                id: value,
                item: Some(Item::Leftovers),
                moves: PointerVec::from(vec![
                    Move::from(MoveId::IronHead),
                    Move::from(MoveId::BodySlam),
                    Move::from(MoveId::Firepunch),
                    Move::from(MoveId::Refresh),
                ]),
                poketype: Poketype::Dual([Type::Steel, Type::Psychic]),
                stats: StatBlock::new(
                    [249, 236, 212, 286, 263],
                    Poketype::Dual([Type::Steel, Type::Psychic]),
                    Some(Item::Leftovers),
                ),
                status: HashMap::new(),
            },
            PokeId::Zapdos => Self {
                ability: Ability::Pressure,
                hp: BoundedI32::new(383, 0, 383).unwrap(),
                id: value,
                item: Some(Item::Leftovers),
                moves: PointerVec::from(vec![
                    Move::from(MoveId::Thunderbolt),
                    Move::from(MoveId::Roost),
                    Move::from(MoveId::Hpice),
                    Move::from(MoveId::Uturn),
                ]),
                poketype: Poketype::Dual([Type::Electric, Type::Flying]),
                stats: StatBlock::new(
                    [193, 288, 286, 216, 244],
                    Poketype::Dual([Type::Electric, Type::Flying]),
                    Some(Item::Leftovers),
                ),
                status: HashMap::new(),
            },
            PokeId::Starmie => Self {
                ability: Ability::NaturalCure,
                hp: BoundedI32::new(261, 0, 261).unwrap(),
                id: value,
                item: Some(Item::Leftovers),
                moves: PointerVec::from(vec![
                    Move::from(MoveId::Hydropump),
                    Move::from(MoveId::Thunderwave),
                    Move::from(MoveId::Icebeam),
                    Move::from(MoveId::Rapidspin),
                ]),
                poketype: Poketype::Dual([Type::Water, Type::Psychic]),
                stats: StatBlock::new(
                    [139, 207, 299, 206, 361],
                    Poketype::Dual([Type::Water, Type::Psychic]),
                    Some(Item::Leftovers),
                ),
                status: HashMap::new(),
            },
            PokeId::Heatran => Self {
                ability: Ability::Flashfire,
                hp: BoundedI32::new(261, 0, 261).unwrap(),
                id: value,
                item: Some(Item::Leftovers),
                moves: PointerVec::from(vec![
                    Move::from(MoveId::Stealthrock),
                    Move::from(MoveId::Fireblast),
                    Move::from(MoveId::Earthpower),
                    Move::from(MoveId::Explosion),
                ]),
                poketype: Poketype::Dual([Type::Fire, Type::Steel]),
                stats: StatBlock::new(
                    [193, 247, 326, 248, 253],
                    Poketype::Dual([Type::Fire, Type::Steel]),
                    Some(Item::Leftovers),
                ),
                status: HashMap::new(),
            },
            PokeId::Tyranitar => Self {
                ability: Ability::SandStream,
                hp: BoundedI32::new(341, 0, 341).unwrap(),
                id: value,
                item: Some(Item::Leftovers),
                moves: PointerVec::from(vec![
                    Move::from(MoveId::Crunch),
                    Move::from(MoveId::Pursuit),
                    Move::from(MoveId::Superpower),
                    Move::from(MoveId::Stoneedge),
                ]),
                poketype: Poketype::Dual([Type::Rock, Type::Dark]),
                stats: StatBlock::new(
                    [367, 256, 206, 237, 243],
                    Poketype::Dual([Type::Rock, Type::Dark]),
                    Some(Item::Leftovers),
                ),
                status: HashMap::new(),
            },
            PokeId::Breloom => Self {
                ability: Ability::Poisonheal,
                hp: BoundedI32::new(280, 0, 280).unwrap(),
                id: value,
                item: Some(Item::Leftovers),
                moves: PointerVec::from(vec![
                    Move::from(MoveId::Spore),
                    Move::from(MoveId::Superpower),
                    Move::from(MoveId::Seedbomb),
                    Move::from(MoveId::Machpunch),
                ]),
                poketype: Poketype::Dual([Type::Grass, Type::Fighting]),
                stats: StatBlock::new(
                    [393, 196, 140, 156, 222],
                    Poketype::Dual([Type::Grass, Type::Fighting]),
                    Some(Item::Leftovers),
                ),
                status: HashMap::new(),
            },
        }
    }
}
