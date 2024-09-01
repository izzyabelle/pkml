use crate::game::HazardId;
use crate::pokemon::Pokemon;
use crate::preset::PokeId;
use crate::selvec::PointerVec;

#[derive(Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub ai: bool,
    pub hazards: Vec<HazardId>,
    pub roster: PointerVec<Pokemon>,
}

impl Player {
    pub fn new(ai: bool) -> Self {
        Self {
            name: "test",
            ai,
            hazards: Vec::new(),
            roster: PointerVec::from(vec![
                Pokemon::from(PokeId::Jirachi),
                Pokemon::from(PokeId::Tyranitar),
                Pokemon::from(PokeId::Heatran),
                Pokemon::from(PokeId::Breloom),
                Pokemon::from(PokeId::Zapdos),
                Pokemon::from(PokeId::Starmie),
            ]),
        }
    }
}
