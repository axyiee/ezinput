//! Keyboard assigned for player 1, and controller assigned for player 2

use bevy::{
    prelude::{App, Bundle, Commands, Component, DefaultPlugins, Query, With},
};
use ezinput::prelude::{InputReceiver::*, *};

input! {
    EnumeratedBinding {
        Movement<EnumeratedMovementBinding> {
            Vertical = [KeyboardKey(KeyCode::W), KeyboardKey(KeyCode::S) => -1., GamepadAxis(GamepadAxisType::LeftStickY)],
            Horizontal = [KeyboardKey(KeyCode::A) => -1. /* default axis value */, KeyboardKey(KeyCode::D), GamepadAxis(GamepadAxisType::LeftStickX)],
        }
    }
}

type EnumeratedInputView = InputView<EnumeratedBinding>;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Name(String);

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    name: Name,
}

impl PlayerBundle {
    pub fn new(name: &str) -> Self {
        Self {
            marker: Player,
            name: Name(String::from(name)),
        }
    }
    pub fn one(commands: &mut Commands) {
        let mut view = EnumeratedBinding::view();
        view.clear_from_specific_source(InputSource::Gamepad);
        commands
            .spawn_bundle(Self::new("Player 1"))
            .insert(view)
            .insert(KeyboardMarker);
    }
    pub fn two(commands: &mut Commands) {
        let mut view = EnumeratedBinding::view();
        view.clear_from_specific_source(InputSource::Keyboard);
        commands
            .spawn_bundle(Self::new("Player 2"))
            .insert(view)
            .insert(GamepadMarker::default());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EZInputPlugin::<EnumeratedBinding>::default())
        .add_startup_system(spawn_players)
        .add_system(check_input)
        .run();
}

fn spawn_players(mut commands: Commands) {
    PlayerBundle::one(&mut commands);
    PlayerBundle::two(&mut commands);
}

fn check_input(query: Query<(&EnumeratedInputView, &Name), With<Player>>) {
    use EnumeratedBinding::*;
    use EnumeratedMovementBinding::*;

    for (view, name) in query.iter() {
        let name = &name.0;

        if let Some(vertical) = view.axis(&Movement(Vertical)).first() {
            let action = if vertical.0 < 0. { "Down" } else { "Up" };

            if vertical.1.just_pressed() {
                println!("({name}) {:?} => {action}", view.last_input_source);
            }

            if let Some(elapsed) = vertical.1.elapsed() {
                println!(
                    "({name}) {:?} => {action} for {:?}",
                    view.last_input_source, elapsed
                );
            }
        }

        if let Some(axis) = view.axis(&Movement(Horizontal)).first() {
            if axis.1 != PressState::Released {
                let action = if axis.0 < 0. { "Left" } else { "Right" };
                println!(
                    "({name}) {:?} => {action}: {:?}",
                    view.last_input_source, axis.0
                );
            }
        }
    }
}
