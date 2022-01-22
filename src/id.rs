use bevy::prelude::Component;

static mut CURRENT_INPUT_ID: u8 = 0;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Component)]
pub struct InputId(pub u8);

impl InputId {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
    pub fn next() -> Self {
        unsafe {
            let x = Self::new(CURRENT_INPUT_ID);
            CURRENT_INPUT_ID += 1;
            x
        }
    }
}