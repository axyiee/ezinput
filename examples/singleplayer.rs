use bevy::prelude::{App, Bundle, Commands, Component, DefaultPlugins, Query, With};
use ezinput::prelude::{InputReceiver::*, *};

input! {
    EnumeratedBinding {
        Movement<EnumeratedMovementBinding> {
            Jump = [KeyboardKey(KeyCode::Space), GamepadButton(GamepadButtonType::South)],
            Vertical = [KeyboardKey(KeyCode::W), KeyboardKey(KeyCode::S) => -1., GamepadAxis(GamepadAxisType::LeftStickY)],
            Horizontal = [KeyboardKey(KeyCode::A) => -1. /* default axis value */, KeyboardKey(KeyCode::D), GamepadAxis(GamepadAxisType::LeftStickX)],
            Hello = [MouseAxis(MouseAxisType::Wheel)]
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
        if axis.press != PressState::Released {
            let action = if axis.value < 0. { "Left" } else { "Right" };
            println!("{:?} => {action}: {:?}", view.last_input_source, axis.value);
        }
    }
    if let Some(axis) = view.axis(&Movement(Vertical)).first() {
        if axis.press != PressState::Released {
            let action = if axis.value < 0. { "Down" } else { "Up" };
            println!("{:?} => {action}: {:?}", view.last_input_source, axis.value);
        }
    }
    if let Some(axis) = view.axis(&Movement(Hello)).first() {
        if axis.press != PressState::Released {
            println!("Mouse => Wheel: {:?}", axis.value);
        }
    }
}
