//! Full gamepad support for EZInput.
use bevy::prelude::*;

use crate::prelude::*;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct GamepadInputHandlingSystem;

// Marker responsible for allowing systems to listen to gamepad input.
#[derive(PartialEq, Debug, Component, Clone)]
pub struct GamepadMarker {
    pub gamepad: Gamepad,
    pub dead_zone: Vec2,
}

impl Default for GamepadMarker {
    fn default() -> Self {
        Self::with_id(0)
    }
}

impl GamepadMarker {
    pub fn with_id(id: usize) -> Self {
        Self {
            gamepad: Gamepad(id),
            dead_zone: Vec2::ZERO,
        }
    }
    pub fn with_dead_zone(id: usize, dead_zone: (f32, f32)) -> Self {
        Self {
            gamepad: Gamepad(id),
            dead_zone: Vec2::new(dead_zone.0, dead_zone.1),
        }
    }
}

impl GamepadMarker {
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
        view.set_axis_value(InputReceiver::GamepadAxis(axis), duration, state);
    }
}

/// Input system responsible for handling gamepad input and setting the button state for each updated button and axis.
pub(crate) fn gamepad_input_system<Keys>(
    mut query: Query<(&mut InputView<Keys>, &mut GamepadMarker)>,
    mut rd: EventReader<GamepadEvent>,
) where
    Keys: BindingTypeView,
{
    for ev in rd.iter() {
        match ev.1 {
            GamepadEventType::ButtonChanged(kind, duration) => {
                for (mut view, mut svc) in query.iter_mut() {
                    if ev.0 != svc.gamepad {
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
                    if ev.0 != svc.gamepad {
                        continue;
                    }
                    let state = if value.abs() <= 0.1 {
                        PressState::Released
                    } else {
                        PressState::Pressed {
                            started_pressing_instant: None,
                        }
                    };
                    if state.pressed()
                        && match kind {
                            GamepadAxisType::LeftStickX | GamepadAxisType::RightStickX => {
                                value.abs() < svc.dead_zone.x
                            }
                            GamepadAxisType::LeftStickY | GamepadAxisType::RightStickY => {
                                value.abs() < svc.dead_zone.y
                            }
                            _ => false,
                        }
                    {
                        continue;
                    };
                    svc.set_gamepad_axis_state::<Keys>(view.as_mut(), kind, state, value);
                    break;
                }
            }
            _ => {}
        }
    }
}
