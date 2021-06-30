use super::event::{EventDetail, EventHandler};
use once_cell::sync::Lazy;

pub trait EmulateKeyboardInput {
    fn press(&self);
    fn release(&self);
    fn click(&self) {
        self.press();
        self.release();
    }
    fn is_pressed(&self) -> bool;
    fn is_toggled(&self) -> bool;
}

pub type KeyboardEvent = EventDetail<Key, KeyboardAction>;
pub type KeyboardEventHandler = EventHandler<Key, KeyboardAction>;

pub static KEYBOARD_HANDLER: Lazy<KeyboardEventHandler> = Lazy::new(KeyboardEventHandler::default);

#[derive(Debug)]
pub enum KeyboardAction {
    Press,
    Release,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Backspace,
    Tab,
    Enter,
    Shift,
    Ctrl,
    Alt,
    CapsLock,
    Esc,
    Henkan,
    Muhenkan,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    PrintScreen,
    Insert,
    Delete,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LMeta,
    RMeta,
    Application,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAsterisk,
    NumpadPlus,
    NumpadMinus,
    NumpadDot,
    NumpadSlash,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Numlock,
    ScrollLock,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    Colon,
    SemiColon,
    Comma,
    Minus,
    Dot,
    Slash,
    At,
    LeftSquareBracket,
    BackSlashWithVerticalBar,
    RightSquareBracket,
    Hat,
    BackSlashWithUnderLine,
    Eisuu,
    KatakanaHiragana,
    HannkakuZenkaku,
    Other(u32),
}
