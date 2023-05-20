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

/// Draws the foreground state on top of the background. Pauses updates for the
/// background state, and run the foreground state until `should_continue` returns
/// false.
pub struct ModalState {
    foreground: Box<dyn State>,
    background: Box<dyn State>,
}

struct EmptyState;

impl State for EmptyState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl Default for Box<dyn State> {
    fn default() -> Self {
        Box::new(EmptyState)
    }
}

impl ModalState {
    pub fn new(foreground: Box<dyn State>, background: Box<dyn State>) -> Box<dyn State> {
        Box::new(Self {
            foreground,
            background,
        })
    }
}

impl State for ModalState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        // keep the borrow contained.
        let fg = self.foreground;
        if fg.should_continue() {
            // FIXME: This seems like it will thrash memory,
            // but using Cell also seems messy
            Self::new(fg.update(), self.background)
        } else {
            self.background
        }
    }

    fn draw(&self) {
        self.background.draw();
        self.foreground.draw();
    }

    fn should_continue(&self) -> bool {
        true
    }
}
