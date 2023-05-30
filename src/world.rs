use std::cell::RefCell;

// use serde::{Deserialize, Serialize};

use crate::{
    drawable::{Drawable, Graphic},
    particle::Particle,
    updateable::Updateable,
    Rc,
};

#[derive(Default /*, Serialize, Deserialize*/)]
pub struct World {
    graphics: Vec<Graphic>,
    particles: Vec<Box<dyn Particle>>,
}

impl World {
    pub fn add_graphic(&mut self, d: Graphic) {
        self.graphics.push(d)
    }

    pub fn add_particle(&mut self, d: Box<dyn Particle>) {
        self.particles.push(d)
    }
}

impl Drawable for World {
    fn draw(&self) {
        self.graphics.iter().for_each(|g| g.draw());
        self.particles.iter().for_each(|p| p.draw());
    }
}

impl Updateable for World {
    fn update(&mut self, delta_time: f32) {
        self.particles.iter_mut().for_each(|p| p.update(delta_time))
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World").finish()
    }
}

pub type RcWorld = Rc<RefCell<World>>;
