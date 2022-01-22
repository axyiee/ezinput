use crate::prelude::*;
use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

pub trait BindingTypeView:
    PartialEq + Eq + Hash + Clone + Copy + Debug + Send + Sync + 'static
{
}

#[derive(
    PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize, strum_macros::Display,
)]
pub enum BindingInputReceiver {
    KeyboardKey(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButtonType),
    MouseAxis(MouseAxisType),
    GamepadAxis(GamepadAxisType),
    MouseAxisDelta(MouseAxisType),
    GamepadAxisDelta(GamepadAxisType),
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct ActionBinding<InputKey>
where
    InputKey: BindingTypeView,
{
    pub key: InputKey,
    pub input_receivers: HashSet<Vec<BindingInputReceiver>>,
}

impl<InputKey> ActionBinding<InputKey>
where
    InputKey: BindingTypeView,
{
    pub fn from(key: InputKey) -> Self {
        Self {
            key,
            input_receivers: vec![].into_iter().collect(),
        }
    }

    pub fn new(key: InputKey, receivers: HashSet<Vec<BindingInputReceiver>>) -> Self {
        Self {
            key,
            input_receivers: receivers,
        }
    }
    pub fn new_from_vec(key: InputKey, receiver: Vec<Vec<BindingInputReceiver>>) -> Self {
        Self {
            key,
            input_receivers: receiver.into_iter().collect(),
        }
    }

    pub fn kind(&mut self, key: InputKey) -> &mut Self {
        self.key = key;
        self
    }

    pub fn receiver(&mut self, receiver: BindingInputReceiver) -> &mut Self {
        self.input_receivers.insert(vec![receiver]);
        self
    }

    pub fn receivers(&mut self, receivers: Vec<BindingInputReceiver>) -> &mut Self {
        self.input_receivers.insert(receivers);
        self
    }
}
