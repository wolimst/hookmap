//! A core library for [`hookmap`].
//!
//! [`hookmap`]: https://crates.io/crates/hookmap
//!
//! This library provides abstracted input emulation and global hooks for keyboard and mouse.
//!
//! # Required traits.
//!
//! In order to support another OS, these traits need to be implemented.
//!
//! * [`EmulateButtonInput`] for [`Key`]
//! * [`EmulateButtonInput`] for [`Mouse`]
//! * [`EmulateMouseCursor`] for [`Mouse`]
//! * [`EmulateMouseWheel`] for [`Mouse`]
//! * [`InstallKeyboardHook`] for [`InputHandler`]
//! * [`InstallMouseHook`] for [`InputHandler`]
//! * [`HandleInput`] for [`InputHandler`]
//!
//! [`InstallKeyboardHook`]: crate::common::keyboard::InstallKeyboardHook
//! [`InstallMouseHook`]: crate::common::mouse::InstallMouseHook
//! [`HandleInput`]: crate::common::handler::HandleInput

pub mod common;
mod macros;

#[cfg(target_os = "windows")]
mod windows;

pub use common::{
    button::{ButtonAction, ButtonInput, ButtonState},
    event::{ButtonEvent, EventBlock},
    handler::{HandlerFunction, InputHandler, INPUT_HANDLER},
    keyboard::{Key, KeyboardEvent},
    mouse::{EmulateMouseCursor, EmulateMouseWheel, Mouse, MouseEvent},
};
