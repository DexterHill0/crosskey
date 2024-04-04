mod hotkey_listener_impl;
mod keyboard_listener_impl;

use std::collections::HashMap;
use std::sync::RwLock;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use super::handle_key_message;

lazy_static::lazy_static! {
    // key: HWND, value: prev window func
    pub static ref WINDOW_SUBCLASSES: RwLock<HashMap<isize, isize>> = RwLock::new(HashMap::new());
}

// window procecure shared between both the keyboard listener and hotkey listener
pub(crate) unsafe extern "system" fn h_wndproc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Ok(subclass) = WINDOW_SUBCLASSES.read() {
        match umsg {
            msg @ (WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP) => {
                handle_key_message(msg, hwnd, wparam)
            },
            _ => (),
        }

        CallWindowProcW(
            std::mem::transmute(subclass[&hwnd.0]),
            hwnd,
            umsg,
            wparam,
            lparam,
        )
    } else {
        LRESULT(1)
    }
}
