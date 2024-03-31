use std::fmt::{self, Display};

use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardLayout, ToUnicodeEx};

use crate::{Key, KeyEvent};

use super::translate_key::ToKeyboardState;

impl Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.key {
            // this displays using the given the modifiers of the keypress
            // which is unlike the character contained in Key::Character as that is created with the `NO_MODIFIERS` state
            Key::Character(_) => {
                let kb_layout = unsafe { GetKeyboardLayout(0) };

                // `WCHAR` is UTF-16, so 2 bytes
                let key_char: &mut [u16] = &mut [0];

                let result = unsafe {
                    ToUnicodeEx(
                        self.raw.virtual_key_code,
                        self.raw.virtual_scan_code,
                        &self.modifiers.to_keyboard_state(),
                        key_char,
                        1,
                        kb_layout,
                    )
                };

                if result == 0 {
                    return write!(f, "");
                }

                let str_char = String::from_utf16(key_char).unwrap_or_default();

                write!(f, "{}", str_char)
            },
            key => {
                write!(f, "{}", key)
            },
        }
    }
}
