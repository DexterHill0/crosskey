// use raw_window_handle::RawWindowHandle;

// use crate::platform_impl::HotkeyListener;
// use crate::{AttachError, ListenerError};

// impl HotkeyListener {
//     pub(crate) fn from_raw_window_handle(
//         raw_window_handle: RawWindowHandle,
//     ) -> Result<Self, ListenerError> {
//         match raw_window_handle {
//             RawWindowHandle::Win32(h) => Ok(Self { handle: h }),
//             _ => Err(ListenerError::InvalidHandle),
//         }
//     }

//     #[allow(clippy::fn_to_numeric_cast)]
//     pub(crate) fn attatch(&self) -> Result<(), AttachError> {
//         let hwnd: isize = self.handle.hwnd.into();

//         let result = unsafe { SetWindowLongPtrW(HWND(hwnd), GWLP_WNDPROC, h_wndproc as isize) };

//         if result == 0 {
//             return Err(AttachError::AttachError(unsafe { GetLastError().0 }));
//         }

//         let mut wndproc = O_WNDPROC_HWND
//             .write()
//             .map_err(|_| AttachError::PoisonError)?;

//         *wndproc = (result, hwnd);

//         Ok(())
//     }
// }
