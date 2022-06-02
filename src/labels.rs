//! The label for each system used in the EZInput library.

use bevy::prelude::SystemLabel;

/// The label for each system used in the EZInput library.
#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub enum EZInputLabels {
    /// Label for the sytem responsible for handling the gamepad input.
    GamepadSystem,

    /// Label for the system responsible for handling the keyboard input.
    KeyboardSystem,

    /// Label for the system responsible for handling the mouse input.
    MouseSystem,

    /// Label for handling input duration and press states.
    TickSystem,
}
