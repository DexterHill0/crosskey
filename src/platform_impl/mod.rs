#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_os = "windows")]
pub(crate) mod windows;

#[cfg(target_os = "linux")]
pub(crate) use crate::platform_impl::linux::LinuxListener as PlatformListener;
#[cfg(target_os = "macos")]
pub(crate) use crate::platform_impl::macos::MacOsListener as PlatformListener;
#[cfg(target_os = "windows")]
pub(crate) use crate::platform_impl::windows::WindowsListener as PlatformListener;
