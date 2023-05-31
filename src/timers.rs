use crate::{alive::IsAlive, particle::AliveUpdatable, updateable::Updateable};

#[derive(Debug, Default, Clone)]
pub struct AliveTimer {
    ttl: f32,
}

impl IsAlive for AliveTimer {
    fn is_alive(&self) -> bool {
        self.ttl > 0.
    }
}

impl Updateable for AliveTimer {
    fn update(&mut self, delta_time: f32) {
        if self.is_alive() {
            self.ttl -= delta_time;
        }
    }
}

impl AliveTimer {
    pub fn new(ttl: f32) -> Self {
        Self { ttl }
    }
}

impl AliveUpdatable for AliveTimer {}
