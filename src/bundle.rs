//! Definition of a ECS component bundle providing an input view and all supported input markers.

use bevy::prelude::Bundle;

use crate::prelude::*;

#[derive(Debug, Bundle)]
pub struct InputHandlingBundle<Keys>
where
    Keys: BindingTypeView,
{
    pub input: InputView<Keys>,
    pub keyboard_input: KeyboardMarker,
    pub mouse_input: MouseMarker,
    pub gamepad_input: GamepadMarker,
}

impl<Keys: BindingTypeView> InputHandlingBundle<Keys> {
    pub fn new(input: InputView<Keys>) -> Self {
        Self {
            input,
            keyboard_input: KeyboardMarker::default(),
            mouse_input: MouseMarker::default(),
            gamepad_input: GamepadMarker::default(),
        }
    }
    pub fn with_deadzone(input: InputView<Keys>, dead_zone: (f32, f32)) -> Self {
        Self {
            gamepad_input: GamepadMarker::with_dead_zone(0, dead_zone),
            ..Self::new(input)
        }
    }
}