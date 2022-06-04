//! This module contains [`BindingTypeView`] and [`ActionBinding`], in which they are used to
//! implement the enumerated binding types and register the binding itself.
use crate::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// A trait to be implemented for enumerated action keys.
pub trait BindingTypeView:
    PartialEq + Eq + Hash + Clone + Copy + Debug + Send + Sync + 'static
{
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, Deserialize, Serialize, Default)]
pub struct InputReceivers(pub Vec<InputReceiver>);

impl From<Vec<InputReceiver>> for InputReceivers {
    fn from(input_receivers: Vec<InputReceiver>) -> Self {
        Self(input_receivers)
    }
}

/// The binding itself, and its associated receivers.
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct ActionBinding<InputKey>
where
    InputKey: BindingTypeView,
{
    pub key: InputKey,
    pub input_receivers: HashSet<InputReceivers>,
    pub default_axis_value: HashMap<InputReceiver, f32>,
}

impl<InputKey> From<InputKey> for ActionBinding<InputKey>
where
    InputKey: BindingTypeView,
{
    /// Creates a new empty actiom binding from a InputKey.
    fn from(key: InputKey) -> Self {
        Self {
            key,
            input_receivers: HashSet::default(),
            default_axis_value: HashMap::default(),
        }
    }
}

impl<InputKey> ActionBinding<InputKey>
where
    InputKey: BindingTypeView,
{
    /// Create a new action binding from a key and a list of input receivers.
    pub fn new(key: InputKey, receivers: HashSet<InputReceivers>) -> Self {
        Self {
            key,
            default_axis_value: HashMap::default(),
            input_receivers: receivers,
        }
    }

    /// Create a new action binding from a key and a non-converted list of input receivers.
    pub fn new_from_vec(key: InputKey, receiver: Vec<Vec<InputReceiver>>) -> Self {
        Self {
            key,
            default_axis_value: HashMap::default(),
            input_receivers: receiver
                .iter()
                .map(|vec| InputReceivers::from(vec.clone()))
                .collect(),
        }
    }

    /// Set the kind/key of this action.
    pub fn kind(&mut self, key: InputKey) -> &mut Self {
        self.key = key;
        self
    }

    /// Add a new input receiver to this action.
    pub fn receiver(&mut self, receiver: InputReceiver) -> &mut Self {
        self.input_receivers
            .insert(InputReceivers::from(vec![receiver]));
        self
    }

    /// Add a collection of input receivers to this action.
    pub fn receivers(&mut self, receivers: Vec<InputReceiver>) -> &mut Self {
        self.input_receivers.insert(InputReceivers::from(receivers));
        self
    }

    pub fn default_axis_value(&mut self, receiver: InputReceiver, value: f32) -> &mut Self {
        self.default_axis_value.insert(receiver, value);
        self
    }

    /// Apply the default axis value for each registered receiver for a specific view.
    pub fn apply_default_axis_to_all_receivers(&mut self, view: &mut InputView<InputKey>) -> &Self {
        for (input, value) in self.default_axis_value.iter() {
            view.descriptor_or_insert(*input).default_axis_value = *value;
        }
        self.default_axis_value.clear();
        self
    }
}
