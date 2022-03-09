//! Keyboard assigned for player 1, and controller assigned for player 2

use bevy::prelude::*;
use ezinput::prelude::*;
use ezinput_macros::*;
use strum_macros::Display;

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum EnumeratedBinding {
    Movement(EnumeratedMovementBinding),
}

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum EnumeratedMovementBinding {
    Jump,
    Left,
    Right,
}
#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    input: InputHandlingBundle<EnumeratedBinding>,
}

impl PlayerBundle {
    pub fn from_input_view(view: InputView<EnumeratedBinding>) -> Self {
        Self {
            player: Player,
            input: InputHandlingBundle { view },
        }
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
    let mut view = InputView::empty();
    use ezinput::prelude::BindingInputReceiver::*;
    use EnumeratedBinding::*;
    use EnumeratedMovementBinding::*;

    view.add_binding(
        Movement(Jump),
        ActionBinding::from(Movement(Jump))
            .receiver(KeyboardKey(KeyCode::Space))
            .receiver(GamepadButton(GamepadButtonType::South)),
    );
    view.add_binding(
        Movement(Left),
        ActionBinding::from(Movement(Left))
            .receiver(KeyboardKey(KeyCode::A))
            .receiver(GamepadAxis(GamepadAxisType::LeftStickX))
            .default_axis_value(KeyboardKey(KeyCode::A), -1.),
    );
    view.add_binding(
        Movement(Right),
        ActionBinding::from(Movement(Right))
            .receiver(KeyboardKey(KeyCode::D))
            .receiver(GamepadAxis(GamepadAxisType::LeftStickX)),
    );

    commands
        .spawn()
        .insert_bundle(PlayerBundle::from_input_view(view.clone()))
        .insert(EZInputKeyboardService);

    // There is better ways for detecting a controller, but this is just an example
    // So we will get the first controller.
    let gamepad = Gamepad(0);
    commands
        .spawn()
        .insert_bundle(PlayerBundle::from_input_view(view.clone()))
        .insert(EZInputGamepadService(gamepad));
}

fn check_input(query: Query<&InputView<EnumeratedBinding>, With<Player>>) {
    use EnumeratedBinding::*;
    use EnumeratedMovementBinding::*;

    for view in query.iter() {
        let player = match view.last_input_source.unwrap_or(InputSource::Keyboard) {
            InputSource::Keyboard | InputSource::Mouse => 1,
            InputSource::Gamepad => 2,
        };

        if view.key(&Movement(Jump)).just_pressed() {
            println!("[Player {}] => Jump", player);
        }

        if let Some(elapsed) = view.key(&Movement(Jump)).elapsed() {
            println!("[Player {}] => Jumping for {:?}", player, elapsed);
        }

        if let Some(left_axis) = view.axis(&Movement(Left)).first() {
            if left_axis.1 != PressState::Released && left_axis.0 < 0. {
                println!("[Player {}] => Left: {:?}", player, left_axis.0);
            }
        }
    }
}
