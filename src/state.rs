#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum State {
    #[default]
    Init,
    Step,
    Playing,
    Paused,
    Exit,
}

impl State {
    pub fn is_exit(&self) -> bool {
        matches!(self, State::Exit)
    }

    pub fn is_playing(&self) -> bool {
        matches!(self, State::Playing | State::Step)
    }
}
