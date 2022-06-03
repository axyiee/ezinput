//! Utility for automatically inserting a type view into a Bevy entity.

use bevy::prelude::Bundle;

use crate::prelude::*;

// #[derive(Bundle)]
// pub struct InputHandlingBundle<Keys> {
//     pub input: InputView<Keys>,
//     keyboard_input: EZInputKeyboardService,
//     mouse_input: EZInputMouseService,
//     gamepad_input: EZInputGamepadService, // You may remove fields for input you don't want to support.
// }
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
}
