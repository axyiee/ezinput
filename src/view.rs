//! A view is a object where all input states are stored. It also has useful methods such checking
//! if a key or axis for a [`BindingTypeView`] is pressed or released by proving the [`PressState`].
use std::collections::HashMap;

use bevy::{prelude::Component, utils::hashbrown::HashSet};

use crate::prelude::*;

/// Agnostic type for representing a input source (e.g. keyboard, mouse, gamepad).
#[derive(PartialEq, Eq, Hash, Debug, Clone, Component, Copy)]
pub enum InputSource {
    Gamepad,
    Keyboard,
    Mouse,
}

#[allow(dead_code)]
impl InputSource {
    /// Returns whether this input source is referent to a gamepad.
    pub fn is_gamepad(&self) -> bool {
        *self == InputSource::Gamepad
    }

    /// Returns whether this input source is referent to a keyboard.
    pub fn is_keyboard(&self) -> bool {
        *self == InputSource::Keyboard
    }

    /// Returns whether this input source is referent to a mouse.
    pub fn is_mouse(&self) -> bool {
        *self == InputSource::Mouse
    }
}

pub type BindingInputReceiver = InputReceiver;

/// The current axis state. In other words, the strength (how much the axis is moved) and press state.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct AxisState {
    pub value: f32,
    pub press: PressState,
}

impl AxisState {
    pub const ZERO: Self = Self {
        value: 0.0,
        press: PressState::Released,
    };

    pub fn new(value: f32, press: PressState) -> Self {
        Self { value, press }
    }
}

/// A view is a object where all input states are stored. It also has useful methods such checking
/// if a key or axis for a [`BindingTypeView`] is pressed or released by proving the [`PressState`].
#[derive(PartialEq, Clone, Debug, Component, Default)]
pub struct InputView<Keys>
where
    Keys: BindingTypeView,
{
    pub last_input_source: Option<InputSource>,
    pub bindings: HashMap<Keys, ActionBinding<Keys>>,
    pub receiver_states: HashMap<InputReceiver, AxisState>,
    pub receiver_default_axis_values: HashMap<InputReceiver, f32>,
}

impl<Keys> InputView<Keys>
where
    Keys: BindingTypeView,
{
    /// Create an empty input view.
    pub fn new() -> Self {
        Self {
            last_input_source: None,
            bindings: HashMap::new(),
            receiver_states: HashMap::new(),
            receiver_default_axis_values: HashMap::new(),
        }
    }

    /// Insert a new binding into the storage.
    pub fn add_binding(&mut self, binding: &ActionBinding<Keys>) -> &mut Self {
        binding.apply_default_axis_to_all_receivers(self);
        self.bindings.insert(binding.key, binding.clone());
        self
    }

    /// Add a default axis value to a receiver.
    pub fn add_receiver_default_axis_values(
        &mut self,
        receiver: InputReceiver,
        value: f32,
    ) -> &mut Self {
        self.receiver_default_axis_values.insert(receiver, value);
        self
    }

    /// Returns the default axis value for a receiver, or `1` if not found.
    pub fn get_receiver_default_axis_value(&self, receiver: InputReceiver) -> f32 {
        *self
            .receiver_default_axis_values
            .get(&receiver)
            .unwrap_or(&1.)
    }

    /// Set the button state for a specific key receiver.
    pub fn get_receiver_state(&mut self, key: InputReceiver) -> &AxisState {
        self.receiver_states.get(&key).unwrap_or(&AxisState::ZERO)
    }

    /// Set the axis state for a specific input receiver.
    pub fn set_axis_value(
        &mut self,
        receiver: InputReceiver,
        value: f32,
        element_state: PressState,
    ) {
        self.receiver_states
            .insert(receiver, AxisState::new(value, element_state));
    }

    /// Set the axis state for a specific input receiver.
    pub fn set_key_receiver_state(&mut self, receiver: InputReceiver, element_state: PressState) {
        self.set_axis_value(
            receiver,
            self.get_receiver_default_axis_value(receiver),
            element_state,
        );
    }

    /// Return the current press state for a specific binding matching with the given BindingTypeView.
    pub fn key(&self, kind: &Keys) -> PressState {
        self.axis(kind).first().unwrap_or(&AxisState::ZERO).press
    }

    /// Return the current axis state for a specific binding matching with the given BindingTypeView.
    pub fn axis(&self, kind: &Keys) -> Vec<AxisState> {
        let binding = self.bindings.get(kind);
        if let Some(binding) = binding {
            'initial: for r in binding.input_receivers.iter() {
                let states: Vec<AxisState> =
                    r.0.iter()
                        .map(|x| *self.receiver_states.get(x).unwrap_or(&AxisState::ZERO))
                        .collect();
                if states.is_empty() {
                    continue 'initial;
                }
                for state in states.iter() {
                    if state.press.released() {
                        continue 'initial;
                    }
                }
                return states;
            }
        }
        Vec::new()
    }

    /// A utility function for removing all receivers with a specific source.
    pub fn clear_from_specific_source(&mut self, source: InputSource) {
        for binding in self.bindings.values_mut() {
            let mut rcvs_: HashSet<InputReceivers> = HashSet::new();
            for rcvs in binding.input_receivers.iter() {
                let rcvs: Vec<InputReceiver> = rcvs
                    .0
                    .iter()
                    .filter(|x| x.source() != source)
                    .copied()
                    .collect();
                if !rcvs.is_empty() {
                    rcvs_.insert(InputReceivers(rcvs));
                }
            }
            binding.input_receivers = rcvs_;
            binding
                .default_axis_value
                .retain(|k, _| k.source() != source);
        }
        self.receiver_default_axis_values
            .retain(|k, _| k.source() != source);
    }
}
