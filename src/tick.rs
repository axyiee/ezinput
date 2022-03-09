use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::Instant;

/// Tick every input system to update the press state for the current time, letting the input view know the press
/// state for the action.
pub(crate) fn tick_system<Keys>(mut query: Query<&mut InputView<Keys>>)
where
    Keys: BindingTypeView,
{
    fn update_time</*T,*/ F>(state: &PressState, exec: F)
    where
        //T: BindingTypeView,
        F: FnOnce() -> (),
    {
        match state {
            PressState::Pressed {
                started_pressing_instant,
            } => {
                if started_pressing_instant.is_none() {
                    exec();
                }
            }
            _ => {}
        }
    }
    for mut view in query.iter_mut() {
        for (rcv, state) in view.key_receiver_states.clone().iter() {
            update_time(state, || {
                view.set_key_receiver_state(
                    rcv.clone(),
                    PressState::Pressed {
                        started_pressing_instant: Some(Instant::now()),
                    },
                )
            });
        }
        for (rcv, state) in view.axis_receiver_states.clone().iter() {
            update_time(&state.1, || {
                view.set_axis_value(
                    rcv.clone(),
                    state.0,
                    PressState::Pressed {
                        started_pressing_instant: Some(Instant::now()),
                    },
                )
            });
        }
    }
}
