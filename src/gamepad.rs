//! Full gamepad support for EZInput.
use bevy::prelude::*;

use crate::prelude::*;

/// Service responsible for storing a specific gamepad for a entity,
/// and allowing for handling gamepad input.
#[derive(PartialEq, Eq, Debug, Component, Clone)]
pub struct EZInputGamepadService(pub Gamepad);

/// Implementation that creates a gamepad service with the first gamepad by default.
impl Default for EZInputGamepadService {
    fn default() -> Self {
        Self(Gamepad(0))
    }
}

impl EZInputGamepadService {
    /// Change the current button state for the given button and set the last input source to Gamepad.
    pub fn set_gamepad_button_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        button: GamepadButtonType,
        state: PressState,
        duration: f32,
    ) where
        Keys: BindingTypeView,
    {
        view.last_input_source = Some(InputSource::Gamepad);
        view.set_key_receiver_state(InputReceiver::GamepadButton(button), state);
        view.set_axis_value(InputReceiver::GamepadButton(button), duration, state);
    }

    /// Change the current axis state for the given axis and set the last input source to Gamepad.
    pub fn set_gamepad_axis_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        axis: GamepadAxisType,
        state: PressState,
        duration: f32,
    ) where
        Keys: BindingTypeView,
    {
        view.last_input_source = Some(InputSource::Gamepad);
        view.set_key_receiver_state(InputReceiver::GamepadAxis(axis), state);
        view.set_axis_value(InputReceiver::GamepadAxis(axis), duration, state);
    }
}

/// Input system responsible for handling gamepad input and setting the button state for each updated button and axis.
pub(crate) fn gamepad_input_system<Keys>(
    mut query: Query<(&mut InputView<Keys>, &mut EZInputGamepadService)>,
    mut rd: EventReader<GamepadEvent>,
) where
    Keys: BindingTypeView,
{
    for ev in rd.iter() {
        match ev.1 {
            GamepadEventType::ButtonChanged(kind, duration) => {
                for (mut view, mut svc) in query.iter_mut() {
                    if ev.0 != svc.0 {
                        continue;
                    }
                    let state = if duration.abs() <= 0.1 {
                        PressState::Released
                    } else {
                        PressState::Pressed {
                            started_pressing_instant: None,
                        }
                    };
                    svc.set_gamepad_button_state::<Keys>(view.as_mut(), kind, state, duration);
                    break;
                }
            }
            GamepadEventType::AxisChanged(kind, value) => {
                for (mut view, mut svc) in query.iter_mut() {
                    if ev.0 != svc.0 {
                        continue;
                    }
                    let state = if value.abs() <= 0.1 {
                        PressState::Released
                    } else {
                        PressState::Pressed {
                            started_pressing_instant: None,
                        }
                    };
                    svc.set_gamepad_axis_state::<Keys>(view.as_mut(), kind, state, value);
                    break;
                }
            }
            _ => {}
        }
    }
}
