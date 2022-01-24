use std::hash::Hash;

use crate::prelude::*;
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    math::Vec2,
    prelude::{Component, EventReader, MouseButton, Query},
    window::CursorMoved,
};
use serde::{Deserialize, Serialize};

#[derive(
    PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize, strum_macros::Display,
)]
pub enum MouseAxisType {
    X,
    Y,
    Wheel,
}

#[derive(PartialEq, Debug, Component, Clone, Default)]
pub struct EZInputMouseService {
    pub mouse_position: Option<Vec2>,
    pub mouse_delta: Option<Vec2>,
    pub does_mouse_location_changed_this_tick: bool,
    pub does_mouse_wheel_changed_this_tick: bool,
}

impl EZInputMouseService {
    /// Change the current mouse location and delta and set the last input source to Mouse.
    pub fn set_mouse_location<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        position: Vec2,
        delta: Vec2,
    ) where
        Keys: BindingTypeView,
    {
        let state = PressState::Pressed {
            started_pressing_instant: None,
        };

        view.set_axis_value(
            BindingInputReceiver::MouseAxis(MouseAxisType::X),
            position.x,
            state,
        );
        view.set_axis_value(
            BindingInputReceiver::MouseAxis(MouseAxisType::Y),
            position.y,
            state,
        );
        view.set_axis_value(
            BindingInputReceiver::MouseAxisDelta(MouseAxisType::X),
            delta.x,
            state,
        );
        view.set_axis_value(
            BindingInputReceiver::MouseAxisDelta(MouseAxisType::Y),
            delta.y,
            state,
        );

        self.mouse_delta = Some(delta);
        self.mouse_position = Some(position);
        self.does_mouse_location_changed_this_tick = true;
        view.last_input_source = Some(InputSource::Mouse);
    }

    /// Tick the mouse by stop moving the axis when released.
    pub fn tick_mouse<Keys>(&mut self, view: &mut InputView<Keys>)
    where
        Keys: BindingTypeView,
    {
        view.set_axis_value(
            BindingInputReceiver::MouseAxis(MouseAxisType::X),
            0.,
            PressState::Released,
        );
        view.set_axis_value(
            BindingInputReceiver::MouseAxis(MouseAxisType::Y),
            0.,
            PressState::Released,
        );
        view.set_axis_value(
            BindingInputReceiver::MouseAxisDelta(MouseAxisType::X),
            0.,
            PressState::Released,
        );
        view.set_axis_value(
            BindingInputReceiver::MouseAxisDelta(MouseAxisType::Y),
            0.,
            PressState::Released,
        );
        self.does_mouse_location_changed_this_tick = false;
        self.does_mouse_wheel_changed_this_tick = false;
        self.mouse_delta = None;
    }

    /// Set the mouse button state for the given button and set the last input source to Mouse.
    pub fn set_mouse_button_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        button: MouseButton,
        state: PressState,
    ) where
        Keys: BindingTypeView,
    {
        view.last_input_source = Some(InputSource::Mouse);
        view.set_key_receiver_state(BindingInputReceiver::MouseButton(button), state);
        view.set_axis_value(
            BindingInputReceiver::MouseButton(button),
            match state {
                PressState::Pressed {..} => {
                    view.get_receiver_default_axis_value(BindingInputReceiver::MouseButton(button))
                }
                PressState::Released => 0.,
            },
            state,
        );
    }
}

/// Input system responsible for handling mouse input and setting the button state for each updated button and axis.
pub(crate) fn mouse_input_system<Keys>(
    mut query: Query<(&mut InputView<Keys>, &mut EZInputMouseService)>,
    mut cursor_rd: EventReader<CursorMoved>,
    mut btn_rd: EventReader<MouseButtonInput>,
    mut mtn_rd: EventReader<MouseMotion>,
) where
    Keys: BindingTypeView,
{
    for (mut view, mut mouse_svc) in query.iter_mut() {
        let view = view.as_mut();
        let mouse_svc = mouse_svc.as_mut();
        mouse_svc.tick_mouse(view);

        for (abs_position, delta) in cursor_rd.iter().zip(mtn_rd.iter()) {
            mouse_svc.set_mouse_location(view, abs_position.position, delta.delta);
        }
        for ev in btn_rd.iter() {
            mouse_svc.set_mouse_button_state(view, ev.button, ev.state.into());
        }
    }
}
