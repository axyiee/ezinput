use std::{fmt::Debug, hash::Hash};
use bevy::prelude::Component;
use ezinput_macros::*;

/// Agnostic type for representing a input source (e.g. keyboard, mouse, gamepad).
pub trait InputSource: Debug + Send + Sync + 'static {
    /// Returns whether this input source is referent to a gamepad.
    fn is_gamepad(&self) -> bool
    where
        Self: Sized + PartialEq<GamepadInputSource>,
    {
        return self == &GamepadInputSource;
    }

    /// Returns whether this input source is referent to a keyboard.
    fn is_keyboard(&self) -> bool
    where
        Self: Sized + PartialEq<KeyboardInputSource>,
    {
        return self == &KeyboardInputSource;
    }

    /// Returns whether this input source is referent to a mouse.
    fn is_mouse(&self) -> bool
    where
        Self: Sized + PartialEq<MouseInputSource>,
    {
        return self == &MouseInputSource;
    }
}

/// The input source used for gamepads.
#[derive(InputSource, PartialEq, Eq, Hash, Debug, Default, Clone, Component)]
pub struct GamepadInputSource;

/// The input source used for keyboard.
#[derive(InputSource, PartialEq, Eq, Hash, Debug, Default, Clone, Component)]
pub struct KeyboardInputSource;

/// The input source used for mouse.
#[derive(InputSource, PartialEq, Eq, Hash, Debug, Default, Clone, Component)]
pub struct MouseInputSource;
