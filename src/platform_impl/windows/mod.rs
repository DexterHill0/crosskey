mod key_display;
mod translate_key;
mod window;

#[cfg(feature = "global")]
mod global;

use std::fmt::{self, Display};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

use raw_window_handle::Win32WindowHandle;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::WindowsAndMessaging::{WM_KEYDOWN, WM_SYSKEYDOWN};

use self::translate_key::get_modifiers;
use crate::platform_impl::platform::translate_key::translate_key;
use crate::{Event, KeyEvent, CHANNEL};

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

static REPEAT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn handle_key_message(msg: u32, wparam: WPARAM) {
    let modifiers = get_modifiers();
    let (key, raw_key_event_data) = translate_key(wparam);

    let key_event = KeyEvent {
        key,
        modifiers,
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

    let _ = CHANNEL.0.send(event);
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

#[cfg(feature = "hotkeys")]
#[derive(Clone, Debug)]
pub(crate) struct HotkeyListener {}
