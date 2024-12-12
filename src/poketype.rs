use std::fmt::Display;

use crate::pokemon::Pokemon;

/// Basically just a database of the type chart

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Poketype {
    Mono(Type),
    Dual([Type; 2]),
}

impl Default for Poketype {
    fn default() -> Self {
        Self::Mono(Default::default())
    }
}

impl Display for Poketype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Poketype::Mono(data) => format!("{}", data),
                Poketype::Dual(data) => format!("{} / {}", data[0], data[1]),
            }
        )
    }
}

impl Poketype {
    pub fn contains(&self, value: Type) -> bool {
        match self {
            Poketype::Mono(data) => value == *data,
            Poketype::Dual(data) => (value == data[0]) || (value == data[1]),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Type {
    #[default]
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Typeless,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}",)
    }
}

enum TypeEff {
    Immune,
    Ineffective,
    Neutral,
    Super,
}

impl From<TypeEff> for f32 {
    fn from(value: TypeEff) -> Self {
        match value {
            TypeEff::Immune => 0.0,
            TypeEff::Ineffective => 0.5,
            TypeEff::Neutral => 1.0,
            TypeEff::Super => 2.0,
        }
    }
}

impl Type {
    fn effectiveness(&self, target_type: &Type) -> TypeEff {
        match self {
            Type::Normal => match target_type {
                Type::Fighting => TypeEff::Super,
                Type::Ghost => TypeEff::Immune,
                _ => TypeEff::Neutral,
            },
            Type::Fire => match target_type {
                Type::Fire | Type::Water | Type::Rock | Type::Dragon => TypeEff::Ineffective,
                Type::Steel | Type::Grass | Type::Ice | Type::Bug => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Water => match target_type {
                Type::Dragon | Type::Water | Type::Grass => TypeEff::Ineffective,
                Type::Fire | Type::Ground | Type::Rock => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Electric => match target_type {
                Type::Water | Type::Flying => TypeEff::Super,
                Type::Dragon | Type::Electric | Type::Grass => TypeEff::Ineffective,
                Type::Ground => TypeEff::Immune,
                _ => TypeEff::Neutral,
            },
            Type::Grass => match target_type {
                Type::Fire
                | Type::Grass
                | Type::Poison
                | Type::Flying
                | Type::Bug
                | Type::Dragon => TypeEff::Ineffective,
                Type::Water | Type::Ground | Type::Rock => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Ice => match target_type {
                Type::Fire | Type::Water | Type::Ice | Type::Steel => TypeEff::Ineffective,
                Type::Grass | Type::Ground | Type::Flying | Type::Dragon => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Fighting => match target_type {
                Type::Poison | Type::Flying | Type::Psychic | Type::Bug => TypeEff::Ineffective,
                Type::Ghost => TypeEff::Immune,
                Type::Rock | Type::Dark | Type::Steel | Type::Normal | Type::Ice => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Poison => match target_type {
                Type::Grass => TypeEff::Super,
                Type::Poison | Type::Ground | Type::Rock | Type::Ghost | Type::Steel => {
                    TypeEff::Ineffective
                }
                _ => TypeEff::Neutral,
            },
            Type::Ground => match target_type {
                Type::Grass | Type::Bug => TypeEff::Ineffective,
                Type::Poison | Type::Rock | Type::Steel | Type::Fire | Type::Electric => {
                    TypeEff::Super
                }
                Type::Flying => TypeEff::Immune,
                _ => TypeEff::Neutral,
            },
            Type::Flying => match target_type {
                Type::Electric | Type::Rock | Type::Steel => TypeEff::Ineffective,
                Type::Grass | Type::Fighting | Type::Bug => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Psychic => match target_type {
                Type::Fighting | Type::Poison => TypeEff::Super,
                Type::Psychic | Type::Steel => TypeEff::Ineffective,
                Type::Dark => TypeEff::Immune,
                _ => TypeEff::Neutral,
            },
            Type::Bug => match target_type {
                Type::Grass | Type::Psychic => TypeEff::Super,
                Type::Fighting
                | Type::Poison
                | Type::Flying
                | Type::Ghost
                | Type::Steel
                | Type::Fire => TypeEff::Ineffective,
                Type::Dark => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Rock => match target_type {
                Type::Fire | Type::Ice => TypeEff::Super,
                Type::Fighting | Type::Ground | Type::Steel => TypeEff::Ineffective,
                Type::Flying | Type::Bug => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Ghost => match target_type {
                Type::Normal => TypeEff::Immune,
                Type::Psychic | Type::Ghost => TypeEff::Super,
                Type::Dark | Type::Steel => TypeEff::Ineffective,
                _ => TypeEff::Neutral,
            },
            Type::Dragon => match target_type {
                Type::Dragon => TypeEff::Super,
                Type::Steel => TypeEff::Ineffective,
                _ => TypeEff::Neutral,
            },
            Type::Dark => match target_type {
                Type::Psychic | Type::Ghost => TypeEff::Super,
                Type::Dark | Type::Steel | Type::Fighting => TypeEff::Ineffective,
                _ => TypeEff::Neutral,
            },
            Type::Steel => match target_type {
                Type::Fire | Type::Water | Type::Electric | Type::Steel => TypeEff::Ineffective,
                Type::Ice | Type::Rock => TypeEff::Super,
                _ => TypeEff::Neutral,
            },
            Type::Typeless => TypeEff::Neutral,
        }
    }

    pub fn calc_eff(&self, defending_type: &Poketype) -> f32 {
        match defending_type {
            Poketype::Mono(target) => f32::from(self.effectiveness(&target)),
            Poketype::Dual(targets) => {
                f32::from(self.effectiveness(&targets[0]))
                    * f32::from(self.effectiveness(&targets[1]))
            }
        }
    }
}

impl Pokemon {
    pub fn has_type(&self, target: &Type) -> bool {
        match *self.poketype.borrow() {
            Poketype::Mono(type_) => *target == type_,
            Poketype::Dual(types) => *target == types[0] || *target == types[1],
        }
    }

    pub fn type_eff(&self, target: &Type) -> f32 {
        target.calc_eff(&self.poketype.borrow())
    }
}
