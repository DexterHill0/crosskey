use raw_window_handle::RawWindowHandle;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_WNDPROC};

use super::{h_wndproc, WINDOW_SUBCLASSES};
use crate::platform_impl::{KeyboardListener, PlatformWindowHandle};
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
        let hwnd: isize = self.handle.hwnd.into();

        let result = unsafe { SetWindowLongPtrW(HWND(hwnd), GWLP_WNDPROC, h_wndproc as isize) };

        if result == 0 {
            return Err(AttachError::AttachError(unsafe { GetLastError().0 }));
        }

        let mut wndproc = WINDOW_SUBCLASSES
            .write()
            .map_err(|_| AttachError::PoisonError)?;

        wndproc.insert(hwnd, result);

        Ok(())
    }

    pub(crate) fn platform_window_handle(&self) -> PlatformWindowHandle {
        self.handle.hwnd.into()
    }
}

impl Drop for KeyboardListener {
    fn drop(&mut self) {
        if let Ok(wndproc) = WINDOW_SUBCLASSES.read() {
            let hwnd: isize = self.handle.hwnd.into();

            let result = unsafe { SetWindowLongPtrW(HWND(hwnd), GWLP_WNDPROC, wndproc[&hwnd]) };

            if result == 0 {
                panic!("failed to remove window procedure in KeyboardListener::drop")
            }
        } else {
            panic!("RwLock poisoned in KeyboardListener::drop ")
        }
    }
}
