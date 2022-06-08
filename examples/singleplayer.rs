use bevy::prelude::{App, Bundle, Commands, Component, DefaultPlugins, Query, With};
use ezinput::prelude::*;

input! {
    EnumeratedBinding {
        Movement<EnumeratedMovementBinding> {
            Jump = [KeyCode::Space, GamepadButtonType::South],
            Vertical = [KeyCode::W, KeyCode::S => -1., GamepadAxisType::LeftStickY],
            Horizontal = [KeyCode::A => -1. /* default axis value */, KeyCode::D, GamepadAxisType::LeftStickX],
            Hello = [MouseAxisType::Wheel],
            Hi = [(MouseAxisType::X, MouseAxisDelta(MouseAxisType::X))],
            Combination = [(KeyCode::E, MouseButton::Left)]
        }
    }
}

type EnumeratedInputView = InputView<EnumeratedBinding>;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    #[bundle]
    pub input: InputHandlingBundle<EnumeratedBinding>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            input: InputHandlingBundle::with_deadzone(EnumeratedBinding::view(), (0.25, 0.25)),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EZInputPlugin::<EnumeratedBinding>::default())
        .add_startup_system(spawn_player)
        .add_system(check_input)
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle::default());
}

fn check_input(query: Query<&EnumeratedInputView, With<Player>>) {
    use EnumeratedBinding::*;
    use EnumeratedMovementBinding::*;

    let view = query.single();
    
    let jump = view.key(&Movement(Jump));
    if jump.pressed() {
        println!("{:?} => Jumping - {}", view.last_input_source, jump);
    }

    if let Some(axis) = view.axis(&Movement(Horizontal)).first() {
        if axis.pressed() {
            let action = if axis.value < 0. { "Left" } else { "Right" };
            println!("{:?} => {action}: {:?}", view.last_input_source, axis.value);
        }
    }
    if let Some(axis) = view.axis(&Movement(Vertical)).first() {
        if axis.pressed() {
            let action = if axis.value < 0. { "Down" } else { "Up" };
            println!("{:?} => {action}: {:?}", view.last_input_source, axis.value);
        }
    }
    if let Some(axis) = view.axis(&Movement(Hello)).first() {
        if axis.pressed() {
            println!("Mouse => Wheel: {:?}", axis.value);
        }
    }

    // In this next examples we gonna use [`itertools::Itertools`] because it is a great crate
    // and it matches perfectly our use-case.
    use itertools::Itertools;

    if let Some(keys) = view.axis(&Movement(Combination)).iter().collect_tuple() {
        let (e, left) = keys;
        if keys.pressed() {
            println!("Keyboard/Mouse => Mouse Left Button: {}, E: {}", left.press, e.press);
        }
    }

    if let Some((x, delta)) = view.axis(&Movement(Hi)).iter().collect_tuple() {
        if x.pressed() {
            println!("Mouse => X: {:?} (Δ of {:?})", x.value, delta.value);
        }
    }
}
