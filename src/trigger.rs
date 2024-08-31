#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Item {
    #[default]
    Leftovers,
    ChoiceScarf,
    ToxicOrb,
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
