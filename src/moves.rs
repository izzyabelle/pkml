use crate::bounded_i32::BoundedI32;
use crate::game::{Effect, HazardId, PlayerId};
use crate::poketype::Type;
use crate::stat::StatId;
use crate::status::Status;
use std::fmt::Display;

#[derive(Debug)]
pub struct Move {
    pub id: MoveId,
    pub pp: BoundedI32,
    pub damage_type: Mtype,
    pub poke_type: Type,
    pub base_power: Option<i32>,
    pub priority: i8,
    pub effects: Vec<Effect>,
    pub freq: Option<f32>,
    pub target: PlayerId,
    pub accuracy: Option<f32>,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\
            Type: {}\n\
            PP: {} / {}\n\
            {}{}",
            self.damage_type,
            self.poke_type,
            self.pp,
            self.pp.max,
            if let Some(data) = self.base_power {
                format!("BP: {}\n", data)
            } else {
                String::new()
            },
            if let Some(data) = self.accuracy {
                format!("Accuracy: {}%\n", (data * 100f32) as i32)
            } else {
                String::new()
            },
        )
    }
}

impl From<MoveId> for Move {
    fn from(id: MoveId) -> Self {
        match id {
            MoveId::Default => Self {
                id,
                pp: BoundedI32::new(24, 0, 24),
                damage_type: Mtype::Physical,
                poke_type: Type::Normal,
                base_power: None,
                priority: 0,
                effects: Vec::new(),
                freq: None,
                target: PlayerId::Inactive,
                accuracy: None,
            },
            MoveId::IronHead => Self {
                id,
                poke_type: Type::Steel,
                base_power: Some(80),
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Flinch)],
                freq: Some(0.3),
                ..Default::default()
            },
            MoveId::BodySlam => Self {
                id,
                base_power: Some(85),
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)],
                freq: Some(0.3),
                ..Default::default()
            },
            MoveId::Uturn => Self {
                id,
                pp: BoundedI32::new(32, 0, 32),
                damage_type: Mtype::Physical,
                poke_type: Type::Bug,
                base_power: Some(70),
                effects: vec![Effect::MidSwitch(PlayerId::Active)],
                ..Default::default()
            },
            MoveId::Stealthrock => Self {
                id,
                pp: BoundedI32::new(32, 0, 32),
                damage_type: Mtype::Status,
                poke_type: Type::Rock,
                effects: vec![Effect::InflictHazard(
                    PlayerId::Inactive,
                    HazardId::Stealthrock,
                )],
                ..Default::default()
            },
            MoveId::Thunderbolt => Self {
                id,
                damage_type: Mtype::Special,
                poke_type: Type::Electric,
                base_power: Some(95),
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)],
                freq: Some(0.1),
                ..Default::default()
            },
            MoveId::Roost => Self {
                id,
                pp: BoundedI32::new(16, 0, 16),
                damage_type: Mtype::Status,
                poke_type: Type::Flying,
                effects: vec![Effect::Heal(PlayerId::Active, 2)],
                ..Default::default()
            },
            MoveId::Hpice => Self {
                id,
                damage_type: Mtype::Special,
                poke_type: Type::Ice,
                base_power: Some(70),
                ..Default::default()
            },
            MoveId::Firepunch => Self {
                id,
                poke_type: Type::Fire,
                base_power: Some(75),
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Burn)],
                freq: Some(0.1),
                ..Default::default()
            },
            MoveId::Refresh => Self {
                id,
                pp: BoundedI32::new(32, 0, 32),
                damage_type: Mtype::Status,
                effects: vec![Effect::Cure(PlayerId::Active)],
                ..Default::default()
            },
            MoveId::Hydropump => Self {
                id,
                poke_type: Type::Water,
                damage_type: Mtype::Special,
                base_power: Some(120),
                accuracy: Some(0.8),
                ..Default::default()
            },
            MoveId::Thunderwave => Self {
                id,
                poke_type: Type::Electric,
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Paralyse)],
                ..Default::default()
            },
            MoveId::Icebeam => Self {
                id,
                damage_type: Mtype::Special,
                poke_type: Type::Ice,
                base_power: Some(95),
                ..Default::default()
            },
            MoveId::Rapidspin => Self {
                id,
                base_power: Some(20),
                pp: BoundedI32::new(64, 0, 64),
                effects: vec![Effect::ClearHazard(PlayerId::Active)],
                ..Default::default()
            },
            MoveId::Fireblast => Self {
                id,
                base_power: Some(120),
                damage_type: Mtype::Special,
                poke_type: Type::Fire,
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Burn)],
                ..Default::default()
            },
            MoveId::Earthpower => Self {
                id,
                base_power: Some(90),
                damage_type: Mtype::Special,
                poke_type: Type::Ground,
                effects: vec![Effect::AlterStat(PlayerId::Inactive, StatId::Spd, -1)],
                ..Default::default()
            },
            MoveId::Explosion => Self {
                id,
                base_power: Some(500),
                effects: vec![Effect::OHKO(PlayerId::Active)],
                ..Default::default()
            },
            MoveId::Crunch => Self {
                id,
                base_power: Some(80),
                poke_type: Type::Dark,
                effects: vec![Effect::AlterStat(PlayerId::Inactive, StatId::Def, -1)],
                ..Default::default()
            },
            MoveId::Pursuit => Self {
                id,
                base_power: Some(40),
                poke_type: Type::Dark,
                ..Default::default()
            },
            MoveId::Superpower => Self {
                id,
                base_power: Some(120),
                poke_type: Type::Fighting,
                effects: vec![
                    Effect::AlterStat(PlayerId::Active, StatId::Atk, -1),
                    Effect::AlterStat(PlayerId::Active, StatId::Def, -1),
                ],

                ..Default::default()
            },
            MoveId::Stoneedge => Self {
                id,
                base_power: Some(120),
                poke_type: Type::Rock,
                pp: BoundedI32 {
                    data: 8,
                    min: 0,
                    max: 8,
                },
                ..Default::default()
            },
            MoveId::Spore => Self {
                id,
                damage_type: Mtype::Status,
                poke_type: Type::Grass,
                effects: vec![Effect::InflictStatus(PlayerId::Inactive, Status::Sleep)],
                ..Default::default()
            },
            MoveId::Seedbomb => Self {
                ..Default::default()
            },
            MoveId::Machpunch => Self {
                ..Default::default()
            },
            MoveId::Struggle => Self {
                ..Default::default()
            },
            MoveId::Switch(_) => Self {
                ..Default::default()
            },
        }
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::from(MoveId::default())
    }
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum MoveId {
    #[default]
    Default,
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

impl Display for MoveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MoveId::Default => String::from("Default"),
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

impl Display for Mtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move Type: {}",
            match self {
                Mtype::Physical => String::from("Physical"),
                Mtype::Special => String::from("Special"),
                Mtype::Status => String::from("Status"),
            }
        )
    }
}
