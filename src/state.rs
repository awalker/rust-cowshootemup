use std::rc::Rc;

use crate::world::World;

pub trait State {
    fn world(&self) -> Rc<World>;
    fn update(&mut self);
    fn draw(&self);
}
