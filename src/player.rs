use crate::game::HazardId;
use crate::moves::MoveId;
use crate::pokemon::Pokemon;
use crate::preset::PokeId;
use crate::Result;

type Layers = u8;

#[derive(Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub ai: bool,
    pub hazards: Vec<Hazard>,
    pub active: Option<Pokemon>,
    pub roster: Vec<Pokemon>,
    pub dead: Vec<Pokemon>,
}

impl Player {
    pub fn use_pp(&mut self) -> Result<()> {
        let active_move = self.get_active_move_ind();
        if active_move > 5 {
            self.get_active_mut().use_move(active_move - 6);
        }
        Ok(())
    }

    pub fn set_active_index(&mut self, new: usize) -> Result<()> {
        self.active_index = new;
        Ok(())
    }

    pub fn switch(&mut self, selection: usize) -> Result<()> {
        if !self.team[selection].is_alive() {
            panic!("Invalid switch");
        }
        self.set_active_index(selection).unwrap();
        Ok(())
    }

    pub fn clear_hazards(&mut self) -> Result<()> {
        for i in 0..3 {
            self.hazards[i] = 0;
        }
        Ok(())
    }

    pub fn inc_hazard(&mut self, hazard: HazardId) -> Result<()> {
        if self.hazards[hazard.index()] == hazard.max_layers() {
            panic!("hazard incremented beyond bounds");
        }
        self.hazards[hazard.index()] += 1;
        Ok(())
    }

    pub fn set_lock(&mut self) -> Result<()> {
        self.choice_lock = Some(self.active_move);
        Ok(())
    }

    pub fn release_lock(&mut self) -> Result<()> {
        self.choice_lock = None;
        Ok(())
    }
}
