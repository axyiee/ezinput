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
    gamepad_input: EZInputGamepadService,
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
    view.add_binding(
        EnumeratedBindings::Movement(EnumeratedMovementBindings::Jump),
        ActionBinding::from(EnumeratedBindings::Movement(
            EnumeratedMovementBindings::Jump,
        ))
        .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Space))
        .receiver(BindingInputReceiver::GamepadButton(
            GamepadButtonType::South,
        )),
    );
    view.add_binding(
        EnumeratedBindings::Movement(EnumeratedMovementBindings::Left),
        ActionBinding::from(EnumeratedBindings::Movement(
            EnumeratedMovementBindings::Left,
        ))
        .receiver(BindingInputReceiver::KeyboardKey(KeyCode::A))
        .receiver(BindingInputReceiver::GamepadAxis(
            GamepadAxisType::LeftStickX,
        )),
    );
    view.add_binding(
        EnumeratedBindings::Movement(EnumeratedMovementBindings::Left),
        ActionBinding::from(EnumeratedBindings::Movement(
            EnumeratedMovementBindings::Right,
        ))
        .receiver(BindingInputReceiver::KeyboardKey(KeyCode::D))
        .receiver(BindingInputReceiver::GamepadAxis(
            GamepadAxisType::LeftStickX,
        )),
    );

    commands.spawn_bundle(PlayerBundle::from_input_view(view));
}

fn check_input(query: Query<&InputView<EnumeratedBindings>, With<Player>>) {
    let view = query.single();

    if view.is_key_active(&EnumeratedBindings::Movement(
        EnumeratedMovementBindings::Jump,
    )) {
        println!("=> Jump");
    }

    if let Some(left_axis) = view
        .get_axis_states(&EnumeratedBindings::Movement(
            EnumeratedMovementBindings::Left,
        ))
        .first()
    {
        if left_axis.1 != PressState::Released && left_axis.0 < 0. {
            println!("=> Left: {:?}", left_axis.0);
        }
    }
}
