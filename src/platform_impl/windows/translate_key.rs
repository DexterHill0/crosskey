use windows::Win32::{
    Foundation::WPARAM,
    UI::Input::KeyboardAndMouse::{GetKeyboardLayout, MapVirtualKeyExW, MAPVK_VK_TO_VSC},
};

/// this function does not fail and instead will return an unknown key if
/// anything is incorrect
pub fn translate_key(raw_key_code: WPARAM) {
    // TODO: should this be called for every key?
    let keyboard_layout = unsafe { GetKeyboardLayout(0) };

    let raw_key_code: u32 = match raw_key_code.0.try_into() {
        Ok(kc) => kc,
        Err(_) => return,
    };

    let vk_scan_code = unsafe { MapVirtualKeyExW(raw_key_code, MAPVK_VK_TO_VSC, keyboard_layout) };

    if vk_scan_code == 0 {
        return;
    }

    println!("mapped kb layout: {vk_scan_code}")
}
