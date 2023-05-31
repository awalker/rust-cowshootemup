use crate::{
    alive::IsAlive, drawable::Drawable, timers::AliveTimer, updateable::Updateable, Accel,
    CenterPt, Velocity,
};
use macroquad::{prelude::Color, shapes::draw_circle};

use super::Particle;

#[derive(Default, Clone)]
pub struct CircleParticle {
    center: CenterPt,
    radius: f32,
    color: Color,
    velocity: Velocity,
    accel: Accel,
    visible: bool,
    alive_timer: Option<AliveTimer>,
}

impl std::fmt::Debug for CircleParticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CircleParticle")
            .field("center", &self.center)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("velocity", &self.velocity)
            .field("accel", &self.accel)
            .field("visible", &self.visible)
            .finish()
    }
}

impl CircleParticle {
    pub fn new(center: CenterPt, radius: f32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
            visible: true,
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

    pub fn with_ttl(mut self, v: f32) -> Self {
        self.alive_timer = Some(AliveTimer::new(v));
        self
    }
}

impl IsAlive for CircleParticle {
    fn is_alive(&self) -> bool {
        self.visible
    }
}

impl Particle for CircleParticle {}

impl Drawable for CircleParticle {
    fn draw(&self) {
        if self.visible {
            draw_circle(self.center.0, self.center.1, self.radius, self.color)
        }
    }
}

impl Updateable for CircleParticle {
    fn update(&mut self, delta_time: f32) {
        if let Some(timer) = &mut self.alive_timer {
            timer.update(delta_time);
            if !timer.is_alive() {
                self.visible = false;
            }
        }
        let vel = self.velocity * self.accel * delta_time;
        log::debug!("vel = {:?}", vel);
        self.center = self.center + vel;
    }
}
