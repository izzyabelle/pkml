use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone, Default)]
pub struct BoundedI32 {
    pub data: i32,
    pub min: i32,
    pub max: i32,
}

impl BoundedI32 {
    pub fn new(data: i32, min: i32, max: i32) -> Self {
        if min > max {
            panic!("min cannot be greater than max")
        } else if data < min {
            Self {
                data: min,
                min,
                max,
            }
        } else if data > max {
            Self {
                data: max,
                min,
                max,
            }
        } else {
            Self { data, min, max }
        }
    }

    pub fn zero(data: i32) -> Self {
        BoundedI32::new(data, 0, data)
    }

    pub fn is_max(&self) -> bool {
        self.data == self.max
    }
}

impl Display for BoundedI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Add<i32> for BoundedI32 {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self::new(self.data + rhs, self.min, self.max)
    }
}

impl AddAssign<i32> for BoundedI32 {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub<i32> for BoundedI32 {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign<i32> for BoundedI32 {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl PartialEq<i32> for BoundedI32 {
    fn eq(&self, other: &i32) -> bool {
        other == &self.data
    }
}
