#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum State {
    #[default]
    Init,
    Idle,
    Playing,
    Paused,
}

impl From<State> for &'static str {
    fn from(value: State) -> Self {
        match value {
            State::Init => "init",
            State::Idle => "idle",
            State::Playing => "playing",
            State::Paused => "paused",
        }
    }
}
