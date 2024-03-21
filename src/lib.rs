use std::fmt::{self, Display};

pub use raw_window_handle::HandleError;
use raw_window_handle::HasWindowHandle;

mod platform_impl;

pub use platform_impl::AttachError;

#[derive(Clone, Debug)]
pub enum ListenerError {
    InvalidHandle,
    HandleError(HandleError),
    AttachError(platform_impl::AttachError),
}

impl Display for ListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListenerError::InvalidHandle => write!(f, "invalid handle provided for this platform"),
            ListenerError::HandleError(h) => write!(f, "{h}"),
            ListenerError::AttachError(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug)]
pub struct KeyboardListener {
    inner: platform_impl::KeyboardListener,
}

impl KeyboardListener {
    pub fn attatch<H: HasWindowHandle>(handle: &H) -> Result<Self, ListenerError> {
        let slf = Self {
            inner: platform_impl::KeyboardListener::from_raw_window_handle(
                handle
                    .window_handle()
                    .map_err(ListenerError::HandleError)?
                    .as_raw(),
            )?,
        };

        slf.inner.attatch().map_err(ListenerError::AttachError)?;

        Ok(slf)
    }

    pub fn listen() {}

    pub fn try_listen() -> Result<(), &'static str> {
        todo!()
    }
}
