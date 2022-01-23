use crate::prelude::*;
use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
};

#[derive(PartialEq, Debug, Component, Clone, Copy, Default)]
pub struct EZInputKeyboardService;

impl EZInputKeyboardService {
    pub fn set_keyboard_key_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        key: KeyCode,
        state: PressState,
    ) where
        Keys: BindingTypeView,
    {
        view.last_input_source = Some(InputSource::Keyboard);
        view.set_key_receiver_state(BindingInputReceiver::KeyboardKey(key), state);
        view.set_axis_value(
            BindingInputReceiver::KeyboardKey(key),
            match state {
                PressState::Pressed | PressState::JustPressed => view.get_receiver_default_axis_value(BindingInputReceiver::KeyboardKey(key)),
                PressState::Released => 0.,
            },
            state,
        );
    }
}

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
