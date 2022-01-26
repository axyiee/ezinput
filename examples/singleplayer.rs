use bevy::prelude::*;
use ezinput::prelude::*;
use ezinput_macros::*;
use strum_macros::Display;

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum EnumeratedBindings {
    Movement(EnumeratedMovementBindings),
}

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum EnumeratedMovementBindings {
    Jump,
    Left,
    Right,
}

#[derive(Bundle)]
pub struct InputBundle {
    #[bundle]
    input_bundle: InputHandlingBundle<EnumeratedBindings>,
    keyboard_input: EZInputKeyboardService,
    mouse_input: EZInputMouseService,
    gamepad_input: EZInputGamepadService, // You may remove fields for input you don't want to support.
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    input: InputBundle,
}

impl PlayerBundle {
    pub fn from_input_view(view: InputView<EnumeratedBindings>) -> Self {
        Self {
            player: Player,
            input: InputBundle {
                input_bundle: InputHandlingBundle { view },
                keyboard_input: EZInputKeyboardService::default(),
                mouse_input: EZInputMouseService::default(),
                gamepad_input: EZInputGamepadService::default(),
            },
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EZInputPlugin::<EnumeratedBindings>::default())
        .add_startup_system(spawn_player)
        .add_system(check_input)
        .run();
}

fn spawn_player(mut commands: Commands) {
    let mut view = InputView::empty();

    use ezinput::prelude::BindingInputReceiver::*;
    use EnumeratedBindings::*;
    use EnumeratedMovementBindings::*;

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

    commands.spawn_bundle(PlayerBundle::from_input_view(view));
}

fn check_input(query: Query<&InputView<EnumeratedBindings>, With<Player>>) {
    let view = query.single();
    use EnumeratedBindings::*;
    use EnumeratedMovementBindings::*;

    if view.key(&Movement(Jump)).just_pressed() {
        println!("=> Jump");
    }

    if let Some(elapsed) = view.key(&Movement(Jump)).elapsed() {
        println!("=> Jumping for {:?}", elapsed);
    }

    if let Some(left_axis) = view.axis(&Movement(Left)).first() {
        if left_axis.1 != PressState::Released && left_axis.0 < 0. {
            println!("=> Left: {:?}", left_axis.0);
        }
    }
}
