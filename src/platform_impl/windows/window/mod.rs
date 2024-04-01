mod hotkey_listener_impl;
mod keyboard_listener_impl;

use std::sync::RwLock;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use super::handle_key_message;

lazy_static::lazy_static! {
    // (WNDPROC, HWND)
    // the hwnd is required in the `Drop` impl to set the window procedure back to the
    // value held in WNDPROC
    pub static ref O_WNDPROC_HWND: RwLock<(isize, isize)> = RwLock::new((0, 0));
}

// window procecure shared between both the keyboard listener and hotkey listener
pub(crate) unsafe extern "system" fn h_wndproc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Ok(wndproc) = O_WNDPROC_HWND.read() {
        if let msg @ (WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP) = umsg {
            handle_key_message(msg, wparam)
        }

        CallWindowProcW(std::mem::transmute(wndproc.0), hwnd, umsg, wparam, lparam)
    } else {
        LRESULT(1)
    }
}
