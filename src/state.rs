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

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = (*self).into();
        write!(f, "{}", s)
    }
}
