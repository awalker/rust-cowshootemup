mod alive;
use std::{f32::consts::PI, todo};

use crate::{alive::IsAlive, drawable::Drawable, updateable::Updateable, CenterPt, Velocity};

pub use alive::CircleParticle;

pub trait Particle: Drawable + Updateable + IsAlive {
    // fn ttl(&self) -> f32;
}

pub trait AliveUpdatable: Updateable + IsAlive {}

#[derive(Debug, Default)]
pub struct Explosion {
    circles: Vec<CircleParticle>,

    center: CenterPt,
    // lines, sparks, or whatever
}

#[derive(Debug, Default, Clone)]
pub struct ExplosionBuilder {
    center: CenterPt,
    velocity: Velocity,
    stage_time: f32,
    circles_per_stage: u8,
    angle_max: f32,
    angle_min: f32,
    dist_max: f32,
    dist_min: f32,
    circles: Vec<CircleParticle>,
}

impl ExplosionBuilder {
    pub fn at(mut self, center: CenterPt) -> Self {
        self.center = center;
        self
    }

    pub fn with_velocity(mut self, v: Velocity) -> Self {
        self.velocity = v;
        self
    }

    /// Set the min/max angle for random balls in the next
    /// stage of the explosion
    pub fn with_angle(mut self, min: f32, max: f32) -> Self {
        self.angle_min = min;
        self.angle_max = max;
        self
    }

    /// Set the min/max dist for random balls in the next
    /// stage of the explosion
    pub fn with_dist(mut self, min: f32, max: f32) -> Self {
        self.dist_min = min;
        self.dist_max = max;
        self
    }

    /// Add a stage of the explosion
    pub fn with_stage(mut self, time: f32, count: u8) -> Self {
        todo!();
        self
    }

    pub fn build(self) -> Explosion {
        Explosion {
            circles: self.circles,
            center: self.center,
        }
    }
}

impl Explosion {
    pub fn new(center: CenterPt, velocity: Velocity) -> ExplosionBuilder {
        ExplosionBuilder {
            velocity: Velocity(0., -2.),
            angle_max: PI * 2.,
            ..Default::default()
        }
    }
}
