use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn tick_system<Keys>(mut query: Query<&mut InputView<Keys>>)
where
    Keys: BindingTypeView,
{
    for mut view in query.iter_mut() {
        for (rcv, state) in view.key_receiver_states.clone().iter() {
            view.set_key_receiver_state(
                rcv.clone(),
                match state {
                    PressState::JustPressed => PressState::Pressed,
                    other => other.clone(),
                },
            );
        }
        for (rcv, state) in view.axis_receiver_states.clone().iter() {
            view.set_axis_value(
                rcv.clone(),
                state.0,
                match state.1 {
                    PressState::JustPressed => PressState::Pressed,
                    other => other.clone(),
                },
            );
        }
    }
}
