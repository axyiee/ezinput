//! The press state for a button or axis. Also useful methods for checking the elapsed time.
use std::fmt::{Debug, Display};

#[allow(unused_imports)]
use std::ops::Add;

use bevy::input::ButtonState;
use bevy::utils::{Duration, Instant};

/// The current state of a specific axis or button. By default, calls return [`PressState::Released`].
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PressState {
    /// The button or axis is pressed, along with the initial instant for the press.
    /// This need to be set as none if is the moment the button is just pressed, since it will
    /// let the input view know that the button is just pressed. The pressing instant is set
    /// in the next tick to allow users to know the pressing duration.
    Pressed {
        started_pressing_instant: Option<Instant>,
    },

    /// The button or axis is released.
    Released,
}

pub trait PressStateExt {
    /// Returns whether if the current press state is released or not.
    fn released(&self) -> bool;

    /// Returns whether if the current press state is pressed for more than a specific duration.
    fn is_pressed_for(&self, duration: Duration) -> bool;

    /// Returns whether the button or axis was just pressed or moved in this exact tick or not.
    fn just_pressed(&self) -> bool;

    /// Returns whether the button or axis is currently pressed or moving.
    fn pressed(&self) -> bool;

    /// Returns the elapsed time since the action was pressed.
    fn elapsed(&self) -> Option<Duration>;
}

impl PressStateExt for PressState {
    #[inline]
    fn released(&self) -> bool {
        *self == PressState::Released
    }

    #[inline]
    fn is_pressed_for(&self, duration: Duration) -> bool {
        if let PressState::Pressed {
            started_pressing_instant,
        } = *self
        {
            started_pressing_instant.is_some()
                && started_pressing_instant.unwrap().elapsed() >= duration
        } else {
            false
        }
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        if let PressState::Pressed {
            started_pressing_instant,
        } = *self
        {
            if let Some(instant) = started_pressing_instant {
                instant.elapsed().as_millis() <= 1
            } else {
                true
            }
        } else {
            false
        }
    }

    #[inline]
    fn pressed(&self) -> bool {
        matches!(*self, PressState::Pressed { .. })
    }

    #[inline]
    fn elapsed(&self) -> Option<Duration> {
        match self {
            PressState::Pressed {
                started_pressing_instant,
            } => started_pressing_instant
                .as_ref()
                .map(|started_pressing_instant| started_pressing_instant.elapsed())
                .or(Some(Duration::ZERO)),
            _ => None,
        }
    }
}

/// Implement partial comparision between press states.
impl PartialOrd for PressState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            PressState::Pressed {
                started_pressing_instant: a,
            } => match other {
                PressState::Pressed {
                    started_pressing_instant: b,
                } => Some(a.cmp(b)),
                PressState::Released => Some(std::cmp::Ordering::Greater),
            },
            PressState::Released => match other {
                PressState::Pressed { .. } => Some(std::cmp::Ordering::Less),
                PressState::Released => Some(std::cmp::Ordering::Equal),
            },
        }
    }
}

/// Implement comparison between press states.
impl Ord for PressState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Implementation responsible for translating Bevy element states to EZInput press states.
/// By default, the default pressing instant is the None.
impl From<ButtonState> for PressState {
    fn from(value: ButtonState) -> PressState {
        match value {
            ButtonState::Pressed => PressState::Pressed {
                started_pressing_instant: None,
            },
            ButtonState::Released => PressState::Released,
        }
    }
}

/// Implementation responsible for allowing the input source to be displayed as a string.
impl Display for PressState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PressState::Pressed { .. } => {
                if self.just_pressed() {
                    write!(f, "Pressing since Now")
                } else {
                    write!(f, "Pressing for {:?}", self.elapsed())
                }
            }

            PressState::Released => write!(f, "Released"),
        }
    }
}

// Test to compare if `PartialOrd` is implemented correctly.
#[test]
fn partial_ord_press_state_test() {
    let a = PressState::Pressed {
        started_pressing_instant: Some(Instant::now()),
    };
    let b = PressState::Pressed {
        started_pressing_instant: Some(Instant::now().add(Duration::from_secs(342534))),
    };
    let value = a.cmp(&b);
    assert_eq!(value, std::cmp::Ordering::Less);
}

/// The current axis state. In other words, the strength (how much the axis is moved) and press state.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct AxisState {
    pub value: f32,
    pub press: PressState,
}

