mod listener;
mod platform_impl;

#[cfg(feature = "hotkeys")]
mod hotkeys;

use std::time::SystemTime;

#[cfg(feature = "hotkeys")]
pub use hotkeys::*;
use kanal::{Receiver, Sender};
pub use listener::*;

/// Re-exported from [`keyboard-types`](https://crates.io/crates/keyboard-types)
pub type Key = keyboard_types::Key;
/// Re-exported from [`keyboard-types`](https://crates.io/crates/keyboard-types)
pub type Modifiers = keyboard_types::Modifiers;

#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    pub timestamp: SystemTime,

    raw: platform_impl::RawKeyEventData,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Event {
    Press { key: KeyEvent, repeat_count: usize },
    Release(KeyEvent),
}

lazy_static::lazy_static! {
    pub static ref CHANNEL: (Sender<Event>, Receiver<Event>) = kanal::unbounded();
}
