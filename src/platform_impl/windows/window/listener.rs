use raw_window_handle::RawWindowHandle;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_WNDPROC};

use super::{h_wndproc, O_WNDPROC};
use crate::platform_impl::KeyboardListener;
use crate::{AttachError, ListenerError};

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
}

impl Drop for KeyboardListener {
    fn drop(&mut self) {
        if let Ok(_wndproc) = O_WNDPROC.read() {
            todo!()
        }
    }
}
