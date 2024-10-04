use core::fmt;
use std::{
    cell::RefCell,
    cmp::Ordering,
    ops::{Index, IndexMut},
    rc::Rc,
};

use crate::{
    bounded_i32::BoundedI32,
    game::WeatherId,
    poketype::{Poketype, Type},
    status::{Status, StatusBlock},
    trigger::Item,
    EmptyResult,
};

#[derive(Debug, Default, Clone)]
pub struct Stat {
    id: StatId,
    base: i32,
    stage: BoundedI32,
    weather: Rc<RefCell<Option<WeatherId>>>,
    poketype: Rc<RefCell<Poketype>>,
    item: Rc<RefCell<Option<Item>>>,
    status: Rc<RefCell<StatusBlock>>,
}

#[derive(Debug, Default, Clone)]
pub struct StatBlock {
    atk: Stat,
    def: Stat,
    spa: Stat,
    spd: Stat,
    spe: Stat,
}

impl StatBlock {
    pub fn new(
        values: [i32; 5],
        poketype: Rc<RefCell<Poketype>>,
        item: Rc<RefCell<Option<Item>>>,
        weather: Rc<RefCell<Option<WeatherId>>>,
        status: Rc<RefCell<StatusBlock>>,
    ) -> Self {
        Self {
            atk: Stat::new(
                values[0],
                StatId::Atk,
                Rc::clone(&poketype),
                Rc::clone(&item),
                Rc::clone(&weather),
                Rc::clone(&status),
            ),
            def: Stat::new(
                values[1],
                StatId::Def,
                Rc::clone(&poketype),
                Rc::clone(&item),
                Rc::clone(&weather),
                Rc::clone(&status),
            ),
            spa: Stat::new(
                values[2],
                StatId::Spa,
                Rc::clone(&poketype),
                Rc::clone(&item),
                Rc::clone(&weather),
                Rc::clone(&status),
            ),
            spd: Stat::new(
                values[3],
                StatId::Spd,
                Rc::clone(&poketype),
                Rc::clone(&item),
                Rc::clone(&weather),
                Rc::clone(&status),
            ),
            spe: Stat::new(values[4], StatId::Spe, poketype, item, weather, status),
            ..Default::default()
        }
    }
}

impl Index<StatId> for StatBlock {
    type Output = Stat;
    fn index(&self, index: StatId) -> &Self::Output {
        match index {
            StatId::Atk => &self.atk,
            StatId::Def => &self.def,
            StatId::Spa => &self.spa,
            StatId::Spd => &self.spd,
            StatId::Spe => &self.spe,
        }
    }
}

impl IndexMut<StatId> for StatBlock {
    fn index_mut(&mut self, index: StatId) -> &mut Self::Output {
        match index {
            StatId::Atk => &mut self.atk,
            StatId::Def => &mut self.def,
            StatId::Spa => &mut self.spa,
            StatId::Spd => &mut self.spd,
            StatId::Spe => &mut self.spe,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum StatId {
    #[default]
    Atk,
    Def,
    Spa,
    Spd,
    Spe,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:>+0width$}", self.curr(), self.stage, width = 2)
    }
}

impl fmt::Display for StatId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatId::Atk => write!(f, "ATK"),
            StatId::Def => write!(f, "DEF"),
            StatId::Spa => write!(f, "SPA"),
            StatId::Spd => write!(f, "SPD"),
            StatId::Spe => write!(f, "SPE"),
        }
    }
}

impl Stat {
    pub fn new(
        value: i32,
        id: StatId,
        poketype: Rc<RefCell<Poketype>>,
        item: Rc<RefCell<Option<Item>>>,
        weather: Rc<RefCell<Option<WeatherId>>>,
        status: Rc<RefCell<StatusBlock>>,
    ) -> Self {
        Self {
            id,
            base: value,
            stage: BoundedI32 {
                data: 0,
                min: -6,
                max: 6,
            },
            weather,
            poketype,
            item,
            status,
        }
    }

    pub fn curr(&self) -> i32 {
        let mut out = match self.stage.data.cmp(&0) {
            Ordering::Less => self.base / (2 - self.stage.data) * 2,
            Ordering::Equal => self.base,
            Ordering::Greater => self.base * ((2 + self.stage.data) / 2),
        };

        match self.id {
            StatId::Spe => {
                if *self.item.borrow() == Some(Item::ChoiceScarf) {
                    out = (out * 3) / 2
                }

                if self.status.borrow().data.contains_key(&Status::Paralyse) {
                    out /= 4
                }
            }
            StatId::Spd => {
                if *self.weather.borrow() == Some(WeatherId::Sand) {
                    out = (out * 3) / 2
                }
            }
            _ => {}
        }

        out
    }

    pub fn alter(&mut self, diff: i32) -> bool {
        let prev = self.stage.data;
        self.stage += diff;
        prev == self.stage.data
    }
}
