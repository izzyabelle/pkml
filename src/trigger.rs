use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Item {
    #[default]
    Leftovers,
    ChoiceScarf,
    ToxicOrb,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Item::Leftovers => String::from("Leftovers"),
                Item::ChoiceScarf => String::from("Choice Scarf"),
                Item::ToxicOrb => String::from("Toxic Orb"),
            }
        )
    }
}

impl Item {
    pub fn is_choice(&self) -> bool {
        match self {
            Item::ChoiceScarf => true,
            _ => false,
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

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ability::SandStream => String::from("Sand Stream"),
                Ability::SereneGrace => String::from("Serene Grace"),
                Ability::Flashfire => String::from("Flash Fire"),
                Ability::Poisonheal => String::from("Poison Heal"),
                Ability::NaturalCure => String::from("Natural Cure"),
                Ability::Pressure => String::from("Pressure"),
                Ability::Levitate => String::from("Levitate"),
            }
        )
    }
}
