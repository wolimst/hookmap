use crate::{hotkey, hotkey::RegisterHotkey, macros::ButtonArgs, seq};
use std::sync::atomic::{AtomicBool, Ordering};

static IS_ALT_TAB_WORKING: AtomicBool = AtomicBool::new(false);

/// Utility function.
pub trait Utils: RegisterHotkey {
    /// Alt-Tab hotkey.
    ///
    /// # Arguments
    ///
    /// * `alt` - A button that act like Alt key.
    /// * `tab` - A button that act like tab key.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_alt_tab(Button::A, Button::T);
    /// ```
    // fn bind_alt_tab<B: EmulateButtonState>(&self, alt: &B, tab: &B) {
    //     alt_tab(self, alt, tab, &Button::Tab);
    // }
    fn bind_alt_tab(&self, alt: ButtonArgs, tab: ButtonArgs) {
        hotkey!(self => {
            on_release [alt] => move |_| {
                IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
                seq!(LAlt up);
            };

            modifier [alt] {
                disable [tab];
                on_press [tab] => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        seq!(LAlt down);
                    }
                    seq!(Tab);
                };
            }
        });
    }

    /// Shift-Alt-Tab hotkey.
    ///
    /// # Arguments
    ///
    /// * `alt` - A button that act like Alt key.
    /// * `tab` - A button that act like tab key.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::*;
    /// let hotkey = Hotkey::new();
    /// hotkey.bind_shift_alt_tab(Button::A, Button::R);
    /// ```
    fn bind_shift_alt_tab(&self, alt: ButtonArgs, tab: ButtonArgs) {
        hotkey!(self => {
            on_release [alt] => move |_| {
                IS_ALT_TAB_WORKING.store(false, Ordering::SeqCst);
                seq!(LAlt up);
            };

            modifier [alt] {
                disable [tab];
                on_press [tab] => move |_| {
                    if !IS_ALT_TAB_WORKING.swap(true, Ordering::SeqCst) {
                        seq!(LAlt down);
                    }
                    seq!(with(LShift), Tab);
                };
            }
        });
    }
}

impl<T: RegisterHotkey> Utils for T {}
