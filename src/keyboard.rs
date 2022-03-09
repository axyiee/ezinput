//! Full keyboard support for EZInput.
use crate::prelude::*;
use bevy::{input::keyboard::KeyboardInput, prelude::*};

/// Service responsible for allowing EZInput to handle keyboard input for a specific entity.
#[derive(PartialEq, Debug, Component, Clone, Copy, Default)]
pub struct EZInputKeyboardService;

impl EZInputKeyboardService {
    /// Change the current button and axis state for the given key for and set the last input source to Keyboard.
    pub fn set_keyboard_key_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        key: KeyCode,
        state: PressState,
    ) where
        Keys: BindingTypeView,
    {
        view.set_last_input_source(Some(InputSource::Keyboard));
        view.set_key_receiver_state(BindingInputReceiver::KeyboardKey(key), state);
        let value = match state {
            PressState::Pressed { .. } => {
                view.get_receiver_default_axis_value(BindingInputReceiver::KeyboardKey(key))
            }
            PressState::Released => 0.,
        };
        view.set_axis_value(BindingInputReceiver::KeyboardKey(key), value, state);
    }
}

/// Input system responsible for handling keyboard input and setting the button state for each updated button and axis.
pub(crate) fn keyboard_input_system<Keys: BindingTypeView>(
    mut query: Query<(&mut InputView<Keys>, &mut EZInputKeyboardService)>,
    mut rd: EventReader<KeyboardInput>,
) {
    for (mut view, mut keyboard_svc) in query.iter_mut() {
        for ev in rd.iter() {
            if let Some(key) = ev.key_code {
                keyboard_svc.set_keyboard_key_state::<Keys>(&mut view, key, ev.state.into());
            }
        }
    }
}
