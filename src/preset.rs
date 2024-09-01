use std::collections::HashMap;
use std::fmt::Display;

use crate::bounded_i32::BoundedI32;
use crate::moves::{Move, MoveId};
use crate::pokemon::Pokemon;
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
                stats: HashMap::from([
                    (StatId::Atk, Stat::from(249)),
                    (StatId::Def, Stat::from(236)),
                    (StatId::Spa, Stat::from(212)),
                    (StatId::Spd, Stat::from(286)),
                    (StatId::Spe, Stat::from(263)),
                ]),
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
                stats: HashMap::from([
                    (StatId::Atk, Stat::from(193)),
                    (StatId::Def, Stat::from(288)),
                    (StatId::Spa, Stat::from(286)),
                    (StatId::Spd, Stat::from(216)),
                    (StatId::Spe, Stat::from(244)),
                ]),
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
                stats: HashMap::from([
                    (StatId::Atk, Stat::from(139)),
                    (StatId::Def, Stat::from(207)),
                    (StatId::Spa, Stat::from(299)),
                    (StatId::Spd, Stat::from(206)),
                    (StatId::Spe, Stat::from(361)),
                ]),
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
                stats: HashMap::from([
                    (StatId::Atk, Stat::from(193)),
                    (StatId::Def, Stat::from(247)),
                    (StatId::Spa, Stat::from(326)),
                    (StatId::Spd, Stat::from(248)),
                    (StatId::Spe, Stat::from(253)),
                ]),
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
                stats: HashMap::from([
                    (StatId::Atk, Stat::from(367)),
                    (StatId::Def, Stat::from(256)),
                    (StatId::Spa, Stat::from(206)),
                    (StatId::Spd, Stat::from(237)),
                    (StatId::Spe, Stat::from(243)),
                ]),
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
                stats: HashMap::from([
                    (StatId::Atk, Stat::from(393)),
                    (StatId::Def, Stat::from(196)),
                    (StatId::Spa, Stat::from(140)),
                    (StatId::Spd, Stat::from(156)),
                    (StatId::Spe, Stat::from(222)),
                ]),
                status: HashMap::new(),
            },
        }
    }
}
