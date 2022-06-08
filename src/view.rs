//! A view is a object where all input states are stored. It also has useful methods such checking
//! if a key or axis for a [`BindingTypeView`] is pressed or released by proving the [`PressState`].
use std::{collections::HashMap};

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

/// A holder for input states and its default value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ReceiverDescriptor {
    pub axis: AxisState,
    pub default_axis_value: f32,
    pub input: InputReceiver,
}

impl ReceiverDescriptor {
    pub fn new(input: InputReceiver, default_axis_value: f32) -> Self {
        Self {
            axis: AxisState::ZERO,
            default_axis_value,
            input,
        }
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
    pub descriptors: Vec<ReceiverDescriptor>,
}

impl<Keys> InputView<Keys>
where
    Keys: BindingTypeView,
{
    /// Creates an empty input view with a default of 16 capacity.
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Creates an empty input view with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            last_input_source: None,
            bindings: HashMap::new(),
            descriptors: Vec::with_capacity(capacity),
        }
    }

    /// Returns the current capcity for this input view.
    pub fn capacity(&self) -> usize {
        self.descriptors.capacity()
    }

    /// Sets a new capacity for this input view.
    pub fn set_capacity(&mut self, capacity: usize) {
        let mut vec = Vec::with_capacity(capacity);
        for descriptor in self.descriptors.iter() {
            if descriptor.default_axis_value == 0. && descriptor.axis.press.released() {
                continue;
            }
            vec.push(*descriptor);
        }
        self.descriptors = vec;
    }

    /// Add a new binding to the input view.
    pub fn add_descriptor(&mut self, descriptor: ReceiverDescriptor) {
        if self.descriptors.len() >= self.capacity() {
            self.cleanup();
        }
        self.descriptors.push(descriptor);
    }

    /// Get an existing descriptor.
    pub fn descriptor(&self, rcv: &InputReceiver) -> Option<&ReceiverDescriptor> {
        self.descriptors.iter().find(|dsc| dsc.input == *rcv)
    }

    /// Get an existing descriptor mutably.
    pub fn descriptor_mut(&mut self, rcv: &InputReceiver) -> Option<&mut ReceiverDescriptor> {
        self.descriptors.iter_mut().find(|dsc| dsc.input == *rcv)
    }

    /// Get a descriptor or insert it if it doesn't exist.
    pub fn descriptor_or_insert(&mut self, input: InputReceiver) -> &mut ReceiverDescriptor {
        if let Some(index) = self.descriptors.iter().position(|dsc| dsc.input == input) {
            &mut self.descriptors[index]
        } else {
            let descriptor = ReceiverDescriptor::new(input, 0.);
            self.add_descriptor(descriptor);
            self.descriptors.last_mut().unwrap()
        }
    }

    /// Insert a new binding into the storage.
    pub fn add_binding(&mut self, binding: &mut ActionBinding<Keys>) -> &mut Self {
        binding.apply_default_axis_to_all_receivers(self);
        self.bindings.insert(binding.key, binding.clone());
        self
    }

    /// Set the button state for a specific key receiver.
    pub fn state(&self, key: &InputReceiver) -> &AxisState {
        self.descriptor(key)
            .map(|descriptor| &descriptor.axis)
            .unwrap_or(&AxisState::ZERO)
    }

    /// Set the axis state for a specific input receiver.
    pub fn set_axis_value(&mut self, input: InputReceiver, value: f32, element_state: PressState) {
        self.descriptor_or_insert(input)
            .axis
            .set(value, element_state);
    }

    /// Set the axis state for a specific input receiver.
    pub fn set_key_receiver_state(&mut self, input: InputReceiver, state: PressState) {
        let descriptor = self.descriptor_or_insert(input);
        let value = match state {
            PressState::Pressed { .. } => {
                if descriptor.axis.press.pressed() {
                   return;
                }
                descriptor.default_axis_value
            },
            PressState::Released => 0.0,
        };
        descriptor.axis.set(value, state);
    }

    /// Return the current press state for a specific binding matching with the given BindingTypeView.
    pub fn key(&self, kind: &Keys) -> PressState {
        self.axis(kind).last().unwrap_or(&AxisState::ZERO).press
    }

    /// Return the current axis state for a specific binding matching with the given BindingTypeView.
    pub fn axis(&self, kind: &Keys) -> Vec<AxisState> {
        let binding = self.bindings.get(kind);
        if let Some(binding) = binding {
            'initial: for r in binding.input_receivers.iter() {
                if r.0.is_empty() {
                    continue 'initial;
                }
                let states = r.0.iter();
                let mut output = Vec::with_capacity(states.len());
                for rcv in states {
                    let state = self.state(rcv);
                    if state.press.released() {
                        continue 'initial;
                    }
                    output.push(*state);
                }
                return output;
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
        self.descriptors.retain(|dsc| dsc.input.source() != source );
    }

    /// Combine the axis states of all given keys into a [`Vec`].
    pub fn combine<const T: usize>(&self, array: &[&Keys; T]) -> Vec<AxisState> {
        let mut output = Vec::with_capacity(T);
        for key in array {
            output.extend(self.axis(key));
        }
        output
    }

    /// Combine the first axis states of all given keys into a [`Vec`].
    pub fn combine_first<const T: usize>(&self, array: &[&Keys; T]) -> Vec<Option<AxisState>> {
        let mut output = Vec::with_capacity(T);
        for key in array {
            output.push(self.axis(key).first().map(|x| *x));
        }
        output
    }

    /// Remove all irrelevant descriptors to be with accordance with the descriptor vector capacity.
    pub fn cleanup(&mut self) {
        self.descriptors.retain(|dsc| dsc.default_axis_value != 0. || dsc.axis.press.pressed());
    }
}
