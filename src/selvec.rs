use std::ops::{Index, IndexMut};

use crate::{
    player::Player,
    EmptyResult,
};

#[derive(Debug, Clone, Default)]
pub struct PointerVec<T> {
    pub active: Option<usize>,
    pub data: Vec<T>,
    pub dead: usize,
}

impl<T> From<Vec<T>> for PointerVec<T>
where
    T: Default,
{
    fn from(value: Vec<T>) -> Self {
        Self {
            dead: value.len(),
            data: value,
            active: Some(0),
            ..Default::default()
        }
    }
}

impl<T> Index<usize> for PointerVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for PointerVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
#[repr(usize)]
pub enum PlayerId {
    #[default]
    Active,
    Inactive,
}

impl Index<PlayerId> for PointerVec<Player> {
    type Output = Player;

    fn index(&self, index: PlayerId) -> &Self::Output {
        let active = self.active.expect("No active player");

        match index {
            PlayerId::Active => &self.data[active],
            PlayerId::Inactive => &self.data[(active + 1) % 2],
        }
    }
}

impl IndexMut<PlayerId> for PointerVec<Player> {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        let active = self.active.expect("No active player");

        match index {
            // unwrap because there is always an active player
            PlayerId::Active => &mut self.data[active],
            PlayerId::Inactive => &mut self.data[(active + 1) % 2],
        }
    }
}

impl<T> PointerVec<T> {
    pub fn active(&self) -> Option<&T> {
        if let Some(data) = self.active {
            Some(&self[data])
        } else {
            None
        }
    }

    pub fn active_mut(&mut self) -> Option<&mut T> {
        if let Some(data) = self.active {
            Some(&mut self[data])
        } else {
            None
        }
    }

    pub fn kill(&mut self) {
        self.data.swap(self.active.unwrap(), self.dead - 1);
        self.dead -= 1;
        self.active = None;
    }

    pub fn deactivate(&mut self) -> EmptyResult {
        self.active = None;
        Ok(())
    }

    pub fn living(&self) -> &[T] {
        &self.data[..self.dead]
    }
}
