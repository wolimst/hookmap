//! Items used in macros.

use crate::button::Button;
use std::iter::{self, FromIterator};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonArgElementTag {
    Direct,
    Inversion,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ButtonArgElement {
    pub tag: ButtonArgElementTag,
    pub button: Button,
}

impl ButtonArgElement {
    pub fn direct(button: Button) -> Self {
        ButtonArgElement {
            tag: ButtonArgElementTag::Direct,
            button,
        }
    }

    pub fn inversion(button: Button) -> Self {
        ButtonArgElement {
            tag: ButtonArgElementTag::Inversion,
            button,
        }
    }

    pub fn invert(&self) -> Self {
        match self.tag {
            ButtonArgElementTag::Direct => ButtonArgElement::inversion(self.button),
            ButtonArgElementTag::Inversion => ButtonArgElement::direct(self.button),
        }
    }
}

/// A struct used in macros to pass multiple buttons to a function.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ButtonArg(Vec<ButtonArgElement>);

impl ButtonArg {
    pub fn iter(&self) -> impl Iterator<Item = ButtonArgElement> + '_ {
        self.0.iter().copied()
    }
}

impl FromIterator<Box<dyn Iterator<Item = ButtonArgElement>>> for ButtonArg {
    fn from_iter<T: IntoIterator<Item = Box<dyn Iterator<Item = ButtonArgElement>>>>(
        iter: T,
    ) -> Self {
        ButtonArg(Vec::from_iter(iter.into_iter().flatten()))
    }
}

pub trait ExpandButtonArg: Sized {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArgElement>>;
    fn expand_inverse(self) -> Box<dyn Iterator<Item = ButtonArgElement>> {
        Box::new(self.expand().map(|e| e.invert()))
    }
}

impl ExpandButtonArg for ButtonArg {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArgElement>> {
        Box::new(self.0.into_iter())
    }
}

impl ExpandButtonArg for Button {
    fn expand(self) -> Box<dyn Iterator<Item = ButtonArgElement>> {
        Box::new(iter::once(ButtonArgElement::direct(self)))
    }
}

/// Constructs [`ButtonArgs`].
#[macro_export]
macro_rules! arg {
    (@inner $parsed:tt , $($rest:tt)*) => {
        $crate::arg!(@inner $parsed $($rest)*)
    };

    (@inner [ $($parsed:tt)* ] !$button:tt $($rest:tt)*) => {
        $crate::arg!(
            @inner
            [
                $($parsed)*
                ($crate::macros::ExpandButtonArg::expand_inverse($crate::button_name!($button).clone()))
            ]
            $($rest)*
        )
    };

    (@inner [ $($parsed:tt)* ] $button:tt $($rest:tt)*) => {
        $crate::arg!(
            @inner
            [
                $($parsed)*
                ($crate::macros::ExpandButtonArg::expand($crate::button_name!($button).clone()))
            ]
            $($rest)*
        )
    };

    (@inner [ $($parsed:tt)* ]) => {
        IntoIterator::into_iter(
            [ $($parsed),* ]
        )
        .collect::<$crate::macros::ButtonArg>()
    };

    ($($args:tt)*) => {
        $crate::arg!(@inner [] $($args)*)
    };
}

