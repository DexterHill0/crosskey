use std::fmt::{self, Display};

pub use raw_window_handle::HandleError;
use raw_window_handle::HasWindowHandle;

mod platform_impl;

#[derive(Clone, Debug)]
pub enum ListenerError {
    InvalidHandle,
    HandleError(HandleError),
}

impl Display for ListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListenerError::InvalidHandle => write!(f, "invalid handle provided for this platform"),
            ListenerError::HandleError(h) => write!(f, "{h}"),
        }
    }
}

pub struct KeyboardListener {
    inner: platform_impl::PlatformListener,
}

impl KeyboardListener {
    pub fn new<H: HasWindowHandle>(handle: H) -> Result<Self, ListenerError> {
        Ok(Self {
            inner: platform_impl::PlatformListener::from_raw_window_handle(
                handle
                    .window_handle()
                    .map_err(ListenerError::HandleError)?
                    .as_raw(),
            )?,
        })
    }
}
