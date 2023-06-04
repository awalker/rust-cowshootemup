use egui_macroquad::egui::{Slider, Ui};
use macroquad::rand::{gen_range, RandomRange};
use rand::{distributions::Standard, prelude::*};
use std::ops::{Add, Div, Mul, RangeInclusive, Rem, Sub};
#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
        + Clone
        + std::fmt::Display
        + RandomRange
        + egui_macroquad::egui::emath::Numeric,
    Standard: Distribution<T>,
{
    pub fn rand(&self) -> T {
        if self.min == self.max {
            return self.min;
        }
        gen_range(self.min, self.max)
    }

    pub fn append(mut self, v: T) -> Self {
        if v < self.min {
            self.min = v
        }
        if v > self.max {
            self.max = v
        }
        self
    }

    pub fn avg(&self) -> T {
        if self.min == self.max {
            return self.min;
        }
        let dist = self.max - self.min;
        self.min + dist / (2.0).into()
    }

    pub fn editor_ui(&mut self, ui: &mut Ui, range: RangeInclusive<T>) {
        ui.columns(2, |cols| {
            let (min, max) = range.into_inner();

            cols[0].add(Slider::new(&mut self.min, min..=self.max).max_decimals(2));
            cols[1].add(Slider::new(&mut self.max, self.min..=max).max_decimals(2));
        })
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
        + Clone
        + RandomRange
        + egui_macroquad::egui::emath::Numeric,
    Standard: Distribution<T>,
{
    pub fn rand_int(&self) -> T {
        if self.min == self.max {
            return self.min;
        }
        gen_range(self.min, self.max)
    }

    pub fn editor_int_ui(&mut self, ui: &mut Ui, range: RangeInclusive<T>) {
        ui.columns(2, |cols| {
            let (min, max) = range.into_inner();
            cols[0].add(Slider::new(&mut self.min, min..=self.max).max_decimals(0));
            cols[1].add(Slider::new(&mut self.max, self.min..=max).max_decimals(0));
        })
    }
}
