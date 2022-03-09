//! An input-agnostic library targeting complete support to axis and button handling for many input sources.
//!
//! Input is relative; the library itself currently implements by default keyboard, mouse and controller, but
//! you can add your own input sources
//! 
//! ## Features
//! - Full joystick and keyboard support for both axis and buttons.
//! - Support for mouse position and delta changes and buttons.
//! - Builder-like pattern for creating bindings.
//! - Support for multiple input receivers.
//! - Allow bindings to be stored in a single view, meaning that you do not need a separate view for keyboard/mouse/gamepad.
//! - Support for multiple bindings per view.
//! - Support for per-entity views, meaning that multiplayer is supported.
//! - Full support for axis and button combinations.
//!
//! ## Current limitations
//! - Mouse wheel support is not implemented yet.
//!
//! If you are interested in examples, you cand find them in the `examples` directory in the source code.

pub mod binding;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod plugin;
pub mod prelude;
pub mod view;
pub mod bundle;
pub mod labels;
pub mod press_state;
pub mod tick;
pub mod source;