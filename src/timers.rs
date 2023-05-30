use std::marker::PhantomData;

use crate::{alive::IsAlive, particle::AliveUpdatable, updateable::Updateable};

#[derive(Debug, Default)]
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

pub struct OneShotTimer<F, T>
where
    F: FnOnce(&mut T) -> (),
{
    ttl: f32,
    callback: Option<F>,
    _data: PhantomData<T>,
}

impl<F, T> OneShotTimer<F, T>
where
    F: FnOnce(&mut T) -> (),
{
    pub fn new(ttl: f32, callback: F) -> Self {
        OneShotTimer {
            ttl,
            callback: Some(callback),
            _data: PhantomData,
        }
    }

    fn update(&mut self, delta_time: f32, obj: &mut T) {
        if self.callback.is_some() {
            let mut ttl = self.ttl;
            if ttl > 0.0 {
                ttl -= delta_time;
                if ttl <= 0.0 {
                    let cb = self.callback.take().unwrap();
                    cb(obj);
                }
            }
        }
    }
}

impl<F, T> IsAlive for OneShotTimer<F, T>
where
    F: FnOnce(&mut T) -> (),
{
    fn is_alive(&self) -> bool {
        self.ttl > 0.
    }
}
