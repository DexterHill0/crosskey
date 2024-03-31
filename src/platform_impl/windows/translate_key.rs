use windows::core::PCSTR;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Globalization::{WideCharToMultiByte, CP_UTF8};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, GetKeyboardLayout, GetKeyboardState, MapVirtualKeyExW, ToUnicodeEx,
    MAPVK_VK_TO_VSC, VIRTUAL_KEY, VK_ACCEPT, VK_ATTN, VK_BROWSER_BACK, VK_BROWSER_FAVORITES,
    VK_BROWSER_FORWARD, VK_BROWSER_HOME, VK_BROWSER_REFRESH, VK_BROWSER_SEARCH, VK_BROWSER_STOP,
    VK_CANCEL, VK_CAPITAL, VK_CLEAR, VK_CONTROL, VK_CONVERT, VK_DELETE, VK_END, VK_ESCAPE,
    VK_EXECUTE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17, VK_F18,
    VK_F19, VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7,
    VK_F8, VK_F9, VK_HELP, VK_HOME, VK_INSERT, VK_LAUNCH_MAIL, VK_LCONTROL, VK_LMENU, VK_LSHIFT,
    VK_LWIN, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_STOP, VK_MENU, VK_NUMLOCK, VK_PAUSE, VK_PLAY, VK_PRINT,
    VK_RCONTROL, VK_RETURN, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SCROLL, VK_SELECT, VK_SHIFT, VK_TAB,
};

use crate::{Key, Modifiers};

type KeyboardState = [u8; 256];

const NO_MODIFIERS: [u8; 256] = [0; 256];

const PRESSED: u8 = 0b10000000;
const TOGGLED: u8 = 0b00000001;

pub(crate) trait ToKeyboardState {
    fn to_keyboard_state(&self) -> KeyboardState;
}

impl ToKeyboardState for Modifiers {
    fn to_keyboard_state(&self) -> KeyboardState {
        macro_rules! i {
            ($vk:expr) => {
                $vk.0 as usize
            };
        }

        let mut kb_state: KeyboardState = [0; 256];

        if self.contains(Modifiers::ALT) {
            kb_state[i!(VK_MENU)] = PRESSED;
        }

        if self.contains(Modifiers::CONTROL) {
            kb_state[i!(VK_CONTROL)] = PRESSED;
        }

        if self.contains(Modifiers::SHIFT) {
            kb_state[i!(VK_SHIFT)] = PRESSED;
        }

        if self.contains(Modifiers::META) || self.contains(Modifiers::SUPER) {
            kb_state[i!(VK_LWIN)] = PRESSED;
            kb_state[i!(VK_RWIN)] = PRESSED;
        }

        if self.contains(Modifiers::CAPS_LOCK) {
            kb_state[i!(VK_CAPITAL)] = TOGGLED;
        }

        if self.contains(Modifiers::SCROLL_LOCK) {
            kb_state[i!(VK_SCROLL)] = TOGGLED;
        }

        if self.contains(Modifiers::NUM_LOCK) {
            kb_state[i!(VK_NUMLOCK)] = TOGGLED;
        }

        kb_state
    }
}

pub fn get_modifiers() -> Modifiers {
    let mut modifiers = Modifiers::empty();

    macro_rules! pressed {
        ($vk:expr) => {{
            unsafe { GetKeyState($vk.0.into()) & (PRESSED as i16) != 0 }
        }};
    }
    macro_rules! toggled {
        ($vk:expr) => {{
            unsafe { GetKeyState($vk.0.into()) & (TOGGLED as i16) == 1 }
        }};
    }

    if pressed!(VK_MENU) || pressed!(VK_LMENU) || pressed!(VK_RMENU) {
        modifiers.insert(Modifiers::ALT);
    }

    if pressed!(VK_CONTROL) || pressed!(VK_LCONTROL) || pressed!(VK_RCONTROL) {
        modifiers.insert(Modifiers::CONTROL);
    }

    if pressed!(VK_SHIFT) || pressed!(VK_LSHIFT) || pressed!(VK_RSHIFT) {
        modifiers.insert(Modifiers::SHIFT);
    }

    if pressed!(VK_LWIN) || pressed!(VK_RWIN) {
        // should these both be set?
        modifiers.insert(Modifiers::SUPER);
        modifiers.insert(Modifiers::META);
    }

    if toggled!(VK_CAPITAL) {
        modifiers.insert(Modifiers::CAPS_LOCK);
    }

    if toggled!(VK_SCROLL) {
        modifiers.insert(Modifiers::SCROLL_LOCK);
    }

    if toggled!(VK_NUMLOCK) {
        modifiers.insert(Modifiers::NUM_LOCK);
    }

    modifiers
}

