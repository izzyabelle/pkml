use std::fmt::Display;

use crate::effect::{Effect, PlayerId};
use crate::game::HazardId;
use crate::poketype::Type;
use crate::stat::StatId;
use crate::status::Status;

#[derive(Debug)]
pub struct Move {
    id: MoveId,
    pp: Pp,
    damage_type: Mtype,
    poke_type: Type,
    base_power: BasePower,
    priority: Priority,
    effects: Vec<Effect>,
    freq: Freq,
    target: PlayerId,
    accuracy: Accuracy,
}

impl From<MoveId> for Move {
    fn from(value: MoveId) -> Self {
        Self {
            id: value,
            pp: Pp::from(value),
            damage_type: Mtype::from(value),
            poke_type: Type::from(value),
            base_power: BasePower::from(value),
            priority: Priority::from(value),
            effects: Vec::from(value),
            freq: Freq::from(value),
            target: PlayerId::from(value),
            accuracy: Accuracy::from(value),
        }
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::from(MoveId::default())
    }
}

impl From<MoveId> for PlayerId {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::Roost => Self::Active,
            MoveId::Refresh => Self::Active,
            _ => Self::Inactive,
        }
    }
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum MoveId {
    #[default]
    IronHead,
    BodySlam,
    Uturn,
    Stealthrock,
    Thunderbolt,
    Roost,
    Hpice,
    Firepunch,
    Refresh,
    Hydropump,
    Thunderwave,
    Icebeam,
    Rapidspin,
    Fireblast,
    Earthpower,
    Explosion,
    Crunch,
    Pursuit,
    Superpower,
    Stoneedge,
    Spore,
    Seedbomb,
    Machpunch,
    Struggle,
    Switch(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mtype {
    Physical,
    Special,
    Status,
}

#[derive(Debug, Default)]
pub struct Pp {
    pub current: i8,
    pub max: i8,
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone, Copy)]
pub struct Priority(i8);

#[derive(Debug, Default, Clone, Copy)]
pub struct Freq(Option<f32>);

#[derive(Debug, Default, Clone, Copy)]
pub struct BasePower(i32);

#[derive(Debug, Default, Clone, Copy)]
pub struct Accuracy(Option<f32>);

impl From<i8> for Pp {
    fn from(value: i8) -> Self {
        Self {
            current: value,
            max: value,
        }
    }
}

impl From<MoveId> for Priority {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::Switch(_) => Priority(5),
            MoveId::Machpunch => Priority(1),
            _ => Priority(0),
        }
    }
}

impl Display for MoveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MoveId::IronHead => String::from("Iron Head"),
                MoveId::BodySlam => String::from("Body Slam"),
                MoveId::Uturn => String::from("U turn"),
                MoveId::Stealthrock => String::from("Stealth Rock"),
                MoveId::Thunderbolt => String::from("Thunderbolt"),
                MoveId::Roost => String::from("Roost"),
                MoveId::Hpice => String::from("HP Ice"),
                MoveId::Firepunch => String::from("Fire Punch"),
                MoveId::Refresh => String::from("Refresh"),
                MoveId::Hydropump => String::from("Hydro Pump"),
                MoveId::Thunderwave => String::from("Thunder Wave"),
                MoveId::Icebeam => String::from("Ice Beam"),
                MoveId::Rapidspin => String::from("Rapid Spin"),
                MoveId::Fireblast => String::from("Fire Blast"),
                MoveId::Earthpower => String::from("Earth Power"),
                MoveId::Explosion => String::from("Explosion"),
                MoveId::Crunch => String::from("Crunch"),
                MoveId::Pursuit => String::from("Pursuit"),
                MoveId::Superpower => String::from("Superpower"),
                MoveId::Stoneedge => String::from("Stone Edge"),
                MoveId::Spore => String::from("Spore"),
                MoveId::Seedbomb => String::from("Seed Bomb"),
                MoveId::Machpunch => String::from("Mach Punch"),
                MoveId::Struggle => String::from("Struggle"),
                MoveId::Switch(_) => String::from("Switch"),
            }
        )
    }
}

impl From<MoveId> for Type {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::IronHead => Type::Steel,
            MoveId::BodySlam => Type::Normal,
            MoveId::Uturn => Type::Bug,
            MoveId::Stealthrock => Type::Rock,
            MoveId::Thunderbolt => Type::Electric,
            MoveId::Roost => Type::Flying,
            MoveId::Hpice => Type::Ice,
            MoveId::Firepunch => Type::Fire,
            MoveId::Refresh => Type::Normal,
            MoveId::Hydropump => Type::Water,
            MoveId::Thunderwave => Type::Electric,
            MoveId::Icebeam => Type::Ice,
            MoveId::Rapidspin => Type::Normal,
            MoveId::Fireblast => Type::Fire,
            MoveId::Earthpower => Type::Ground,
            MoveId::Explosion => Type::Normal,
            MoveId::Crunch => Type::Dark,
            MoveId::Pursuit => Type::Dark,
            MoveId::Superpower => Type::Fighting,
            MoveId::Stoneedge => Type::Rock,
            MoveId::Spore => Type::Grass,
            MoveId::Seedbomb => Type::Grass,
            MoveId::Machpunch => Type::Fighting,
            _ => Type::None,
        }
    }
}

