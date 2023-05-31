use crate::{
    alive::IsAlive, drawable::Drawable, updateable::Updateable, Accel, CenterPt, Velocity,
};
use macroquad::{prelude::Color, shapes::draw_circle};

use super::Particle;

#[derive(Default, Clone, Debug)]
pub struct CircleParticle {
    center: CenterPt,
    radius: f32,
    color: Color,
    velocity: Velocity,
    accel: Accel,
    ttl: f32,
    delay: f32,
}

impl CircleParticle {
    pub fn new(center: CenterPt, radius: f32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
            ttl: 5.0,
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

    pub fn with_delay(mut self, v: f32) -> Self {
        self.delay = v;
        self
    }

    pub fn with_ttl(mut self, v: f32) -> Self {
        self.ttl = v;
        self
    }

    pub fn is_visible(&self) -> bool {
        self.ttl > 0. && self.delay <= 0.
    }
}

impl IsAlive for CircleParticle {
    fn is_alive(&self) -> bool {
        self.delay >= 0. && self.ttl > 0.
    }
}

impl Particle for CircleParticle {}

impl Drawable for CircleParticle {
    fn draw(&self) {
        if self.is_visible() {
            draw_circle(self.center.0, self.center.1, self.radius, self.color)
        }
    }
}

impl Updateable for CircleParticle {
    fn update(&mut self, delta_time: f32) {
        if self.delay > 0. {
            self.delay -= delta_time;
        } else if self.ttl > 0. {
            self.ttl -= delta_time;
            let vel = self.velocity * self.accel * delta_time;
            log::debug!("vel = {:?}", vel);
            self.center = self.center + vel;
        }
    }
}
