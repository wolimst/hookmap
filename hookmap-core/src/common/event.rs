use super::button::{Button, ButtonAction};
use std::{collections::HashMap, hash::Hash, sync::Mutex};

/// Indicates whether to pass the generated event to the next program or not .
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventBlock {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Unblock,
}

impl Default for &EventBlock {
    fn default() -> Self {
        if cfg!(feature = "block-input-event") {
            &EventBlock::Block
        } else {
            &EventBlock::Unblock
        }
    }
}

impl Default for EventBlock {
    fn default() -> Self {
        *<&EventBlock>::default()
    }
}

#[derive(Debug)]
pub struct ButtonEventBlockMap(Mutex<HashMap<Button, EventBlock>>);

impl ButtonEventBlockMap {
    pub fn get_or_default(&self, k: Button) -> EventBlock {
        *self.0.lock().unwrap().get(&k).unwrap_or_default()
    }

    pub fn insert(&self, k: Button, v: EventBlock) -> Option<EventBlock> {
        self.0.lock().unwrap().insert(k, v)
    }
}

impl Default for ButtonEventBlockMap {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Information about the generated event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonEvent {
    /// Target of the generated event.
    pub target: Button,

    /// Action of the generated event.
    pub action: ButtonAction,
}

impl ButtonEvent {
    /// Creates a new `ButtonEvent<T, A>`.
    pub fn new(target: Button, action: ButtonAction) -> Self {
        Self { target, action }
    }
}
