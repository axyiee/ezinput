use bevy::prelude::SystemLabel;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub enum EZInputLabels {
    GamepadSystem,
    KeyboardSystem,
    MouseSystem,
    TickSystem,
}