use super::pokemon::Pokemon;
use super::status::Status;
use crate::effect::{Damage, Effect, PlayerId};

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Item {
    #[default]
    Leftovers,
    ChoiceScarf,
    ToxicOrb,
}

impl Pokemon {
    pub fn trigger_item(&self) -> Vec<Effect> {
        if let Some(item) = self.item {
            match item {
                Item::Leftovers => vec![Effect::Damage(
                    PlayerId::Active,
                    Damage::Fractional(-1.0, 16.0),
                )],
                Item::ToxicOrb => vec![Effect::InflictStatus(PlayerId::Active, Status::Toxic(0))],
                _ => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }

    pub fn has_item(&self, item: &Item) -> bool {
        if let Some(item_) = self.item {
            item_ == *item
        } else {
            false
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Ability {
    #[default]
    SandStream,
    SereneGrace,
    Flashfire,
    Poisonheal,
    NaturalCure,
    Pressure,
    Levitate,
}

impl Pokemon {
    pub fn has_ability(&self, ability: &Ability) -> bool {
        self.ability == *ability
    }
}
