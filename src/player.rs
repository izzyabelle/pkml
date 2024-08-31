use crate::game::HazardId;
use crate::pokemon::Pokemon;

#[derive(Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub ai: bool,
    pub hazards: Vec<HazardId>,
    pub active: Option<Pokemon>,
    pub roster: Vec<Pokemon>,
    pub dead: Vec<Pokemon>,
}
