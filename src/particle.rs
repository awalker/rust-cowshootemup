mod circle;
use crate::{
    alive::IsAlive,
    drawable::{Drawable, Gizmo},
    minmax::MinMax,
    updateable::Updateable,
    CenterPt, Velocity,
};
pub use circle::CircleParticle;
use macroquad::{
    prelude::{Color, GREEN, YELLOW},
    shapes::draw_line,
};
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

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplosionStage {
    pub center: CenterPt,
    pub velocity: Velocity,
    pub stage_time: MinMax<f32>,
    pub circles_per_stage: MinMax<u8>,
    /// a random angle from the center at which to spawn particles
    pub angle: MinMax<f32>,
    /// a random distance from the center at which to spawn particles
    pub dist: MinMax<f32>,
    /// How large is the particle (circle/spark)
    pub radius: MinMax<f32>,
    pub delay: MinMax<f32>,
    #[serde(with = "crate::utils::color_format")]
    pub color: Color,
}

#[derive(Debug, Default, Clone)]
pub struct ExplosionBuilder {
    current: ExplosionStage,
    /// We currently pregenerating all the circles, maybe we should store the stages and generate
    /// on build
    stages: Vec<ExplosionStage>,
}

impl ExplosionStage {
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

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn generate_circle_particles(&self) -> Vec<CircleParticle> {
        let desired_circles = self.circles_per_stage.rand_int();
        let mut circles = Vec::new();
        for _i in 0..desired_circles {
            let t = self.stage_time.rand();
            let d = self.delay.rand();
            let (cx, cy) = self.center.into();
            let (vx, vy) = self.velocity.into();
            let (ax, ay) = self.angle.rand().sin_cos();
            let r = self.dist.rand();

            let cp = CircleParticle::new(
                (cx + ax * r, cy + ay * r).into(),
                self.radius.rand(),
                self.color,
            )
            .with_ttl(t)
            .with_delay(d)
            .with_velocity((vx + ax * r, vy + ay * r).into());
            circles.push(cp);
        }
        circles
    }

    pub fn with_circle_stage(self, b: &mut ExplosionBuilder) -> Self {
        b.current = self.clone();
        assert!(self.stage_time.max != 0., "Max Stage time can not be zero!");
        b.stages.push(self.clone());
        self
    }
}

impl ExplosionBuilder {
    pub fn build_stage(&self) -> ExplosionStage {
        self.current.clone()
    }

    /// Add a stage of the explosion
    pub fn with_circle_stage(mut self, stage: ExplosionStage) -> Self {
        self.current = stage.clone();
        assert!(
            stage.stage_time.max != 0.,
            "Max Stage time can not be zero!"
        );
        self.stages.push(stage);
        self
    }

    pub fn build(self) -> Explosion {
        assert!(!self.stages.is_empty());
        Explosion {
            circles: self
                .stages
                .iter()
                .flat_map(|f| f.generate_circle_particles())
                .collect(),
            // center: self.center,
            // ..Default::default()
        }
    }
}

impl Explosion {
    pub fn begin(center: CenterPt) -> ExplosionBuilder {
        ExplosionBuilder {
            current: ExplosionStage {
                center,
                circles_per_stage: MinMax::new(1, 1),
                radius: MinMax::new(10., 10.),
                angle: MinMax::new(0., PI * 2.),
                color: YELLOW,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Drawable for Explosion {
    fn draw(&self) {
        self.circles.iter().for_each(|c| c.draw())
    }
}

impl Drawable for ExplosionStage {
    fn draw(&self) {}
    fn draw_gizmos(&self) {
        let color = GREEN;
        let r = self.radius.max;
        let ang = self.angle.min;
        let (arc1x, arc1y) = ang.sin_cos();
        let (arc2x, arc2y) = self.angle.max.sin_cos();
        let (cx, cy) = self.center.into();
        draw_line(cx, cy, cx + arc1x * r, cy + arc1y * r, 1., color);
        draw_line(cx, cy, cx + arc2x * r, cy + arc2y * r, 1., color);
        draw_line(
            cx + arc1x * r,
            cy + arc1y * r,
            cx + arc2x * r,
            cy + arc2y * r,
            1.,
            color,
        );
    }
}

impl IsAlive for ExplosionStage {
    fn is_alive(&self) -> bool {
        true
    }
}

impl Gizmo for ExplosionStage {}

impl Updateable for Explosion {
    fn update(&mut self, delta_time: f32) {
        self.circles.iter_mut().for_each(|c| c.update(delta_time));
        if self.circles.iter().any(|c| !c.is_alive()) {
            self.circles = self
                .circles
                .clone()
                .into_iter()
                .filter(|c| c.is_alive())
                .collect::<Vec<_>>()
        }
    }
}

impl IsAlive for Explosion {
    fn is_alive(&self) -> bool {
        self.circles.iter().any(|c| c.is_alive())
    }
}

impl AliveUpdatable for Explosion {}

impl Particle for Explosion {}
