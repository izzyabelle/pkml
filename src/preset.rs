use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::bounded_i32::BoundedI32;
use crate::game::WeatherId;
use crate::moves::{Move, MoveId};
use crate::pokemon::Pokemon;
use crate::poketype::{Poketype, Type};
use crate::selvec::PointerVec;
use crate::stat::StatBlock;
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

impl Pokemon {
    pub fn preset(value: PokeId, weather: Rc<RefCell<Option<WeatherId>>>) -> Self {
        match value {
            PokeId::Jirachi => Pokemon::new(
                Ability::SereneGrace,
                BoundedI32::zero(404),
                value,
                Some(Item::Leftovers),
                PointerVec::from(vec![
                    Move::from(MoveId::IronHead),
                    Move::from(MoveId::BodySlam),
                    Move::from(MoveId::Firepunch),
                    Move::from(MoveId::Refresh),
                ]),
                Poketype::Dual([Type::Steel, Type::Psychic]),
                [249, 236, 212, 286, 263],
                weather,
            ),
            PokeId::Zapdos => Pokemon::new(
                Ability::Pressure,
                BoundedI32::zero(383),
                value,
                Some(Item::Leftovers),
                PointerVec::from(vec![
                    Move::from(MoveId::Thunderbolt),
                    Move::from(MoveId::Roost),
                    Move::from(MoveId::Hpice),
                    Move::from(MoveId::Uturn),
                ]),
                Poketype::Dual([Type::Electric, Type::Flying]),
                [193, 288, 286, 216, 244],
                weather,
            ),
            PokeId::Starmie => Pokemon::new(
                Ability::NaturalCure,
                BoundedI32::zero(261),
                value,
                Some(Item::Leftovers),
                PointerVec::from(vec![
                    Move::from(MoveId::Hydropump),
                    Move::from(MoveId::Thunderwave),
                    Move::from(MoveId::Icebeam),
                    Move::from(MoveId::Rapidspin),
                ]),
                Poketype::Dual([Type::Water, Type::Psychic]),
                [139, 207, 299, 206, 361],
                weather,
            ),
            PokeId::Heatran => Pokemon::new(
                Ability::Flashfire,
                BoundedI32::zero(261),
                value,
                Some(Item::Leftovers),
                PointerVec::from(vec![
                    Move::from(MoveId::Stealthrock),
                    Move::from(MoveId::Fireblast),
                    Move::from(MoveId::Earthpower),
                    Move::from(MoveId::Explosion),
                ]),
                Poketype::Dual([Type::Fire, Type::Steel]),
                [193, 247, 326, 248, 253],
                weather,
            ),
            PokeId::Tyranitar => Pokemon::new(
                Ability::SandStream,
                BoundedI32::zero(341),
                value,
                Some(Item::Leftovers),
                PointerVec::from(vec![
                    Move::from(MoveId::Crunch),
                    Move::from(MoveId::Pursuit),
                    Move::from(MoveId::Superpower),
                    Move::from(MoveId::Stoneedge),
                ]),
                Poketype::Dual([Type::Rock, Type::Dark]),
                [367, 256, 206, 237, 243],
                weather,
            ),
            PokeId::Breloom => Pokemon::new(
                Ability::SandStream,
                BoundedI32::zero(280),
                value,
                Some(Item::ToxicOrb),
                PointerVec::from(vec![
                    Move::from(MoveId::Spore),
                    Move::from(MoveId::Superpower),
                    Move::from(MoveId::Seedbomb),
                    Move::from(MoveId::Machpunch),
                ]),
                Poketype::Dual([Type::Grass, Type::Fighting]),
                [393, 196, 140, 156, 222],
                weather,
            ),
        }
    }
}