pub fn translate_key(raw_key_code: WPARAM) -> Key {
    let key_code: u32 = match raw_key_code.0.try_into() {
        Ok(k) => k,
        Err(_) => return Key::Unidentified,
    };

    let kb_layout = unsafe { GetKeyboardLayout(0) };

    let scan_code = unsafe { MapVirtualKeyExW(key_code, MAPVK_VK_TO_VSC, kb_layout) };

    // `WCHAR` is UTF-16, so 2 bytes
    let key_char: &mut [u16] = &mut [0];

    let result = unsafe { ToUnicodeEx(key_code, scan_code, &NO_MODIFIERS, key_char, 1, kb_layout) };

    if result == 0 {
        return Key::Unidentified;
    }

    let char_code = key_char[0];

    if (0x0000..=0x001F).contains(&char_code) || (0x007F..=0x009F).contains(&char_code) {
        // non printable UTF-16 characters

        // as far as i can tell these are all the VK keys that have a keyboard-types equivalent
        #[deny(unused_variables, non_snake_case)]
        match VIRTUAL_KEY(key_code as u16) {
            VK_MENU => Key::Alt,
            VK_LMENU => Key::Alt,
            VK_RMENU => Key::AltGraph,
            VK_RETURN => Key::Enter,
            VK_CONTROL => Key::Control,
            VK_LCONTROL => Key::Control,
            VK_RCONTROL => Key::Control,
            VK_SHIFT => Key::Shift,
            VK_LSHIFT => Key::Shift,
            VK_RSHIFT => Key::Shift,
            VK_LWIN => Key::Super,
            VK_RWIN => Key::Super,
            VK_CAPITAL => Key::CapsLock,
            VK_SCROLL => Key::ScrollLock,
            VK_NUMLOCK => Key::NumLock,
            VK_TAB => Key::Tab,
            VK_END => Key::End,
            VK_HOME => Key::Home,
            VK_CLEAR => Key::Clear,
            VK_DELETE => Key::Delete,
            VK_INSERT => Key::Insert,
            VK_ACCEPT => Key::Accept,
            VK_ATTN => Key::Attn,
            VK_CANCEL => Key::Cancel,
            VK_ESCAPE => Key::Escape,
            VK_EXECUTE => Key::Execute,
            VK_HELP => Key::Help,
            VK_PAUSE => Key::Pause,
            VK_PLAY => Key::Play,
            VK_SELECT => Key::Select,
            VK_CONVERT => Key::Convert,
            VK_MEDIA_PLAY_PAUSE => Key::MediaPlayPause,
            VK_MEDIA_STOP => Key::MediaStop,
            VK_PRINT => Key::Print,
            VK_LAUNCH_MAIL => Key::LaunchMail,
            VK_BROWSER_BACK => Key::BrowserBack,
            VK_BROWSER_FAVORITES => Key::BrowserFavorites,
            VK_BROWSER_FORWARD => Key::BrowserForward,
            VK_BROWSER_HOME => Key::BrowserHome,
            VK_BROWSER_REFRESH => Key::BrowserRefresh,
            VK_BROWSER_SEARCH => Key::BrowserSearch,
            VK_BROWSER_STOP => Key::BrowserStop,
            VK_F1 => Key::F1,
            VK_F2 => Key::F2,
            VK_F3 => Key::F3,
            VK_F4 => Key::F4,
            VK_F5 => Key::F5,
            VK_F6 => Key::F6,
            VK_F7 => Key::F7,
            VK_F8 => Key::F8,
            VK_F9 => Key::F9,
            VK_F10 => Key::F10,
            VK_F11 => Key::F11,
            VK_F12 => Key::F12,
            VK_F13 => Key::F13,
            VK_F14 => Key::F14,
            VK_F15 => Key::F15,
            VK_F16 => Key::F16,
            VK_F17 => Key::F17,
            VK_F18 => Key::F18,
            VK_F19 => Key::F19,
            VK_F20 => Key::F20,
            VK_F21 => Key::F21,
            VK_F22 => Key::F22,
            VK_F23 => Key::F23,
            VK_F24 => Key::F24,
            _ => Key::Unidentified,
        }
    } else {
        // printable UTF-16 characters

        let str_char = String::from_utf16(key_char);

        match str_char {
            Ok(c) => Key::Character(c),
            Err(_) => Key::Unidentified,
        }
    }
}
