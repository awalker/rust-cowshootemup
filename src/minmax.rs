use rand::{distributions::Standard, prelude::*};
use std::ops::{Add, Div, Mul, Rem, Sub};
#[derive(Debug, Default, Clone, Copy)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

impl<T> MinMax<T>
where
    T: PartialOrd,
{
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T> MinMax<T>
where
    f32: std::convert::From<T>,
    T: std::convert::From<f32>
        + PartialEq
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Add<Output = T>
        + PartialOrd
        + Clone,
    Standard: Distribution<T>,
{
    pub fn rand(&self) -> T {
        if self.min == self.max {
            return self.min.clone();
        }
        let dist = self.max.clone() - self.min.clone();
        let r = rand::random::<T>() * dist;
        self.min.clone() + r
    }

    pub fn append(mut self, v: T) -> Self {
        if v < self.min {
            self.min = v.clone()
        }
        if v > self.max {
            self.max = v
        }
        self
    }

    pub fn avg(&self) -> T {
        if self.min == self.max {
            return self.min.clone();
        }
        let dist = self.max.clone() - self.min.clone();
        self.min.clone() + dist / (2.0).into()
    }
}
impl<T> MinMax<T>
where
    u8: std::convert::From<T>,
    T: std::convert::From<u8>
        + PartialEq
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Add<Output = T>
        + Clone,
    Standard: Distribution<T>,
{
    pub fn rand_int(&self) -> T {
        if self.min == self.max {
            return self.min.clone();
        }
        let dist = self.max.clone() - self.min.clone();
        let r = rand::random::<T>() % dist;
        self.min.clone() + r
    }
}
