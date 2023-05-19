pub trait State {
    fn update(self: Box<Self>) -> Box<dyn State>;
    fn draw(&self) {}
    fn should_continue(&self) -> bool {
        true
    }
}

pub struct ExitState;

impl State for ExitState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        // Stop updates
        self
    }

    fn should_continue(&self) -> bool {
        false
    }
}
