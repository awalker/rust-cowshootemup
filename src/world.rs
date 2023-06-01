use std::cell::RefCell;

// use serde::{Deserialize, Serialize};

use crate::{
    drawable::{Drawable, Gizmo, Graphic},
    particle::Particle,
    updateable::Updateable,
    Rc,
};

#[derive(Default /*, Serialize, Deserialize*/)]
pub struct World {
    graphics: Vec<Graphic>,
    particles: Vec<Box<dyn Particle>>,
    gizmos: Vec<Rc<dyn Gizmo>>,
}

impl World {
    pub fn add_graphic(&mut self, d: Graphic) {
        self.graphics.push(d)
    }

    pub fn add_particle(&mut self, d: Box<dyn Particle>) {
        self.particles.push(d)
    }

    pub fn add_gizmos(&mut self, d: Rc<dyn Gizmo>) {
        self.gizmos.push(d);
    }
}

impl Drawable for World {
    fn draw(&self) {
        self.graphics.iter().for_each(|g| g.draw());
        self.particles.iter().for_each(|p| p.draw());
    }

    fn draw_gizmos(&self) {
        self.graphics.iter().for_each(|g| g.draw_gizmos());
        self.particles.iter().for_each(|p| p.draw_gizmos());
        self.gizmos.iter().for_each(|p| p.draw_gizmos());
    }
}

impl Updateable for World {
    fn update(&mut self, delta_time: f32) {
        self.particles.iter_mut().for_each(|p| p.update(delta_time))
        // TODO: Remove dead particles...
        // TODO: Remove dead gizmos...
        // TODO: gizmos might need updating too...
        // self.gizmos.iter_mut()
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World").finish()
    }
}

pub type RcWorld = Rc<RefCell<World>>;

impl Updateable for RcWorld {
    fn update(&mut self, delta_time: f32) {
        let mut world = (*self).borrow_mut();
        world.update(delta_time);
    }
}

impl Drawable for RcWorld {
    fn draw(&self) {
        self.borrow().draw();
    }
    fn draw_gizmos(&self) {
        self.borrow().draw_gizmos();
    }
}
