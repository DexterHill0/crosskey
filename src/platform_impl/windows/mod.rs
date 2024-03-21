use raw_window_handle::{RawWindowHandle, Win32WindowHandle};

use crate::ListenerError;

pub(crate) struct WindowsListener {
    handle: Win32WindowHandle,
}

impl WindowsListener {
    pub(crate) fn from_raw_window_handle(
        raw_window_handle: RawWindowHandle,
    ) -> Result<Self, ListenerError> {
        match raw_window_handle {
            RawWindowHandle::Win32(h) => Ok(Self { handle: h }),
            _ => Err(ListenerError::InvalidHandle),
        }
    }
}
