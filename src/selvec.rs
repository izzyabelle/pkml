use std::{
    fmt::Error,
    ops::{Index, IndexMut},
};

use crate::{trigger::Item, EmptyResult};

pub enum Direction {
    Up,
    Down,
}

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
    pub fn shift(&mut self, dir: Direction) -> EmptyResult<()> {
        match dir {
            Direction::Up => {
                if self.selection != self.data.len() - 1 {
                    self.selection += 1;
                }
            }
            Direction::Down => {
                if self.selection != 0 {
                    self.selection -= 1;
                }
            }
        }
        Ok(())
    }

    pub fn kill(&mut self) -> EmptyResult<()> {
        self.data.swap(self.selection, self.dead - 1);
        self.dead -= 1;
        self.active = None;
        Ok(())
    }
}

impl<Move> SelVec<Move> {
    pub fn activate(&mut self, item: Item) -> EmptyResult<()> {
        if self.lock && self.selection != self.active {
            return Result::Err();
        }
        if self.selection < self.dead && !self.lock {
            self.active = Some(self.selection);
            Ok(())
        } else {
            Error
        }
    }
}
