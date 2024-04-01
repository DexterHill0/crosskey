use crate::platform_impl;

#[derive(Clone, Debug)]
pub struct HotkeyListener {
    inner: platform_impl::HotkeyListener,
}
