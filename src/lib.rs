pub mod config;
pub mod controller;
pub mod rng;
pub mod utils;

#[cfg(feature = "device-info")]
pub mod devices;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interception initialization failed")]
    InterceptionInitFailed,
    #[error("Invalid scan 1 make code found")]
    InvalidScanCode,
    #[error("Invalid mouse button name")]
    InvalidMouseButton,
    #[error("Keyboard device not found")]
    KeyboardNotFound,
    #[error("Mouse device not found")]
    MouseNotFound,

    #[cfg(feature = "device-info")]
    #[error("Win32 Error: {0}")]
    WindowsError(#[from] windows::core::Error),
}
