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
    pub keyboard_input: EZInputKeyboardService,
    pub mouse_input: EZInputMouseService,
    pub gamepad_input: EZInputGamepadService,
}

impl<Keys: BindingTypeView> InputHandlingBundle<Keys> {
    pub fn new(input: InputView<Keys>) -> Self {
        Self {
            input,
            keyboard_input: EZInputKeyboardService::default(),
            mouse_input: EZInputMouseService::default(),
            gamepad_input: EZInputGamepadService::default(),
        }
    }
}
