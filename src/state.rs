use bevy::input::ElementState;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, strum_macros::Display)]
pub enum PressState {
    JustPressed,
    Pressed,
    Released,
}

impl Into<PressState> for ElementState {
    fn into(self) -> PressState {
        match self {
            ElementState::Pressed => PressState::JustPressed,
            ElementState::Released => PressState::Released,
        }
    }
}
