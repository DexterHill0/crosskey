mod listener;

use std::sync::RwLock;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use super::handle_key_message;

lazy_static::lazy_static! {
    pub static ref O_WNDPROC: RwLock<isize> = RwLock::new(0);
}

pub(crate) unsafe extern "system" fn h_wndproc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Ok(wndproc) = O_WNDPROC.read() {
        if let msg @ (WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP) = umsg {
            handle_key_message(msg, wparam)
        }

        CallWindowProcW(std::mem::transmute(*wndproc), hwnd, umsg, wparam, lparam)
    } else {
        LRESULT(1)
    }
}
