mod hook;
mod modifier_keys;
mod storage;

pub use modifier_keys::ModifierKeys;

use crate::{
    macros::{ButtonArg, ButtonArgTag, ButtonArgs},
    runtime::Runtime,
};
use hook::{HookProcess, HotkeyHook, MouseHook, RemapHook};
use hookmap_core::{Button, ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};
use std::{
    cell::RefCell,
    sync::{atomic::Ordering, Arc},
};
use storage::HotkeyStorage;

use self::hook::HotkeyCondition;

/// Methods for registering hotkeys.
pub trait RegisterHotkey {
    /// Makes `target` behave like a `behavior`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.remap(Button::A, Button::B);
    /// ```
    ///
    fn remap(&self, target: ButtonArgs, behavior: Button);

    /// Run `process` when `target` is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_press(Button::A, Arc::new(|e| println!("Pressed: {:?}")));
    /// ```
    ///
    fn on_press(&self, target: ButtonArgs, process: HookProcess<ButtonEvent>);

    /// Run `process` when `target` is released.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.on_release(Button::A, Arc::new(|_| println!("Released: {:?}")));
    /// ```
    ///
    fn on_release(&self, target: ButtonArgs, process: HookProcess<ButtonEvent>);

    /// Run `process` when a mouse wheel is rotated.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{hotkey::Hotkey, RegisterHotkey}
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_wheel(Arc::new(|delta| println!("Delta: {:?}", delta)));
    /// ```
    ///
    fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>);

    /// Run `process` when a mouse cursor is moved.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{hotkey::Hotkey, RegisterHotkey};
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.mouse_cursor(Arc::new(|(x, y)| println!("Cursor: ({}, {})", x, y)));
    /// ```
    ///
    fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>);

    /// Disables the button and blocks events.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey},
    ///     button::Button,
    /// };
    /// use std::sync::Arc;
    ///
    /// let hotkey = Hotkey::new();
    /// hotkey.disable(Button::A);
    /// ```
    ///
    fn disable(&self, target: ButtonArgs);

    /// Adds modifier keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey, ModifierKeys},
    ///     button::{Button, ButtonSet},
    /// };
    ///
    /// let hotkey = Hotkey::new();
    /// let modifier_keys = ModifierKeys::new(vec![ButtonSet::Any(vec![Button::A, Button::B])], vec![]);
    /// let a_or_b = hotkey.add_modifier_keys(&modifier_keys);
    /// a_or_b.remap(Button::C, Button::D);
    /// ```
    fn add_modifier_keys(&self, modifier_keys: ButtonArgs) -> ModifierHotkey;

    /// Changes the operation for native events to block or dispatch.
    ///
    /// # Examples
    ///
    /// ```
    /// use hookmap::{
    ///     hotkey::{Hotkey, RegisterHotkey, ModifierKeys},
    ///     button::Button,
    ///     hook::NativeEventBlock,
    /// };
    ///
    /// let hotkey = Hotkey::new();
    /// let blocking_hotkey = hotkey.change_native_event_operation(NativeEventOperation::Block);
    /// blocking_hotkey.on_press(Button::A, Arc::new(|e| println!("Press: {:?}", e)));
    /// ```
    ///
    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey;
}

/// Registering Hotkeys.
///
/// # Examples
///
/// ```no_run
/// use hookmap::{hotkey::Hotkey, button::Button};
///
/// let hotkey = Hotkey::new();
/// hotkey.remap(Button::A, Button::B);
/// hotkey.install();
/// ```
///
#[derive(Default)]
pub struct Hotkey {
    storage: RefCell<HotkeyStorage>,
    modifier_keys: Arc<ModifierKeys>,
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
        let runtime = Runtime::new(self.storage.into_inner());
        runtime.start();
    }
}

impl RegisterHotkey for Hotkey {
    fn remap(&self, target: ButtonArgs, behavior: Button) {
        for arg in target.iter() {
            match arg.tag {
                ButtonArgTag::Inversion => panic!(),
                ButtonArgTag::Direct => {
                    let hook = RemapHook::new(Arc::clone(&self.modifier_keys), behavior);
                    self.storage.borrow_mut().register_remap(arg.button, hook);
                }
            }
        }
    }

    fn on_press(&self, target: ButtonArgs, process: HookProcess<ButtonEvent>) {
        for arg in target.iter() {
            let hook = HotkeyHook::new(
                HotkeyCondition::Any,
                Arc::clone(&process),
                NativeEventOperation::default(),
            );
            match arg.tag {
                ButtonArgTag::Direct => {
                    self.storage
                        .borrow_mut()
                        .register_hotkey_on_press(arg.button, hook);
                }
                ButtonArgTag::Inversion => {
                    self.storage
                        .borrow_mut()
                        .register_hotkey_on_release(arg.button, hook);
                }
            }
        }
    }

    fn on_release(&self, target: ButtonArgs, process: HookProcess<ButtonEvent>) {
        for arg in target.iter() {
            let hook = HotkeyHook::new(
                HotkeyCondition::Any,
                Arc::clone(&process),
                NativeEventOperation::default(),
            );
            match arg.tag {
                ButtonArgTag::Direct => {
                    self.storage
                        .borrow_mut()
                        .register_hotkey_on_release(arg.button, hook);
                }
                ButtonArgTag::Inversion => {
                    self.storage
                        .borrow_mut()
                        .register_hotkey_on_press(arg.button, hook);
                }
            }
        }
    }

    fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            NativeEventOperation::default(),
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&self, target: ButtonArgs) {
        let process = Arc::new(|_| {}) as Arc<dyn Fn(_) + Send + Sync>;
        self.on_press(target.clone(), Arc::clone(&process));
        self.on_press(target, process);
    }

    fn add_modifier_keys(&self, modifier_keys: ButtonArgs) -> ModifierHotkey {
        ModifierHotkey::new(
            &self.storage,
            Arc::new(ModifierKeys::from(modifier_keys)),
            NativeEventOperation::default(),
        )
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(&self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}

/// Registers Hotkeys with modifier keys.
pub struct ModifierHotkey<'a> {
    storage: &'a RefCell<HotkeyStorage>,
    modifier_keys: Arc<ModifierKeys>,
    native_event_operation: NativeEventOperation,
}

impl<'a> ModifierHotkey<'a> {
    fn new(
        storage: &'a RefCell<HotkeyStorage>,
        modifier_keys: Arc<ModifierKeys>,
        native_event_operation: NativeEventOperation,
    ) -> Self {
        ModifierHotkey {
            storage,
            modifier_keys,
            native_event_operation,
        }
    }
}

impl RegisterHotkey for ModifierHotkey<'_> {
    fn remap(&self, target: ButtonArgs, behavior: Button) {
        for arg in target.iter() {
            match arg.tag {
                ButtonArgTag::Inversion => panic!(),
                ButtonArgTag::Direct => {
                    let hook = RemapHook::new(Arc::clone(&self.modifier_keys), behavior);
                    self.storage.borrow_mut().register_remap(arg.button, hook);
                }
            }
        }
    }

    fn on_press(&self, target: ButtonArgs, process: HookProcess<ButtonEvent>) {
        for arg in target.iter() {
            let hook = HotkeyHook::new(
                HotkeyCondition::Modifier(Arc::clone(&self.modifier_keys)),
                Arc::clone(&process),
                self.native_event_operation,
            );
            match arg.tag {
                ButtonArgTag::Direct => {
                    self.storage
                        .borrow_mut()
                        .register_hotkey_on_press(arg.button, hook);
                }
                ButtonArgTag::Inversion => {
                    self.storage
                        .borrow_mut()
                        .register_hotkey_on_release(arg.button, hook);
                }
            }
        }
    }

    fn on_release(&self, target: ButtonArgs, process: HookProcess<ButtonEvent>) {
        let is_active = Arc::default();
        let mut storage = self.storage.borrow_mut();
        for arg in target.iter() {
            let inactivation_hook = HotkeyHook::new(
                HotkeyCondition::Activation(Arc::clone(&is_active)),
                Arc::clone(&process),
                self.native_event_operation,
            );
            let is_active = Arc::clone(&is_active);
            let activation_hook = HotkeyHook::new(
                HotkeyCondition::Modifier(Arc::clone(&self.modifier_keys)),
                Arc::new(move |_| is_active.store(true, Ordering::SeqCst)),
                NativeEventOperation::Dispatch,
            );

            match arg.tag {
                ButtonArgTag::Direct => {
                    storage.register_hotkey_on_press(arg.button, activation_hook);
                    storage.register_hotkey_on_release(arg.button, inactivation_hook);
                }
                ButtonArgTag::Inversion => {
                    storage.register_hotkey_on_release(arg.button, activation_hook);
                    storage.register_hotkey_on_press(arg.button, inactivation_hook);
                }
            }
        }

        for target in &self.modifier_keys.pressed {
            let inactivation_hook = HotkeyHook::new(
                HotkeyCondition::Activation(Arc::clone(&is_active)),
                Arc::clone(&process),
                self.native_event_operation,
            );
            storage.register_hotkey_on_release(*target, inactivation_hook);
        }
        for target in &self.modifier_keys.released {
            let inactivation_hook = HotkeyHook::new(
                HotkeyCondition::Activation(Arc::clone(&is_active)),
                Arc::clone(&process),
                self.native_event_operation,
            );
            storage.register_hotkey_on_press(*target, inactivation_hook);
        }
    }

    fn mouse_wheel(&self, process: HookProcess<MouseWheelEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            self.native_event_operation,
        );
        self.storage.borrow_mut().register_mouse_wheel_hotkey(hook);
    }

    fn mouse_cursor(&self, process: HookProcess<MouseCursorEvent>) {
        let hook = MouseHook::new(
            Arc::clone(&self.modifier_keys),
            process,
            self.native_event_operation,
        );
        self.storage.borrow_mut().register_mouse_cursor_hotkey(hook);
    }

    fn disable(&self, target: ButtonArgs) {
        let process = Arc::new(|_| {}) as Arc<dyn Fn(_) + Send + Sync>;
        self.on_press(target.clone(), Arc::clone(&process));
        self.on_press(target, process);
    }

    fn add_modifier_keys(&self, modifier_keys: ButtonArgs) -> ModifierHotkey {
        ModifierHotkey::new(
            self.storage,
            Arc::new(self.modifier_keys.merge(ModifierKeys::from(modifier_keys))),
            self.native_event_operation,
        )
    }

    fn change_native_event_operation(&self, operation: NativeEventOperation) -> ModifierHotkey {
        ModifierHotkey::new(self.storage, Arc::clone(&self.modifier_keys), operation)
    }
}