/// Expands button names.
///
/// If the argument is enclosed in square brackets, it will be expanded without any action.
///
/// # Example
/// ```no_run
/// use hookmap::{button_name, devices::Button};
/// assert_eq!(Button::Key0, button_name!(0));
/// assert_eq!(Button::A, button_name!(A));
///
/// let button_a = Button::A;
/// assert_eq!(Button::A, button_name!([button_a]));
/// ```
///
// Using `#[rustfmt_skip]` instead, the following error is generated.
// error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
#[allow(clippy::deprecated_cfg_attr)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[macro_export]
macro_rules! button_name {
    ([$button:expr]) => ($button);
    (Shift)          => ($crate::devices::SHIFT);
    (Ctrl)           => ($crate::devices::Ctrl);
    (Alt)            => ($crate::devices::Alt);
    (Meta)           => ($crate::devices::Meta);
    ($button:ident)  => ($crate::devices::Button::$button);
    (0)              => ($crate::devices::Button::Key0);
    (1)              => ($crate::devices::Button::Key1);
    (2)              => ($crate::devices::Button::Key2);
    (3)              => ($crate::devices::Button::Key3);
    (4)              => ($crate::devices::Button::Key4);
    (5)              => ($crate::devices::Button::Key5);
    (6)              => ($crate::devices::Button::Key6);
    (7)              => ($crate::devices::Button::Key7);
    (8)              => ($crate::devices::Button::Key8);
    (9)              => ($crate::devices::Button::Key9);
}

