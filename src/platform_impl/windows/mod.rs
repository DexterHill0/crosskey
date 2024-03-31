pub mod key_display;
mod translate_key;

use std::fmt;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::RwLock;
use std::time::SystemTime;

pub use flume::TryRecvError;
use flume::{Receiver, Sender};
use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use translate_key::translate_key;
use windows::Win32::Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, SetWindowLongPtrW, UnhookWindowsHookEx, GWLP_WNDPROC, HHOOK, WM_KEYDOWN,
    WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use crate::platform_impl::platform::translate_key::get_modifiers;
use crate::{Event, KeyEvent, ListenerError};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum AttachError {
    AttachError(u32),
    PoisonError,
}

impl Display for AttachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttachError::AttachError(e) => write!(f, "failed to attach listener: ({e:#01X})"),
            AttachError::PoisonError => write!(f, "failed to attach listener: poisoned RwLock"),
        }
    }
}

lazy_static::lazy_static! {
    static ref O_WNDPROC: RwLock<isize> = RwLock::new(0);
    static ref CHANNEL: (Sender<()>, Receiver<()>) = flume::unbounded();
}

static REPEAT_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe extern "system" fn h_wndproc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Ok(wndproc) = O_WNDPROC.read() {
        match umsg {
            msg @ (WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP) => {
                let modifiers = get_modifiers();
                let (key, raw_key_event_data) = translate_key(wparam);

                let key_event = KeyEvent {
                    key,
                    modifiers,
                    #[cfg(feature = "timestamp")]
                    timestamp: SystemTime::now(),
                    raw: raw_key_event_data,
                };

                let event = if matches!(msg, WM_KEYDOWN | WM_SYSKEYDOWN) {
                    let e = Event::Press {
                        key: key_event,
                        repeat_count: REPEAT_COUNT.load(Ordering::Relaxed),
                    };

                    REPEAT_COUNT.fetch_add(1, Ordering::Relaxed);

                    e
                } else {
                    REPEAT_COUNT.store(0, Ordering::Relaxed);
                    Event::Release(key_event)
                };

                dbg!(event);
            },
            _ => {},
        }

        CallWindowProcW(std::mem::transmute(*wndproc), hwnd, umsg, wparam, lparam)
    } else {
        LRESULT(1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct RawKeyEventData {
    virtual_key_code: u32,
    virtual_scan_code: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct KeyboardListener {
    handle: Win32WindowHandle,
}

impl KeyboardListener {
    pub(crate) fn from_raw_window_handle(
        raw_window_handle: RawWindowHandle,
    ) -> Result<Self, ListenerError> {
        match raw_window_handle {
            RawWindowHandle::Win32(h) => Ok(Self { handle: h }),
            _ => Err(ListenerError::InvalidHandle),
        }
    }

    #[allow(clippy::fn_to_numeric_cast)]
    pub(crate) fn attatch(&self) -> Result<(), AttachError> {
        let result = unsafe {
            SetWindowLongPtrW(
                HWND(self.handle.hwnd.into()),
                GWLP_WNDPROC,
                h_wndproc as isize,
            )
        };

        if result == 0 {
            return Err(AttachError::AttachError(unsafe { GetLastError().0 }));
        }

        let mut wndproc = O_WNDPROC.write().map_err(|_| AttachError::PoisonError)?;
        *wndproc = result;

        Ok(())
    }

    /// See: [`KeyboardListener::try_recv`]
    ///
    /// **Note: This function is blocking!**
    pub(crate) fn recv<F>(callback: F) {
        match Self::try_recv(callback) {
            Ok(..) => (),
            Err(e) => panic!("failed to listen: {e}"),
        }
    }

    /// **Note: This function is blocking!**
    pub(crate) fn try_recv<F>(callback: F) -> Result<(), TryRecvError> {
        loop {
            let event = CHANNEL.1.try_recv()?;
            // callback(event);
        }
    }
}

impl Drop for KeyboardListener {
    fn drop(&mut self) {
        if let Ok(wndproc) = O_WNDPROC.read() {
            if unsafe { UnhookWindowsHookEx(HHOOK(*wndproc)) }.is_err() {
                let err = unsafe { GetLastError() };

                // `Invalid hook handle`
                // i'm not exactly sure what to do here
                // if you close the window and *then* the listener is dropped this error will occur
                // since the window no longer exists, but, what if this occurs for some other reason
                if err.0 != 1404 {
                    panic!("failed to remove windows hook: {err:?}");
                }
            }
        } else {
            panic!("drop called with missing windows hook");
        }
    }
}
