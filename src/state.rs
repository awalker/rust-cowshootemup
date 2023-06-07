#[derive(Debug, Default, Clone, Copy)]
pub enum State {
    #[default]
    Init,
    Step,
    StepAdvance,
    Playing,
    Paused,
    Exit,
}

impl State {
    pub fn is_exit(&self) -> bool {
        matches!(self, State::Exit)
    }

    pub fn is_playing(&self) -> bool {
        matches!(self, State::Playing | State::Step | State::StepAdvance)
    }
}
