use bevy::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Debug, Component, Clone)]
pub struct EZInputGamepadService(pub Gamepad);

impl Default for EZInputGamepadService {
    fn default() -> Self {
        Self(Gamepad(0))
    }
}

impl EZInputGamepadService {
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
        view.set_key_receiver_state(BindingInputReceiver::GamepadButton(button), state);
        view.set_axis_value(BindingInputReceiver::GamepadButton(button), duration, state);
    }

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
        view.set_key_receiver_state(BindingInputReceiver::GamepadAxis(axis), state);
        view.set_axis_value(BindingInputReceiver::GamepadAxis(axis), duration, state);
    }
}

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
                        PressState::JustPressed
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
                        PressState::JustPressed
                    };
                    svc.set_gamepad_axis_state::<Keys>(view.as_mut(), kind, state, value);
                    break;
                }
            }
            _ => {}
        }
    }
}
