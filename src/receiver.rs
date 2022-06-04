//! All types of input accepted by default on ezinput. This doesn't mean that you can´t implement your own input sources by your own.

use std::fmt::Debug;

use bevy::prelude::{GamepadAxisType, GamepadButtonType, KeyCode, MouseButton};
use serde::{Deserialize, Serialize};

use crate::{prelude::MouseAxisType, view::InputSource};

/// A agnostic type, representing a type of input that can be accepted on ezinput systems.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum InputReceiver {
    KeyboardKey(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButtonType),
    MouseAxis(MouseAxisType),
    GamepadAxis(GamepadAxisType),
    MouseAxisDelta(MouseAxisType),
}

impl InputReceiver {
    pub fn source(&self) -> InputSource {
        match *self {
            InputReceiver::KeyboardKey(_) => InputSource::Keyboard,
            InputReceiver::GamepadButton(_) | InputReceiver::GamepadAxis(_) => InputSource::Gamepad,
            InputReceiver::MouseButton(_)
            | InputReceiver::MouseAxis(_)
            | InputReceiver::MouseAxisDelta(_) => InputSource::Mouse,
        }
    }
}

impl Into<InputReceiver> for KeyCode {
    fn into(self) -> InputReceiver {
        InputReceiver::KeyboardKey(self)
    }
}

impl Into<InputReceiver> for MouseButton {
    fn into(self) -> InputReceiver {
        InputReceiver::MouseButton(self)
    }
}

impl Into<InputReceiver> for GamepadButtonType {
    fn into(self) -> InputReceiver {
        InputReceiver::GamepadButton(self)
    }
}

impl Into<InputReceiver> for GamepadAxisType {
    fn into(self) -> InputReceiver {
        InputReceiver::GamepadAxis(self)
    }
}

impl Into<InputReceiver> for MouseAxisType {
    fn into(self) -> InputReceiver {
        InputReceiver::MouseAxis(self)
    }
}
