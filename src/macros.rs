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
