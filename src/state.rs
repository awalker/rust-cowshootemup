pub trait State {
    fn update(self: Box<Self>) -> Box<dyn State>;
    fn draw(&self);
    fn should_continue(&self) -> bool {
        true
    }
}

pub struct ExitState {
    old: Box<dyn State>,
}

impl ExitState {
    pub fn new(prev_state: Box<dyn State>) -> Box<Self> {
        Box::new(Self { old: prev_state })
    }
}

impl State for ExitState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        // Stop updates
        self
    }

    fn draw(&self) {
        // Might still need to a draw an old frame
        self.old.draw();
    }

    fn should_continue(&self) -> bool {
        false
    }
}
