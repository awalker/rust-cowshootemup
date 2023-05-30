use crate::{
    drawable::{HasCenter, HasVelocity},
    updateable::Updateable,
};

pub trait Movable: HasCenter + HasVelocity {}

impl<T: Movable> Updateable for T {
    fn update(&mut self, _delta_time: f32) {
        todo!()
    }
}
