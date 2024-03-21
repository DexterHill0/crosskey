use std::fmt::Display;
use std::sync::RwLock;
use std::{fmt, sync::Arc};

use flume::{Receiver, Sender};
use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use windows::Win32::UI::WindowsAndMessaging::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP};
use windows::Win32::{
    Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrW, GWLP_WNDPROC},
};

pub use flume::TryRecvError;

use crate::ListenerError;

#[derive(Clone, Debug)]
pub enum AttachError {
    AttachError(u32),
    PoisonError,
}

impl Display for AttachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttachError::AttachError(e) => write!(f, "failed to attach listener: ({e})"),
            AttachError::PoisonError => write!(f, "failed to attach listener: poisoned RwLock"),
        }
    }
}

lazy_static::lazy_static! {
    static ref O_WNDPROC: RwLock<isize> = RwLock::new(0);
    static ref CHANNEL: (Sender<()>, Receiver<()>) = flume::unbounded();
}

unsafe extern "system" fn h_wndproc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Ok(wndproc) = O_WNDPROC.read() {
        match umsg {
            msg @ (WM_KEYDOWN | WM_SYSKEYDOWN) => {
                println!("key down!")
            }
            msg @ (WM_KEYUP | WM_SYSKEYUP) => {
                println!("key up!")
            }
            _ => {}
        }

        CallWindowProcW(std::mem::transmute(*wndproc), hwnd, umsg, wparam, lparam)
    } else {
        LRESULT(1)
    }
}

#[derive(Debug)]
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
