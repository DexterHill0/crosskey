#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
#[path = "linux/mod.rs"]
pub(crate) mod platform;
#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
pub(crate) mod platform;
#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
pub(crate) mod platform;

pub use self::platform::*;
