mod platform_impl;

use std::fmt::{self, Display};
#[cfg(feature = "timestamp")]
use std::time::SystemTime;

use flume::TryRecvError;
pub use raw_window_handle::HandleError;
use raw_window_handle::HasWindowHandle;

/// Re-exported from [`keyboard-types`](https://crates.io/crates/keyboard-types)
pub type Key = keyboard_types::Key;
/// Re-exported from [`keyboard-types`](https://crates.io/crates/keyboard-types)
pub type Modifiers = keyboard_types::Modifiers;

pub use crate::platform_impl::AttachError;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum ListenerError {
    InvalidHandle,
    HandleError(HandleError),
    AttachError(platform_impl::AttachError),
}

impl Display for ListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListenerError::InvalidHandle => write!(f, "invalid handle provided for this platform"),
            ListenerError::HandleError(h) => write!(f, "{h}"),
            ListenerError::AttachError(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,

    #[cfg_attr(all(feature = "serde", not(feature = "timestamp")), serde(skip))]
    #[cfg(feature = "timestamp")]
    pub timestamp: SystemTime,

    raw: platform_impl::RawKeyEventData,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Event {
    Press { key: KeyEvent, repeat_count: usize },
    Release(KeyEvent),
}

#[derive(Clone, Debug)]
pub struct KeyboardListener {
    inner: platform_impl::KeyboardListener,
}

impl KeyboardListener {
    pub fn attatch<H: HasWindowHandle>(handle: &H) -> Result<Self, ListenerError> {
        let slf = Self {
            inner: platform_impl::KeyboardListener::from_raw_window_handle(
                handle
                    .window_handle()
                    .map_err(ListenerError::HandleError)?
                    .as_raw(),
            )?,
        };

        slf.inner.attatch().map_err(ListenerError::AttachError)?;

        Ok(slf)
    }

    /// See: [`KeyboardListener::try_recv`]
    ///
    /// **Note: This function is blocking!**
    pub fn recv<F>(callback: F)
    where
        F: Fn(Event),
    {
        platform_impl::KeyboardListener::recv(callback)
    }

    /// **Note: This function is blocking!**
    pub fn try_recv<F>(callback: F) -> Result<(), TryRecvError>
    where
        F: Fn(Event),
    {
        platform_impl::KeyboardListener::try_recv(callback)
    }
}
