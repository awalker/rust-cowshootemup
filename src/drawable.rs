use std::fmt::Debug;

use macroquad::{
    prelude::Color,
    shapes::{draw_circle, draw_line},
};

use crate::{world::RcWorld, CenterPt, Size, TopLeftPt};

pub trait HasCenter {
    fn center(&self) -> CenterPt;
}

pub trait HasTopLeft {
    fn top_left(&self) -> TopLeftPt;
}

pub trait HasSize {
    fn size(&self) -> Size;
}

pub trait Drawable: Debug {
    fn draw(&self, world: RcWorld);
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: Color,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Circle {
    center: CenterPt,
    radius: f32,
    color: Color,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            color,
        }
    }
}

impl Drawable for Line {
    fn draw(&self, _world: RcWorld) {
        draw_line(self.x1, self.y1, self.x2, self.y2, 1.0, self.color);
    }
}

impl Circle {
    pub fn new(center: CenterPt, r: f32, color: Color) -> Self {
        Self {
            center,
            radius: r,
            color,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, _world: RcWorld) {
        draw_circle(self.center.0, self.center.1, self.radius, self.color)
    }
}

#[derive(Debug, Clone)]
pub enum Graphic {
    Line(Line),
    Circle(Circle),
}

impl Drawable for Graphic {
    fn draw(&self, world: RcWorld) {
        match self {
            Graphic::Line(l) => l.draw(world),
            Graphic::Circle(c) => c.draw(world),
        }
    }
}

impl Graphic {
    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) -> Self {
        Graphic::Line(Line::new(x1, y1, x2, y2, color))
    }

    pub fn circle(center: CenterPt, radius: f32, color: Color) -> Self {
        Graphic::Circle(Circle::new(center, radius, color))
    }
}