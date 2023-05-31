mod alive;
use crate::{
    alive::IsAlive, drawable::Drawable, minmax::MinMax, updateable::Updateable, CenterPt, Velocity,
};
pub use alive::CircleParticle;
use macroquad::prelude::YELLOW;
use std::f32::consts::PI;

pub trait Particle: Drawable + Updateable + IsAlive {
    // fn ttl(&self) -> f32;
}

pub trait AliveUpdatable: Updateable + IsAlive {}

#[derive(Default)]
pub struct Explosion {
    circles: Vec<CircleParticle>,
    // lines, sparks, or whatever
}

#[derive(Debug, Default, Clone)]
pub struct ExplosionBuilder {
    center: CenterPt,
    velocity: Velocity,
    stage_time: MinMax<f32>,
    circles_per_stage: MinMax<u8>,
    /// a random angle from the center at which to spawn particles
    angle: MinMax<f32>,
    /// a random distance from the center at which to spawn particles
    dist: MinMax<f32>,
    /// How large is the particle (circle/spark)
    radius: MinMax<f32>,
    delay: MinMax<f32>,
    /// We currently pregenerating all the circles, maybe we should store the stages and generate
    /// on build
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
        self.angle = MinMax::new(min, max);
        self
    }

    /// Set the min/max dist for random balls in the next
    /// stage of the explosion
    pub fn with_dist(mut self, min: f32, max: f32) -> Self {
        self.dist = MinMax::new(min, max);
        self
    }

    /// Set the min/max radius for random balls in the next
    /// stage of the explosion
    pub fn with_radius(mut self, min: f32, max: f32) -> Self {
        self.radius = MinMax::new(min, max);
        self
    }

    /// Set the min/max delay for random balls in the next
    /// stage of the explosion
    pub fn with_delay(mut self, min: f32, max: f32) -> Self {
        self.delay = MinMax::new(min, max);
        self
    }

    pub fn with_count(mut self, min: u8, max: u8) -> Self {
        self.circles_per_stage = MinMax::new(min, max);
        self
    }

    pub fn with_age(mut self, min: f32, max: f32) -> Self {
        self.stage_time = MinMax::new(min, max);
        self
    }

    /// Add a stage of the explosion
    pub fn with_circle_stage(mut self) -> Self {
        assert!(self.stage_time.max != 0., "Max Stage time can not be zero!");
        let mut time = MinMax {
            min: f32::MAX,
            max: f32::MIN,
        };
        let desired_circles = self.circles_per_stage.rand_int();
        for _i in 0..desired_circles {
            let t = self.stage_time.rand();
            let d = self.delay.rand();
            time = time.append(t + d);
            let center = self.center;
            let velocity = self.velocity;
            let cp = CircleParticle::new(center, self.radius.rand(), YELLOW)
                .with_ttl(t)
                .with_delay(d)
                .with_velocity(velocity);
            self.circles.push(cp);
        }
        let time = time.avg();
        self.with_delay(time * 0.4, time * 0.8)
    }

    pub fn build(self) -> Explosion {
        assert!(self.circles.is_empty());
        Explosion {
            circles: self.circles,
            // center: self.center,
            // ..Default::default()
        }
    }
}

impl Explosion {
    pub fn begin(center: CenterPt, velocity: Velocity) -> ExplosionBuilder {
        ExplosionBuilder {
            center,
            velocity,
            circles_per_stage: MinMax::new(1, 1),
            radius: MinMax::new(10., 10.),
            angle: MinMax::new(0., PI * 2.),
            ..Default::default()
        }
    }
}

impl Drawable for Explosion {
    fn draw(&self) {
        self.circles.iter().for_each(|c| c.draw())
    }
}

impl Updateable for Explosion {
    fn update(&mut self, delta_time: f32) {
        self.circles.iter_mut().for_each(|c| c.update(delta_time))
    }
}

impl IsAlive for Explosion {
    fn is_alive(&self) -> bool {
        self.circles.iter().any(|c| c.is_alive())
    }
}

impl AliveUpdatable for Explosion {}

impl Particle for Explosion {}
