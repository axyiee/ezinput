/// A macro that generates input-related enumerations for easier use in ezinput.
/// 
/// ## Examples
/// 
/// This code:
/// ```rust
/// input! {
///     EnumeratedBinding {
///         Movement<EnumeratedMovementBinding> {
///             Vertical = [KeyboardKey(KeyCode::W), KeyboardKey(KeyCode::S) => -1., GamepadAxis(GamepadAxisType::LeftStickY)],
///             Horizontal = [KeyboardKey(KeyCode::A) => -1. /* default axis value */, KeyboardKey(KeyCode::D), GamepadAxis(GamepadAxisType::LeftStickX)],
///         },
///     }
/// }
/// ```
/// produces the code below:
/// ```rust
/// #[derive(BindingTypeView, Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// pub enum EnumeratedBinding {
///     Movement(EnumeratedMovementBinding),
/// }

/// #[derive(BindingTypeView, Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// pub enum EnumeratedMovementBinding {
///     Vertical,
///     Horizontal,
/// }

/// impl EnumeratedBinding {
///     pub fn view() -> InputView<Self> {
///         let mut view = InputView::new();
///         EnumeratdMovementBinding::apply(&mut view);
///         view
///     }
/// }
/// impl EnumeratedMovementBinding {
///     pub fn apply(view: &mut InputView<EnumeratedBinding>) {
///         let mut binding = ActionBinding::from(EnumeratedBinding::Movement(EnumeratedMovementBinding::Vertical));
///         binding.receiver(KeyboardKey(KeyCode::W));
///         binding.receiver(KeyboardKey(KeyCode::S));
///         binding.default_axis_value(KeyboardKey(KeyCode::S), -1);
///         view.add_binding(binding);
///         // ...
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! input {
    {
        $name:ident {
            $($category:ident<$category_enum:ident> {
                $($key:ident = [$($kind:expr $(=> $default:expr)?),* $(,)?]),* $(,)?
            }),* $(,)?
        }
    } => {

        #[derive(ezinput::BindingTypeView, Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($category($category_enum)),*
        }
        $(
            #[derive(ezinput::BindingTypeView, Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum $category_enum {
                $($key),*
            }

            impl $category_enum {
                pub fn apply(input: &mut ezinput::prelude::InputView<$name>) {
                    $(
                        let mut binding = ezinput::prelude::ActionBinding::from($name::$category($category_enum::$key));
                        $(
                            binding.receiver($kind);
                            $(
                                binding.default_axis_value($kind, $default);
                            )?
                        )*
                        input.add_binding(&binding);
                    )*
                }
            }
        )*

        impl $name {
            pub fn view() -> ezinput::prelude::InputView<$name> {
                let mut view = ezinput::prelude::InputView::new();
                $(
                    $category_enum::apply(&mut view);
                )*
                view
            }
        }
    };
}

pub use input;