impl From<MoveId> for Mtype {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::IronHead => Mtype::Physical,
            MoveId::BodySlam => Mtype::Physical,
            MoveId::Uturn => Mtype::Physical,
            MoveId::Stealthrock => Mtype::Status,
            MoveId::Thunderbolt => Mtype::Special,
            MoveId::Roost => Mtype::Status,
            MoveId::Hpice => Mtype::Special,
            MoveId::Firepunch => Mtype::Physical,
            MoveId::Refresh => Mtype::Status,
            MoveId::Hydropump => Mtype::Special,
            MoveId::Thunderwave => Mtype::Status,
            MoveId::Icebeam => Mtype::Special,
            MoveId::Rapidspin => Mtype::Physical,
            MoveId::Fireblast => Mtype::Special,
            MoveId::Earthpower => Mtype::Special,
            MoveId::Explosion => Mtype::Physical,
            MoveId::Crunch => Mtype::Physical,
            MoveId::Pursuit => Mtype::Physical,
            MoveId::Superpower => Mtype::Physical,
            MoveId::Stoneedge => Mtype::Physical,
            MoveId::Spore => Mtype::Status,
            MoveId::Seedbomb => Mtype::Physical,
            MoveId::Machpunch => Mtype::Physical,
            MoveId::Struggle => Mtype::Physical,
            _ => Mtype::Status,
        }
    }
}

impl From<MoveId> for Freq {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::IronHead => Self(Some(0.3)),
            MoveId::BodySlam => Self(Some(0.3)),
            MoveId::Thunderbolt => Self(Some(0.1)),
            MoveId::Firepunch => Self(Some(0.1)),
            MoveId::Icebeam => Self(Some(0.1)),
            MoveId::Fireblast => Self(Some(0.1)),
            MoveId::Earthpower => Self(Some(0.2)),
            MoveId::Crunch => Self(Some(0.2)),
            _ => Self::default(),
        }
    }
}

impl From<MoveId> for Vec<Effect> {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::IronHead => vec![Effect::InflictStatus(PlayerId::Inactive, Status::Flinch)],
            MoveId::BodySlam => vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)],
            MoveId::Uturn => vec![Effect::MidSwitch(PlayerId::Active)],
            MoveId::Stealthrock => vec![Effect::InflictHazard(
                PlayerId::Inactive,
                HazardId::Stealthrock,
            )],
            MoveId::Thunderbolt => {
                vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)]
            }
            MoveId::Roost => vec![Effect::Heal(PlayerId::Active)],
            MoveId::Firepunch => vec![Effect::InflictStatus(PlayerId::Inactive, Status::Burn)],
            MoveId::Refresh => vec![Effect::Cure(PlayerId::Active)],
            MoveId::Thunderwave => {
                vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)]
            }
            MoveId::Icebeam => vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)],
            MoveId::Rapidspin => vec![Effect::ClearHazard(PlayerId::Active)],
            MoveId::Fireblast => vec![Effect::InflictStatus(PlayerId::Inactive, Status::Burn)],
            MoveId::Earthpower => vec![Effect::AlterStat(PlayerId::Inactive, StatId::Spd, -1)],
            MoveId::Explosion => vec![Effect::OHKO(PlayerId::Active)],
            MoveId::Crunch => vec![Effect::AlterStat(PlayerId::Inactive, StatId::Def, -1)],
            MoveId::Superpower => vec![
                Effect::AlterStat(PlayerId::Active, StatId::Atk, -1),
                Effect::AlterStat(PlayerId::Active, StatId::Def, -1),
            ],
            MoveId::Spore => vec![Effect::InflictStatus(PlayerId::Inactive, Status::Sleep)],
            _ => Self::default(),
        }
    }
}

impl From<MoveId> for BasePower {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::IronHead => Self(80),
            MoveId::BodySlam => Self(85),
            MoveId::Uturn => Self(70),
            MoveId::Thunderbolt => Self(95),
            MoveId::Hpice => Self(70),
            MoveId::Firepunch => Self(75),
            MoveId::Hydropump => Self(120),
            MoveId::Icebeam => Self(95),
            MoveId::Rapidspin => Self(20),
            MoveId::Fireblast => Self(120),
            MoveId::Earthpower => Self(90),
            MoveId::Explosion => Self(250),
            MoveId::Crunch => Self(90),
            MoveId::Pursuit => Self(40),
            MoveId::Superpower => Self(120),
            MoveId::Stoneedge => Self(120),
            MoveId::Seedbomb => Self(80),
            MoveId::Machpunch => Self(40),
            MoveId::Struggle => Self(50),
            _ => Self::default(),
        }
    }
}

impl From<MoveId> for Pp {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::IronHead => Self::from(24),
            MoveId::BodySlam => Self::from(24),
            MoveId::Uturn => Self::from(32),
            MoveId::Stealthrock => Self::from(32),
            MoveId::Thunderbolt => Self::from(24),
            MoveId::Roost => Self::from(16),
            MoveId::Hpice => Self::from(24),
            MoveId::Firepunch => Self::from(24),
            MoveId::Refresh => Self::from(32),
            MoveId::Hydropump => Self::from(8),
            MoveId::Thunderwave => Self::from(32),
            MoveId::Icebeam => Self::from(16),
            MoveId::Rapidspin => Self::from(64),
            MoveId::Fireblast => Self::from(8),
            MoveId::Earthpower => Self::from(16),
            MoveId::Explosion => Self::from(8),
            MoveId::Crunch => Self::from(24),
            MoveId::Pursuit => Self::from(32),
            MoveId::Superpower => Self::from(8),
            MoveId::Stoneedge => Self::from(8),
            MoveId::Spore => Self::from(24),
            MoveId::Seedbomb => Self::from(24),
            MoveId::Machpunch => Self::from(48),
            _ => Self::default(),
        }
    }
}

impl From<MoveId> for Accuracy {
    fn from(value: MoveId) -> Self {
        match value {
            MoveId::Hydropump => Self(Some(0.8)),
            MoveId::Fireblast => Self(Some(0.85)),
            MoveId::Stoneedge => Self(Some(0.8)),
            _ => Self::default(),
        }
    }
}
