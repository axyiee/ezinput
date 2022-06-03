<div align="center">
    <h1>ezinput</h1>
    <a href="https://git.exst.fun/ezinput">
        <img src="https://img.shields.io/github/stars/eexsty/ezinput?colorA=1e1e28&colorB=1187c9&style=for-the-badge&logo=github" alt="GitHub" />
    </a>
    <a href="https://crates.io/crates/ezinput">
        <img src="https://img.shields.io/crates/v/ezinput.svg?style=for-the-badge&colorA=1e1e28&colorB=1187c9&logo=rust" alt="crates.io">
    </a>
    <a href="https://git.exst.fun/ezinput/blob/master/.github/workflows/build.yml">
        <img src="https://img.shields.io/github/workflow/status/eexsty/ezinput/Rust%20CI%20with%20Cargo?colorA=1e1e28&colorB=1187c9&label=Rust&style=for-the-badge&logo=rust" alt="GitHub Actions" />
    </a>
    <a href="https://docs.rs/ezinput/latest/ezinput/">
        <img src="https://img.shields.io/docsrs/ezinput?logo=docs.rs&colorA=1e1e28&colorB=1187c9&style=for-the-badge" alt="GitHub Actions" />
    </a>
    <br/>
    <strong>A powerful input-agnostic library targeting complete support to axis and button handling for the Bevy game engine.</strong>
</div>


### Table of contents

1. [About](#about)
1. [Branches](#branches)
1. [Getting started](#getting-started)
1. [Examples](https://git.exst.fun/ezinput/tree/master/examples)


## About

Input is relative; the library itself currently implements by default keyboard, mouse and gamepad support, but this is subject to change.
Please feel free to contribute to the library by submitting pull requests. Touch support is stil planned, but not yet implemented.

ezinput strives to be simple as possible, while still being powerful and flexible without using any unsafe code.

All bindings are stored in a `InputView` struct, which is passed as a component to your ECS entitity. To allow an input method to be handled,
you need to add a service marker component (`MouseMarker`, `KeyboardMarker` or `GamepadMarker`) to the ECS entity. You aren't limited to one marker, since you can use multiple markers to handle multiple input methods. An advantage of this implementation is that input views aren't
limited to specific input sources, so you can reutilize the same view for multiple input methods just by adding new input receivers to bindings.

Not everything is documented yet or documented with a high level of detail, so any feedback is appreciated. You can contact me on [Discord]
or here on GitHub!

### Limitations

* Mouse wheel support is not implemented yet.
* Touch support is not implemented yet.
* Input receivers are limited only to implemented input sources.
* Input sources are a hard-coded enumeration (it might not be that bad in most cases though).

## Branches

<table>
    <tr>
        <th>Branch</th>
        <th>Bevy version</th>
    </tr>
    <tr>
        <td>master</td>
        <td><code>^0.7</code></td>
    </tr>
    <tr>
       <td>bevy_main</td>
       <td><strong>git (https://github.com/bevyengine/bevy.git)</td>
    </tr>
</table>


## Getting started

Add the following to your `Cargo.toml` (replace `^0.3` with the latest version):
```toml
[dependencies]
ezinput = "^0.3"
```

* Now, you need to import `ezinput::prelude::*`.
* Create an input view by using the `input!` macro. You can see an example [here](https://git.exst.fun/ezinput/tree/bevy_main/examples). 
```rust
use ezinput::prelude::*;

input! {
    EnumeratedBinding {
        Movement<EnumeratedMovementBinding> {
            Vertical = [KeyboardKey(KeyCode::W), KeyboardKey(KeyCode::S) => -1., GamepadAxis(GamepadAxisType::LeftStickY)],
            Horizontal = [KeyboardKey(KeyCode::A) => -1. /* default axis value */, KeyboardKey(KeyCode::D), GamepadAxis(GamepadAxisType::LeftStickX)],
        },
    }
}

// Is the same as this:
// #[derive(BindingTypeView, Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum EnumeratedBinding {
//     Movement(EnumeratedMovementBinding),
// }

// #[derive(BindingTypeView, Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum EnumeratedMovementBinding {
//     Vertical,
//     Horizontal,
// }

// impl EnumeratedBinding {
//     pub fn view() -> InputView<Self> {
//         let mut view = InputView::new();
//         EnumeratdMovementBinding::apply(&mut view);
//         view
//     }
// }

// impl EnumeratedMovementBinding {
//     pub fn apply(view: &mut InputView<EnumeratedBinding>) {
//         let mut binding = ActionBinding::from(EnumeratedBinding::Movement(EnumeratedMovementBinding::Vertical));
//         binding.receiver(KeyboardKey(KeyCode::W));
//         binding.receiver(KeyboardKey(KeyCode::S));
//         binding.default_axis_value(KeyboardKey(KeyCode::S), -1);
//         view.add_binding(binding);
//         // ...
//     }
// }
```

[Discord]: https://discord.com/users/929877747151548487