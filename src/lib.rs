#![allow(clippy::type_complexity)]

mod platform_impl;

#[cfg(feature = "hotkeys")]
mod hotkey_listener;

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::RwLock;
use std::time::SystemTime;

#[cfg(feature = "hotkeys")]
pub use hotkey_listener::*;
use kanal::{Receiver, Sender};
pub use raw_window_handle::HandleError;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

pub use crate::platform_impl::AttachError;

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Event {
    Press { key: KeyEvent, repeat_count: usize },
    Release(KeyEvent),
}

#[derive(PartialEq, Hash, Eq)]
pub(crate) struct SendSyncRwh(pub platform_impl::PlatformWindowHandle);
// SAFETY: the data itself is not being sent across threads
unsafe impl Send for SendSyncRwh {}
unsafe impl Sync for SendSyncRwh {}

lazy_static::lazy_static! {
    pub(crate) static ref CHANNELS: RwLock<HashMap<SendSyncRwh, (Sender<Event>, Receiver<Event>)>> = RwLock::new(HashMap::new());
}

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

pub enum ReceiveError {
    PoisonError,
    Kanal(kanal::ReceiveError),
}

impl Display for ReceiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReceiveError::PoisonError => {
                write!(f, "RwLock poisoned in try_recv (multi_window enabled)")
            },
            ReceiveError::Kanal(k) => write!(f, "{k}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyboardListener {
    inner: platform_impl::KeyboardListener,
}

impl KeyboardListener {
    pub fn attatch<H: HasWindowHandle>(handle: &H) -> Result<Self, ListenerError> {
        let rwh = handle
            .window_handle()
            .map_err(ListenerError::HandleError)?
            .as_raw();

        let slf = Self {
            inner: platform_impl::KeyboardListener::from_raw_window_handle(rwh)?,
        };

        CHANNELS
            .write()
            .map_err(|_| ListenerError::AttachError(AttachError::PoisonError))?
            .insert(
                SendSyncRwh(slf.inner.platform_window_handle()),
                kanal::unbounded(),
            );

        slf.inner.attatch().map_err(ListenerError::AttachError)?;

        Ok(slf)
    }

    /// See: [`KeyboardListener::try_recv`]
    ///
    /// **Note: This function is blocking!**
    pub fn recv<F>(&self, callback: F)
    where
        F: Fn(Event),
    {
        match self.try_recv(callback) {
            Ok(..) => (),
            Err(e) => panic!("failed to receive: {e}"),
        }
    }

    /// **Note: This function is blocking!**
    pub fn try_recv<F>(&self, callback: F) -> Result<(), ReceiveError>
    where
        F: Fn(Event),
    {
        let channels = CHANNELS.read().map_err(|_| ReceiveError::PoisonError)?;
        let channel = channels
            .get(&SendSyncRwh(self.inner.platform_window_handle()))
            .unwrap();

        loop {
            let event = channel.1.recv().map_err(ReceiveError::Kanal)?;
            callback(event);
        }
    }
}