impl AxisState {
    pub const ZERO: Self = Self {
        value: 0.0,
        press: PressState::Released,
    };

    pub fn new(value: f32, press: PressState) -> Self {
        Self { value, press }
    }

    pub fn set(&mut self, value: f32, press: PressState) {
        self.value = value;
        self.press = press;
    }
}

impl PressStateExt for AxisState {
    #[inline]
    fn released(&self) -> bool {
        self.press.released()
    }

    #[inline]
    fn is_pressed_for(&self, duration: Duration) -> bool {
        self.press.is_pressed_for(duration)
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.press.just_pressed()
    }

    #[inline]
    fn pressed(&self) -> bool {
        self.press.pressed()
    }

    #[inline]
    fn elapsed(&self) -> Option<Duration> {
        self.press.elapsed()
    }
}

pub trait AxisStateVecExt {
    fn pressed(&self) -> bool;

    fn just_pressed(&self) -> bool;

    fn released(&self) -> bool;
}

impl AxisStateVecExt for Vec<AxisState> {
    fn pressed(&self) -> bool {
        self.iter().all(|s| s.press.pressed())
    }

    fn just_pressed(&self) -> bool {
        self.iter().all(|s| s.press.just_pressed())
    }

    fn released(&self) -> bool {
        self.iter().all(|s| s.press.released())
    }
}

impl AxisStateVecExt for (&PressState, &PressState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.pressed() && self.1.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.just_pressed() && self.1.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.released() && self.1.released()
    }
}

impl AxisStateVecExt for (&PressState, &PressState, &PressState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.pressed() && self.1.pressed() && self.2.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.just_pressed() && self.1.just_pressed() && self.2.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.released() && self.1.released() && self.2.released()
    }
}

impl AxisStateVecExt for (&PressState, &PressState, &PressState, &PressState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.pressed() && self.1.pressed() && self.2.pressed() && self.3.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.just_pressed()
            && self.1.just_pressed()
            && self.2.just_pressed()
            && self.3.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.released() && self.1.released() && self.2.released() && self.3.released()
    }
}

impl AxisStateVecExt
    for (
        &PressState,
        &PressState,
        &PressState,
        &PressState,
        &PressState,
    )
{
    #[inline]
    fn pressed(&self) -> bool {
        self.0.pressed()
            && self.1.pressed()
            && self.2.pressed()
            && self.3.pressed()
            && self.4.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.just_pressed()
            && self.1.just_pressed()
            && self.2.just_pressed()
            && self.3.just_pressed()
            && self.4.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.released()
            && self.1.released()
            && self.2.released()
            && self.3.released()
            && self.4.released()
    }
}

impl AxisStateVecExt for (&AxisState, &AxisState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.press.pressed() && self.1.press.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.press.just_pressed() && self.1.press.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.press.released() && self.1.press.released()
    }
}

impl AxisStateVecExt for (&AxisState, &AxisState, &AxisState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.press.pressed() && self.1.press.pressed() && self.2.press.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.press.just_pressed() && self.1.press.just_pressed() && self.2.press.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.press.released() && self.1.press.released() && self.2.press.released()
    }
}

impl AxisStateVecExt for (&AxisState, &AxisState, &AxisState, &AxisState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.press.pressed()
            && self.1.press.pressed()
            && self.2.press.pressed()
            && self.3.press.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.press.just_pressed()
            && self.1.press.just_pressed()
            && self.2.press.just_pressed()
            && self.3.press.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.press.released()
            && self.1.press.released()
            && self.2.press.released()
            && self.3.press.released()
    }
}

impl AxisStateVecExt for (&AxisState, &AxisState, &AxisState, &AxisState, &AxisState) {
    #[inline]
    fn pressed(&self) -> bool {
        self.0.press.pressed()
            && self.1.press.pressed()
            && self.2.press.pressed()
            && self.3.press.pressed()
            && self.4.press.pressed()
    }

    #[inline]
    fn just_pressed(&self) -> bool {
        self.0.press.just_pressed()
            && self.1.press.just_pressed()
            && self.2.press.just_pressed()
            && self.3.press.just_pressed()
            && self.4.press.just_pressed()
    }

    #[inline]
    fn released(&self) -> bool {
        self.0.press.released()
            && self.1.press.released()
            && self.2.press.released()
            && self.3.press.released()
            && self.4.press.released()
    }
}
