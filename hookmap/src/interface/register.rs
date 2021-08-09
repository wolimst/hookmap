use crate::{
    handler::{ButtonHandler, HandlerVec},
    modifier::ModifierButtonSet,
};
use hookmap_core::{Button, ButtonAction, ButtonInput};
use std::{cell::RefCell, rc::Weak, sync::Arc};

pub struct ButtonRegister {
    handler: Weak<RefCell<ButtonHandler>>,
    modifier: Arc<ModifierButtonSet>,
    button: Button,
}

impl ButtonRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<ButtonHandler>>,
        modifier: Arc<ModifierButtonSet>,
        button: Button,
    ) -> Self {
        Self {
            handler,
            modifier,
            button,
        }
    }

    /// Registers a handler called when the specified button is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_press(|_| println!("The A key is pressed"));
    /// ```
    ///
    pub fn on_press<F>(&self, callback: F)
    where
        F: FnMut(()) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_press
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified button is pressed or released.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing whether the specified key
    /// is pressed or released as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{ButtonAction, Button, Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_press_or_release(|event| {
    ///     match event {
    ///         ButtonAction::Press => println!("The A key is pressed"),
    ///         ButtonAction::Release => println!("The A key is released"),
    ///     };
    /// });
    /// ```
    ///
    pub fn on_press_or_release<F>(&self, callback: F)
    where
        F: FnMut(ButtonAction) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_press_or_release
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }

    /// Registers a handler called when the specified button is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::A).on_release(|_| println!("The A key is released"));
    /// ```
    ///
    pub fn on_release<F>(&self, callback: F)
    where
        F: FnMut(()) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_release
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }

    /// Register a handler to be called when a specified button is released and
    /// not other buttons(except modifier buttons) are pressed while that button is pressed.
    ///
    /// The specified button must be registered as a modifier.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hook = Hook::new();
    /// let _mod_space = hook.modifier(Button::Space);
    /// hook.bind(Button::Space)
    ///     .on_release_alone(|_| Button::Space.click());
    ///
    /// ```
    ///
    pub fn on_release_alone<F>(&self, callback: F)
    where
        F: FnMut(()) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .on_release_alone
            .get(self.button)
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }
    /// When the specified button is pressed, the key passed in the argument will be pressed.
    /// The same applies when the button is released.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Button, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind(Button::H).like(Button::LeftArrow);
    /// ```
    ///
    pub fn like(&self, button: Button) {
        self.on_press(move |_| button.press());
        self.on_release(move |_| button.release());
    }
}

/// A struct for registering a handler for the mouse cursor.
#[derive(Debug)]
pub struct MouseCursorRegister {
    handler: Weak<RefCell<HandlerVec<(i32, i32)>>>,
    modifier: Arc<ModifierButtonSet>,
}

impl MouseCursorRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<HandlerVec<(i32, i32)>>>,
        modifier: Arc<ModifierButtonSet>,
    ) -> Self {
        Self { handler, modifier }
    }

    /// Registers a handler called when the mouse cursor is moved.
    ///
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_cursor().on_move(|event| {
    ///     println!("Current mouse cursor position(x, y): {:?}", event);
    /// });
    /// ```
    pub fn on_move<F>(&self, callback: F)
    where
        F: FnMut((i32, i32)) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }
}

/// A struct for registering a handler for the mouse wheel.
#[derive(Debug)]
pub struct MouseWheelRegister {
    handler: Weak<RefCell<HandlerVec<i32>>>,
    modifier: Arc<ModifierButtonSet>,
}

impl MouseWheelRegister {
    pub(crate) fn new(
        handler: Weak<RefCell<HandlerVec<i32>>>,
        modifier: Arc<ModifierButtonSet>,
    ) -> Self {
        Self { handler, modifier }
    }

    /// Registers a handler called when the mouse wheel is rotated.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes `EventInfo` containing a rotation speed of the mouse
    /// wheel as an argument.
    ///
    /// # Example
    /// ```
    /// use hookmap::{Hook, SelectHandleTarget};
    /// let hook = Hook::new();
    /// hook.bind_mouse_wheel().on_rotate(|event| {
    ///     println!("Mouse wheel rotation speed: {}", event);
    /// });
    /// ```
    ///
    pub fn on_rotate<F>(&self, callback: F)
    where
        F: FnMut(i32) + Send + 'static,
    {
        self.handler
            .upgrade()
            .unwrap()
            .borrow_mut()
            .push(Box::new(callback), Arc::clone(&self.modifier));
    }
}
