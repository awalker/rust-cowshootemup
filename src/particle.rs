use macroquad::{prelude::Color, shapes::draw_circle};

use crate::{
    drawable::Drawable, impl_pts, movable::Movable, updateable::Updateable, Accel, CenterPt,
    Velocity,
};

pub trait Particle: Drawable + Updateable {}

#[derive(Default, Debug)]
pub struct CircleParticle {
    center: CenterPt,
    radius: f32,
    color: Color,
    velocity: Velocity,
    accel: Accel,
}

impl CircleParticle {
    pub fn new(center: CenterPt, radius: f32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
            ..Default::default()
        }
    }

    pub fn with_velocity(mut self, v: Velocity) -> Self {
        self.velocity = v;
        self
    }

    pub fn with_accel(mut self, v: Accel) -> Self {
        self.accel = v;
        self
    }
}

impl Drawable for CircleParticle {
    fn draw(&self) {
        draw_circle(self.center.0, self.center.1, self.radius, self.color)
    }
}

impl Updateable for CircleParticle {
    fn update(&mut self, delta_time: f32) {
        self.update_move(delta_time);
    }
}

impl_pts!( center CircleParticle);
impl_pts!( velocity CircleParticle);
impl_pts!( accel CircleParticle);
