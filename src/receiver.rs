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

pub trait IntoReceiverVec {
    fn into_receiver_vec(&self) -> Vec<InputReceiver>;
}

impl<T> IntoReceiverVec for T
where
    T: Into<InputReceiver> + Clone,
{
    fn into_receiver_vec(&self) -> Vec<InputReceiver> {
        vec![self.clone().into()]
    }
}

impl<A, B> IntoReceiverVec for (A, B)
where
    A: IntoReceiverVec,
    B: IntoReceiverVec,
{
    fn into_receiver_vec(&self) -> Vec<InputReceiver> {
        let mut vec = self.0.into_receiver_vec();
        vec.extend(self.1.into_receiver_vec());
        vec
    }
}

impl<A, B, C> IntoReceiverVec for (A, B, C)
where
    A: IntoReceiverVec,
    B: IntoReceiverVec,
    C: IntoReceiverVec,
{
    fn into_receiver_vec(&self) -> Vec<InputReceiver> {
        let mut vec = self.0.into_receiver_vec();
        vec.extend(self.1.into_receiver_vec());
        vec.extend(self.2.into_receiver_vec());
        vec
    }
}

impl<A, B, C, D> IntoReceiverVec for (A, B, C, D)
where
    A: IntoReceiverVec,
    B: IntoReceiverVec,
    C: IntoReceiverVec,
    D: IntoReceiverVec,
{
    fn into_receiver_vec(&self) -> Vec<InputReceiver> {
        let mut vec = self.0.into_receiver_vec();
        vec.extend(self.1.into_receiver_vec());
        vec.extend(self.2.into_receiver_vec());
        vec.extend(self.3.into_receiver_vec());
        vec
    }
}

impl<A, B, C, D, E> IntoReceiverVec for (A, B, C, D, E)
where
    A: IntoReceiverVec,
    B: IntoReceiverVec,
    C: IntoReceiverVec,
    D: IntoReceiverVec,
    E: IntoReceiverVec,
{
    fn into_receiver_vec(&self) -> Vec<InputReceiver> {
        let mut vec = self.0.into_receiver_vec();
        vec.extend(self.1.into_receiver_vec());
        vec.extend(self.2.into_receiver_vec());
        vec.extend(self.3.into_receiver_vec());
        vec.extend(self.4.into_receiver_vec());
        vec
    }
}
