//! Registering Hotkeys.

#[doc(hidden)]
pub mod button_arg;
mod entry;
mod hook;
mod modifiers;
mod storage;

pub use button_arg::ButtonArg;

use crate::runtime::Runtime;
use entry::{Context, HotkeyEntry};
use hook::Process;
use modifiers::Modifiers;

use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

use std::sync::Arc;

/// Methods for registering hotkeys.
pub trait RegisterHotkey {
    /// Makes `target` behave like a `behavior`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.remap(buttons!(A), Button::B);
    /// ```
    ///
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) -> &Self;

    /// Run `process` when `target` is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_press(buttons!(A), Arc::new(|e| println!("Pressed: {:?}", e)));
    /// ```
    ///
    fn on_press(
        &self,
        target: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &Self;

    /// Run `process` when `target` is released.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_release(buttons!(A), Arc::new(|e| println!("Released: {:?}", e)));
    /// ```
    ///
    fn on_release(
        &self,
        target: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &Self;

    /// Run `process` when a mouse wheel is rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{hotkey::{Hotkey, RegisterHotkey}, event::WheelEvent};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_wheel(Arc::new(|e: WheelEvent| println!("Delta: {}", e.delta)));
    /// ```
    ///
    fn mouse_wheel(&self, process: impl Into<Process<WheelEvent>>) -> &Self;

    /// Run `process` when a mouse cursor is moved.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{hotkey::{Hotkey, RegisterHotkey}, event::CursorEvent};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_cursor(Arc::new(|e: CursorEvent| println!("movement distance: {:?}", e.delta)));
    /// ```
    ///
    fn mouse_cursor(&self, process: impl Into<Process<CursorEvent>>) -> &Self;

    /// Disables the button and blocks events.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.disable(buttons!(A));
    /// ```
    ///
    fn disable(&self, target: impl Into<ButtonArg>) -> &Self;

    /// Adds modifier keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// let a_or_b = hotkey.add_modifiers(buttons!(A, B));
    /// a_or_b.remap(buttons!(C), Button::D);
    /// ```
    fn add_modifiers(&self, modifiers: impl Into<ButtonArg>) -> BranchedHotkey;

    /// If the hotkey registered in the return value of this method is executed,
    /// the input event will be blocked.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::prelude::*;
    ///
    /// let hotkey = Hotkey::new();
    /// let blocking_hotkey = hotkey.block_input_event();
    /// blocking_hotkey.on_press(Button::A, |event| println!("An input event {:?} will be blocked.", event));
    /// ```
    ///
    fn block_input_event(&self) -> BranchedHotkey;

    // If the hotkey registered in the return value of this method is executed,
    // the input event will not be blocked. However, if any other blocking hotkey
    // is executed, the input event will be blocked.
    //
    // # Examples
    //
    // ```
    // use hookmap::prelude::*;
    //
    // let hotkey = Hotkey::new();
    // let dispatching_hotkey = hotkey.dispatch_input_event();
    // dispatch_input_event.remap(Button::A, Button::B);
    // ```
    //
    fn dispatch_input_event(&self) -> BranchedHotkey;
}

/// Register Hotkeys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::prelude::*;
///
/// let hotkey = Hotkey::new();
/// hotkey.remap(buttons!(A), Button::B);
/// hotkey.install();
/// ```
///
#[derive(Default)]
pub struct Hotkey {
    entry: HotkeyEntry,
    context: Context,
}

impl Hotkey {
    /// Creates a new insgance of `Hotkey`.
    pub fn new() -> Self {
        Hotkey::default()
    }

    /// Installs registered hotkeys.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hookmap::hotkey::Hotkey;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.install();
    /// ```
    ///
    pub fn install(self) {
        let runtime = Runtime::new(self.entry.into_inner());
        runtime.start();
    }
}

impl RegisterHotkey for Hotkey {
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) -> &Self {
        self.entry
            .remap(target.into(), behavior, self.context.clone());
        self
    }

    fn on_press(
        &self,
        target: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &Self {
        self.entry
            .on_press(target.into(), process.into(), self.context.clone());
        self
    }

    fn on_release(
        &self,
        target: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &Self {
        self.entry
            .on_release(target.into(), process.into(), self.context.clone());
        self
    }

    fn mouse_wheel(&self, process: impl Into<Process<WheelEvent>>) -> &Self {
        self.entry.mouse_wheel(process.into(), self.context.clone());
        self
    }

    fn mouse_cursor(&self, process: impl Into<Process<CursorEvent>>) -> &Self {
        self.entry
            .mouse_cursor(process.into(), self.context.clone());
        self
    }

    fn disable(&self, target: impl Into<ButtonArg>) -> &Self {
        self.entry.disable(target.into(), self.context.clone());
        self
    }

    fn add_modifiers(&self, modifiers: impl Into<ButtonArg>) -> BranchedHotkey {
        let context = Context {
            modifiers: Some(Arc::new(Modifiers::from(modifiers.into()))),
            native_event_operation: self.context.native_event_operation,
        };
        BranchedHotkey::new(&self.entry, context)
    }

    fn block_input_event(&self) -> BranchedHotkey {
        let context = Context {
            native_event_operation: NativeEventOperation::Block,
            modifiers: self.context.modifiers.clone(),
        };
        BranchedHotkey::new(&self.entry, context)
    }

    fn dispatch_input_event(&self) -> BranchedHotkey {
        let context = Context {
            native_event_operation: NativeEventOperation::Dispatch,
            modifiers: self.context.modifiers.clone(),
        };
        BranchedHotkey::new(&self.entry, context)
    }
}

/// Registers Hotkeys with some conditions.
pub struct BranchedHotkey<'a> {
    entry: &'a HotkeyEntry,
    context: Context,
}

impl<'a> BranchedHotkey<'a> {
    fn new(entry: &'a HotkeyEntry, context: Context) -> Self {
        BranchedHotkey { entry, context }
    }
}

impl RegisterHotkey for BranchedHotkey<'_> {
    fn remap(&self, target: impl Into<ButtonArg>, behavior: Button) -> &Self {
        self.entry
            .remap(target.into(), behavior, self.context.clone());
        self
    }

    fn on_press(
        &self,
        target: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &Self {
        self.entry
            .on_press(target.into(), process.into(), self.context.clone());
        self
    }

    fn on_release(
        &self,
        target: impl Into<ButtonArg>,
        process: impl Into<Process<ButtonEvent>>,
    ) -> &Self {
        self.entry
            .on_release(target.into(), process.into(), self.context.clone());
        self
    }

    fn mouse_wheel(&self, process: impl Into<Process<WheelEvent>>) -> &Self {
        self.entry.mouse_wheel(process.into(), self.context.clone());
        self
    }

    fn mouse_cursor(&self, process: impl Into<Process<CursorEvent>>) -> &Self {
        self.entry
            .mouse_cursor(process.into(), self.context.clone());
        self
    }

    fn disable(&self, target: impl Into<ButtonArg>) -> &Self {
        self.entry.disable(target.into(), self.context.clone());
        self
    }

    fn add_modifiers(&self, modifiers: impl Into<ButtonArg>) -> BranchedHotkey {
        let new = Modifiers::from(modifiers.into());
        let modifiers = if let Some(modifiers) = &self.context.modifiers {
            modifiers.merge(new)
        } else {
            new
        };
        let context = Context {
            modifiers: Some(Arc::new(modifiers)),
            native_event_operation: self.context.native_event_operation,
        };
        BranchedHotkey::new(self.entry, context)
    }

    fn block_input_event(&self) -> BranchedHotkey {
        let context = Context {
            native_event_operation: NativeEventOperation::Block,
            modifiers: self.context.modifiers.clone(),
        };
        BranchedHotkey::new(self.entry, context)
    }

    fn dispatch_input_event(&self) -> BranchedHotkey {
        let context = Context {
            native_event_operation: NativeEventOperation::Dispatch,
            modifiers: self.context.modifiers.clone(),
        };
        BranchedHotkey::new(self.entry, context)
    }
}
