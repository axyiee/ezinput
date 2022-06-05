#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod binding;
pub mod bundle;
pub mod gamepad;
pub mod keyboard;
pub mod macros;
pub mod mouse;
pub mod plugin;
pub mod state;
pub mod receiver;
pub mod view;
pub use ezinput_macros::*;

pub mod prelude {
    pub use crate::binding::*;
    pub use crate::bundle::*;
    pub use crate::gamepad::*;
    pub use crate::keyboard::*;
    pub use crate::macros::*;
    pub use crate::mouse::*;
    pub use crate::plugin::*;
    pub use crate::state::*;
    pub use crate::receiver::*;
    pub use crate::view::*;
    pub use crate::BindingTypeView;
    pub use crate::receiver::InputReceiver::*;
    pub use bevy::prelude::{GamepadAxisType, GamepadButtonType, KeyCode, MouseButton};
}
