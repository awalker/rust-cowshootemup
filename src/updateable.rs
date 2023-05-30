pub trait Updateable {
    fn update(&mut self, delta_time: f32);
}

pub struct EmptyUpdatable;

impl Updateable for EmptyUpdatable {
    fn update(&mut self, _delta_time: f32) {}
}
