use std::collections::HashMap;

use bevy::prelude::Component;

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, strum_macros::Display)]
pub enum InputSource {
    Gamepad,
    Keyboard,
    Mouse,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct AxisState(pub f32, pub PressState);

#[derive(PartialEq, Clone, Debug, Component, Default)]
pub struct InputView<Keys>
where
    Keys: BindingTypeView,
{
    pub last_input_source: Option<InputSource>,
    pub bindings: HashMap<Keys, ActionBinding<Keys>>,
    pub key_receiver_states: HashMap<BindingInputReceiver, PressState>,
    pub axis_receiver_states: HashMap<BindingInputReceiver, AxisState>,
}

impl<Keys> InputView<Keys>
where
    Keys: BindingTypeView,
{
    pub fn empty() -> Self {
        Self {
            last_input_source: None,
            bindings: HashMap::new(),
            key_receiver_states: HashMap::new(),
            axis_receiver_states: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, key: Keys, binding: &ActionBinding<Keys>) -> &mut Self {
        self.bindings.insert(key, binding.clone());
        self
    }

    pub fn set_key_receiver_state(&mut self, key: BindingInputReceiver, state: PressState) {
        self.key_receiver_states.insert(key, state);
    }

    pub fn set_axis_value(
        &mut self,
        receiver: BindingInputReceiver,
        value: f32,
        element_state: PressState,
    ) {
        self.axis_receiver_states
            .insert(receiver, AxisState(value, element_state));
    }

    pub fn is_key_active(&self, kind: &Keys) -> bool {
        let binding = self.bindings.get(kind);
        if let Some(binding) = binding {
            binding.input_receivers.iter().any(|r| {
                r.iter()
                    .all(|b| self.key_receiver_states.get(b).unwrap_or(&PressState::Released) != &PressState::Released)
            })
        } else {
            false
        }
    }

    pub fn get_axis_states(&self, kind: &Keys) -> Vec<&AxisState> {
        let binding = self.bindings.get(kind);
        if let Some(binding) = binding {
            let mut vec = Vec::new();
            for r in binding.input_receivers.iter() {
                for r in r.iter() {
                    vec.push(
                        self.axis_receiver_states
                            .get(r)
                            .unwrap_or(&AxisState(0.0, PressState::Released)),
                    );
                }
            }
            vec
        } else {
            Vec::new()
        }
    }
}
