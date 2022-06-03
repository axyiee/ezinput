use bevy::prelude::{App, Bundle, Commands, Component, DefaultPlugins, Query, With};
use ezinput::prelude::{InputReceiver::*, *};

input! {
    EnumeratedBinding {
        Movement<EnumeratedMovementBinding> {
            Jump = [KeyboardKey(KeyCode::Space), GamepadButton(GamepadButtonType::South)],
            Vertical = [KeyboardKey(KeyCode::W), KeyboardKey(KeyCode::S) => -1., GamepadButton(GamepadButtonType::South)],
            Horizontal = [KeyboardKey(KeyCode::A) => -1. /* default axis value */, KeyboardKey(KeyCode::D), GamepadAxis(GamepadAxisType::LeftStickX)],
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
            input: InputHandlingBundle::new(EnumeratedBinding::view()),
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
    let view = query.single();
    use EnumeratedBinding::*;
    use EnumeratedMovementBinding::*;

    if view.key(&Movement(Jump)).just_pressed() {
        println!("{:?} => Jump", view.last_input_source);
    }

    if let Some(elapsed) = view.key(&Movement(Jump)).elapsed() {
        println!("{:?} => Jumping for {:?}", view.last_input_source, elapsed);
    }

    if let Some(axis) = view.axis(&Movement(Horizontal)).first() {
        if axis.press != PressState::Released {
            let action = if axis.value < 0. { "Left" } else { "Right" };
            println!("{:?} => {action}: {:?}", view.last_input_source, axis.value);
        }
    }
}
