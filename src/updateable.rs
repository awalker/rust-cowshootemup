pub trait Updateable {
    fn update(&mut self, delta_time: f32);
}

pub struct EmptyUpdatable;

impl Updateable for EmptyUpdatable {
    fn update(&mut self, _delta_time: f32) {}
}

impl<T: Updateable> Updateable for Vec<T> {
    fn update(&mut self, delta_time: f32) {
        self.iter_mut().for_each(|f| f.update(delta_time));
    }
}
