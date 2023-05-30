use crate::{
    drawable::{HasCenter, HasVelocity, UpdateCenter},
    updateable::Updateable,
};

pub trait Movable {
    fn update_move(&mut self, delta_time: f32);
}

impl<T: Movable + UpdateCenter + HasCenter + HasVelocity> Movable for T {
    fn update_move(&mut self, delta_time: f32) {
        let vel = self.velocity() * delta_time;
        let center = self.center() + vel;
        self.update_center(center);
    }
}
impl<T: Movable> Updateable for T {
    fn update(&mut self, delta_time: f32) {
        self.update_move(delta_time)
    }
}
