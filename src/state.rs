use std::cell::RefCell;

use crate::{drawable::Drawable, updateable::Updateable};

/// Game State (eg. Loading, Menu, Paused, Playing)
/// At least one `State` implementation should contain
/// a `World`.
/// Implementations should contain data that is cheap to clone.
/// Use `RC`, interior mutability, or similar for large data.
pub trait State: Updateable + Drawable {
    /// return the next state or None to stay in this state
    fn transition(&self) -> Option<Box<dyn State>> {
        None
    }
    /// If the root state returns false, the game loop will exit.
    /// `ModalState` and others may use this as a transition signal.
    fn should_continue(&self) -> bool {
        true
    }
}

/// A simple state that informs the main to exit.
pub struct ExitState;

impl State for ExitState {
    fn should_continue(&self) -> bool {
        false
    }
}

impl Updateable for ExitState {
    fn update(&mut self, _delta_time: f32) {}
}

impl Drawable for ExitState {
    fn draw(&self) {}
}

/// Draws the foreground state on top of the background. Pauses updates for the
/// background state, and run the foreground state until `should_continue` returns
/// false.
pub struct ModalState<T: State + Clone> {
    foreground: Box<dyn State>,
    background: Box<T>,
}

/// A state that does nothing
struct EmptyState;

impl State for EmptyState {}

impl Updateable for EmptyState {
    fn update(&mut self, _delta_time: f32) {}
}

impl Drawable for EmptyState {
    fn draw(&self) {}
}

impl Default for Box<dyn State> {
    fn default() -> Self {
        Box::new(EmptyState)
    }
}

impl std::fmt::Debug for Box<dyn State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Box<dyn State>")
    }
}

impl<T: State + Clone + 'static> ModalState<T> {
    pub fn new(foreground: Box<dyn State>, background: Box<T>) -> Box<Self> {
        Box::new(Self {
            foreground,
            background,
        })
    }
}

impl<T: State + Clone + 'static> State for ModalState<T> {
    fn should_continue(&self) -> bool {
        true
    }

    fn transition(&self) -> Option<Box<dyn State>> {
        let fg = &self.foreground;
        if fg.should_continue() {
            fg.transition()
        } else {
            Some(Box::new((*self.background).clone()))
        }
    }
}
impl<T: State + Clone + 'static> Drawable for ModalState<T> {
    fn draw(&self) {
        self.background.draw();
        self.foreground.draw();
    }
}

impl<T: State + Clone + 'static> Updateable for ModalState<T> {
    fn update(&mut self, delta_time: f32) {
        self.foreground.update(delta_time);
    }
}

#[derive(Default)]
/// A type that can enable another state so store a dynamic state change.
/// Allows your `State` structures to still derive `Default`, `Clone`, and `Debug`
pub struct NextState(RefCell<Option<Box<dyn State>>>);

impl std::fmt::Debug for NextState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            let t = self.0.borrow();
            if t.is_none() {
                "same"
            } else {
                "something else"
            }
        };
        f.write_fmt(format_args!("NextState: {}", s))
    }
}

impl Clone for NextState {
    fn clone(&self) -> Self {
        Self(None.into())
    }
}

impl NextState {
    pub fn new(value: Option<Box<dyn State>>) -> Self {
        Self(value.into())
    }

    pub fn some(value: Box<dyn State>) -> Self {
        Self(Some(value).into())
    }

    pub fn boxed(v: impl State + 'static) -> Self {
        Self(RefCell::new(Some(Box::new(v))))
    }

    /// Call `take` during your `State`'s `transition` method.
    pub fn take(&self) -> Option<Box<dyn State>> {
        {
            let t = self.0.borrow();
            if t.is_none() {
                return None;
            }
        }
        self.0.take()
    }
}
