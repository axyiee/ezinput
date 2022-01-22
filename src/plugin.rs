use std::marker::PhantomData;

use crate::prelude::*;
use bevy::prelude::*;

pub struct EZInputPlugin<Keys>
where
    Keys: BindingTypeView,
{
    phantom_keys: PhantomData<Keys>,
}

impl<Keys> Default for EZInputPlugin<Keys>
where
    Keys: BindingTypeView,
{
    fn default() -> Self {
        Self {
            phantom_keys: PhantomData,
        }
    }
}

impl<Keys> Plugin for EZInputPlugin<Keys>
where
    Keys: BindingTypeView,
{
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::First,
            tick_system::<Keys>.label(EZInputLabels::TickSystem),
        );
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            keyboard_input_system::<Keys>.label(EZInputLabels::KeyboardSystem),
        );
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            mouse_input_system::<Keys>.label(EZInputLabels::MouseSystem),
        );
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            gamepad_input_system::<Keys>.label(EZInputLabels::GamepadSystem),
        );
    }
}