/// Registers hotkeys.
///
/// # Commands
///
/// * [remap](#remap)
/// * [on_press](#on_press)
/// * [on_release](#on_release)
/// * [disable](#disable)
/// * [mouse_cursor](#mouse_cursor)
/// * [mouse_wheel](#mouse_wheel)
/// * [modifier](#modifier)
/// * [block](#block)
/// * [dispatch](#dispatch)
/// * [call](#call)
///
/// ## remap
///
/// Remap keys.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     remap A => B;
///     remap C, D => E;
/// });
/// ```
///
/// ## on_press
///
/// Registers a function to be called when the specified button is pressed.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     on_press A => |event| {};
///     on_press B, C => |_| {};
/// });
/// ```
///
/// ## on_release
///
/// Registers a function to be called when the specified button is released.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     on_release A => |event| {};
///     on_release B, C => |_| {};
/// });
/// ```
///
/// ## disable
///
/// Disables the specified button.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     disable A;
///     disable B, C;
/// });
/// ```
///
/// ## mouse_cursor
///
/// Registers a function to be called when the mouse cursor is moved.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     mouse_cursor => |(x, y)| {};
/// })
/// ```
///
/// ## mouse_wheel
///
/// Registers a function to be called when the mouse wheel is rotated.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     mouse_wheel => |speed| {};
/// });
/// ```
///
/// ## modifier
///
/// Adds modifier keys to hotkeys defined enclosed in Curly brackets.
/// The "!" in front of the button indicates that the button is released.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     modifier LShift, !RCtrl {
///         remap A => B;
///     }
/// })
/// ```
///
/// ## block
///
/// The button/mouse event will be blocked if the hotkey defined in this statement is executed.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     block {
///         on_press A => |_| {};
///     }
/// });
/// ```
///
/// ## dispatch
///
/// The button/mouse event will not be blocked if the hotkey defined in this statement is executed.
///
/// If the hotkeys defined in the `block_event` statement are executed at the same time,
/// the button event will be blocked.
///
/// ```no_run
/// use hookmap::prelude::*;
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     dispatch {
///         on_press A => |_| {};
///     }
/// });
/// ```
///
/// ## call
///
/// Calls associated functions of [`RegisterHotkey`].
///
/// [`RegisterHotkey`]: crate::hotkey::RegisterHotkey
///
/// ```no_run
/// use hookmap::prelude::*;
/// trait RemapAsTab: RegisterHotkey {
///     fn remap_as_tab(&self, target: Button) {
///         hotkey!(self => {
///             remap [target] => Tab;
///         });
///     }
/// }
/// impl<T: RegisterHotkey> RemapAsTab for T {}
///
/// let hotkey = Hotkey::new();
/// hotkey!(hotkey => {
///     call remap_as_tab(A);
/// });
/// ```
///
#[macro_export]
macro_rules! hotkey {
    {
        $hotkey:expr => {
            $($cmd:tt)*
        }
    } => {{
        let hotkey = &$hotkey;
        $crate::hotkey!(@command hotkey $($cmd)*);
    }};

    // Terminate
    (@command $hotkey:ident) => {};

    // Ignored token: =>
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ] => $($rest:tt)*) => {
        $crate::hotkey!(@$command $hotkey ( $crate::arg!($($collected)*) ) $($rest)*)
    };

    // Ignored token: ;
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ]; $($rest:tt)*) => {
        $crate::hotkey!(@$command $hotkey ( $crate::arg!($($collected)*) ) $($rest)*)
    };

    // Ignored token: { }
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ] { $($rest1:tt)* } $($rest2:tt)*) => {
        $crate::hotkey!(@$command $hotkey ( $crate::arg!($($collected)*) ) { $($rest1)* } $($rest2)*)
    };

    // Munch tokens
    (@parse_button_args_until_ignored_tokens $hotkey:ident $command:ident [ $($collected:tt)* ] $button:tt $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey $command [ $($collected)* $button ] $($rest)*)
    };

    // Matches `remap`
    (@remap $hotkey:ident $parsed:tt $rhs:tt; $($rest:tt)*) => {
        $hotkey.remap($parsed, $crate::button_name!($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `remap`.
    (@command $hotkey:ident remap $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey remap [] $($rest)*)
    };

    // Matches `on_perss`.
    (@on_press $hotkey:ident $parsed:tt $rhs:expr; $($rest:tt)*) => {
        $hotkey.on_press($parsed, std::sync::Arc::new($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_perss`.
    (@command $hotkey:ident on_press $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey on_press [] $($rest)*)
    };

    // Matches `on_release`.
    (@on_release $hotkey:ident $parsed:tt $rhs:expr; $($rest:tt)*) => {
        $hotkey.on_release($parsed, std::sync::Arc::new($rhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `on_release`.
    (@command $hotkey:ident on_release $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey on_release [] $($rest)*)
    };

    // Matches `disable`.
    (@disable $hotkey:ident $parsed:tt $($rest:tt)*) => {
        $hotkey.disable($parsed);
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `disable`.
    (@command $hotkey:ident disable $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey disable [] $($rest)*)
    };

    // Matches `mouse_cursor`.
    (@command $hotkey:ident mouse_cursor => $lhs:expr; $($rest:tt)*) => {
        $hotkey.mouse_cursor(std::sync::Arc::new($lhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `mouse_wheel`.
    (@command $hotkey:ident mouse_wheel => $lhs:expr; $($rest:tt)*) => {
        $hotkey.mouse_wheel(std::sync::Arc::new($lhs));
        $crate::hotkey!(@command $hotkey $($rest)*)
    };

    // Matches `modifier`
    (@modifier $hotkey:ident $parsed:tt { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.add_modifier_keys($parsed);
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `modifier`
    (@command $hotkey:ident modifier $($rest:tt)*) => {
        $crate::hotkey!(@parse_button_args_until_ignored_tokens $hotkey modifier [] $($rest)*)
    };

    // Matches `block`
    (@command $hotkey:ident block { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.change_native_event_operation($crate::event::NativeEventOperation::Block);
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `dispatch`.
    (@command $hotkey:ident dispatch { $($cmd:tt)* } $($rest:tt)*) => {
        {
            #[allow(unused_variables)]
            let $hotkey = $hotkey.change_native_event_operation($crate::event::NativeEventOperation::Dispatch);
            $crate::hotkey!(@command $hotkey $($cmd)*);
        }
        $crate::hotkey!(@command $hotkey $($rest)*);
    };

    // Matches `call`.
    (@command $hotkey:ident call $name:ident($($arg:tt),*); $($rest:tt)*) => {
        $hotkey.$name(
            $($crate::button_name!($arg)),*
        );
        $crate::hotkey!(@command $hotkey $($rest)*);
    };
}

/// Sends keyboard input.
/// Unlike send!, seq! does not ignore modifier keys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// seq!(A, B);
/// ```
///
/// Use `down` and `up` to press and release keys.
///
/// ```no_run
/// use hookmap::*;
/// seq!(LCtrl down, A, LCtrl up);
/// ```
///
/// Use `with(...)` to specify the keys to hold down when sending.
///
/// ```no_run
/// use hookmap::*;
/// seq!(with(LShift, LCtrl), Tab);
/// seq!(LShift down, LCtrl down, Tab, LShift up, LCtrl up); // equals to above
/// ```
///
#[macro_export]
macro_rules! seq {
    // trailing comma case
    (with($($modifier:tt)*) $(, $($button:tt $($action:ident)?),*)? ,) => {
        $crate::seq!(with($($modifier)*) $(, $($button$($action)?),*)?)
    };

    (with($($modifier:tt),*) $(, $($rest:tt)*)?) => {
        $crate::seq!($($modifier down,)* $($($rest)*,)? $($modifier up),*)
    };

    ($($button:tt $($action:ident)?),* $(,)?) => {
        $(
            $crate::seq!(@single $crate::button_name!($button) $(, $action)?);
        )*
    };

    (@single $button:expr) => {
        $crate::devices::ButtonInput::click(&$button);
    };

    (@single $button:expr, down) => {
        $crate::devices::ButtonInput::press(&$button);
    };

    (@single $button:expr, up) => {
        $crate::devices::ButtonInput::release(&$button);
    };
}

pub const MODIFIER_LIST: [Button; 8] = [
    Button::LShift,
    Button::RShift,
    Button::LCtrl,
    Button::RCtrl,
    Button::LAlt,
    Button::RAlt,
    Button::LMeta,
    Button::RMeta,
];

/// Ignores the modifier keys and sends the input events.
///
/// # Examples
///
/// ```no_run
/// use hookmap::*;
/// send!(A, B, C);
/// ```
///
/// Use `down` and `up` to press and release keys.
///
/// ```no_run
/// use hookmap::*;
/// send!(LCtrl down, A, LCtrl up);
/// ```
///
/// Use `with(...)` to specify the keys to hold down when sending.
///
/// ```no_run
/// use hookmap::*;
/// send!(with(LShift, LCtrl), Tab);
/// send!(LShift down, LCtrl down, Tab, LShift up, LCtrl up); // equals to above
/// ```
///
#[macro_export]
macro_rules! send {
    ($($input:tt)*) => {{
        let pressed_modifiers = $crate::macros::MODIFIER_LIST
            .iter()
            .filter(|button| $crate::devices::ButtonState::is_pressed(button))
            .collect::<Vec<_>>();
        pressed_modifiers.iter().for_each(|button| $crate::devices::ButtonInput::release(button));
        $crate::seq!($($input)*);
        pressed_modifiers.iter().for_each(|button| $crate::devices::ButtonInput::press(button));
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        button::{Button, ALT, CTRL, META, SHIFT},
        hotkey::{Hotkey, RegisterHotkey},
    };

    #[test]
    fn button_args() {
        use Button::*;
        assert_eq!(arg!(A), ButtonArg(vec![ButtonArgElement::direct(A)]));
        assert_eq!(arg!(!A), ButtonArg(vec![ButtonArgElement::inversion(A)]));
        assert_eq!(
            arg!(A, !B),
            ButtonArg(vec![
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B)
            ])
        );
        assert_eq!(
            arg!(A, !B),
            ButtonArg(vec![
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B)
            ])
        );
        let button_args = ButtonArg(vec![
            ButtonArgElement::direct(A),
            ButtonArgElement::inversion(B),
        ]);
        assert_eq!(arg!([button_args]), button_args);
        assert_eq!(
            arg!([button_args], C, !D),
            ButtonArg(vec![
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B),
                ButtonArgElement::direct(C),
                ButtonArgElement::inversion(D)
            ])
        );
        assert_eq!(
            arg!(C, !D, [button_args]),
            ButtonArg(vec![
                ButtonArgElement::direct(C),
                ButtonArgElement::inversion(D),
                ButtonArgElement::direct(A),
                ButtonArgElement::inversion(B)
            ]),
        );
    }

    #[test]
    fn remap() {
        hotkey!(Hotkey::new() => {
            remap A => B;
            remap A, B => C;
            remap Shift => B;
            remap [Button::A], [SHIFT] => [Button::B];
            remap A, [Button::B], [SHIFT] => A;
        });
    }

    #[test]
    fn on_press_command() {
        hotkey!(Hotkey::new() => {
            on_press A => |_| {};
            on_press A, B => |_| {};
            on_press A, !B => |_| {};
            on_press [Button::A] => |_| {};
            on_press [Button::A], [Button::B] => |_| {};
            on_press [SHIFT] => |_| {};
            on_press A, [Button::B], [SHIFT] => |_| {};
        });
    }

    #[test]
    fn on_release_command() {
        hotkey!(Hotkey::new() => {
            on_release A => |_| {};
            on_release A, B => |_| {};
            on_release A, !B => |_| {};
            on_release [Button::A] => |_| {};
            on_release [Button::A], [Button::B] => |_| {};
            on_release [SHIFT] => |_| {};
            on_release A, [Button::B], [SHIFT] => |_| {};
        });
    }

    #[test]
    fn disable_command() {
        hotkey!(Hotkey::new() => {
            disable A;
            disable A, B;
            disable [Button::A];
            disable [Button::A], [Button::B];
            disable [SHIFT];
            disable A, [Button::B], [SHIFT];
        });
    }

    #[test]
    fn mouse_cursor_command() {
        hotkey!(Hotkey::new() => {
            mouse_cursor => |_| {};
        });
    }

    #[test]
    fn mouse_wheel_command() {
        hotkey!(Hotkey::new() => {
            mouse_wheel => |_| {};
        });
    }

    #[test]
    fn modifier_command() {
        hotkey!(Hotkey::new() => {
            modifier A {}
            modifier A, B {}
            modifier !A {}
            modifier A, !A, !B, C {}
            modifier [Button::A], ![Button::B] {}
            modifier ![SHIFT], [CTRL], ![ALT] {}
            modifier ![META] {
                modifier A {}
            }
            modifier A {
                remap A => B;
            }
        });
    }

    #[test]
    fn block_command() {
        hotkey!(Hotkey::new() => {
            block {}
            block {
                dispatch {
                    remap A => B;
                }
            }
        });
    }

    #[test]
    fn dispatch_command() {
        hotkey!(Hotkey::new() => {
            dispatch {}
            dispatch {
                block {
                    remap A => B;
                }
            }
        });
    }

    #[test]
    fn button_name_macro() {
        assert_eq!(button_name!(A), Button::A);
        assert_eq!(button_name!([Button::LShift]), Button::LShift);
    }

    #[test]
    #[ignore = "This function sends keyboard input"]
    fn seq_macro() {
        seq!();
        seq!(A, B);
        seq!(A,);
        seq!([Button::A], [Button::B]);
        seq!([CTRL], [SHIFT]);
        seq!(A up, B down, [CTRL] up);
        seq!(with(A));
        seq!(with(A),);
        seq!(with(A), C,);
        seq!(with(A, B), C);
        seq!(with([Button::A], [SHIFT]), [CTRL]);
    }

    #[test]
    #[ignore = "This function sends keyboard input"]
    fn send_macro() {
        send!();
        send!(A, B);
        send!(A,);
        send!([Button::A], [Button::B]);
        send!([CTRL], [SHIFT]);
        send!(A up, B down, [CTRL] up);
        send!(with(A));
        send!(with(A),);
        send!(with(A), C,);
        send!(with(A, B), C);
        send!(with([Button::A], [SHIFT]), [CTRL]);
    }
}
