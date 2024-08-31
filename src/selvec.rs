use std::ops::{Index, IndexMut};

use crate::{
    moves::Move,
    trigger::{Ability, Item},
    EmptyResult,
};

#[derive(Debug, Default)]
pub struct SelVec<T> {
    active: Option<usize>,
    data: Vec<T>,
    pub dead: usize,
    pub lock: bool,
    selection: usize,
}

impl<T> From<Vec<T>> for SelVec<T>
where
    T: Default,
{
    fn from(value: Vec<T>) -> Self {
        Self {
            dead: value.len(),
            data: value,
            ..Default::default()
        }
    }
}

impl<T> Index<usize> for SelVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for SelVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> SelVec<T> {
    pub fn kill(&mut self) -> EmptyResult {
        self.data.swap(self.selection, self.dead - 1);
        self.dead -= 1;
        self.active = None;
        Ok(())
    }

    pub fn deactivate(&mut self) -> EmptyResult {
        self.active = None;
        Ok(())
    }
}

impl SelVec<Move> {
    pub fn activate(&mut self, item: Item, ability: Ability) -> Result<(), &'static str> {
        if self.lock && self.selection != self.active.expect("Locked without active move") {
            return Err("Locked into different move");
        }

        if self.selection < self.dead {
            self.active = Some(self.selection);
            self.data[self.selection].pp -= if ability == Ability::Pressure { 2 } else { 1 };
            self.lock = item.is_choice();
            Ok(())
        } else {
            Err("Selected move has no pp")
        }
    }
}

// impl SelVec<Pokemon> {
//     pub fn activate(&mut self, )
// }
