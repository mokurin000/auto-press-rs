pub mod config;
pub mod controller;
pub mod devices;
pub mod rng;
pub mod utils;

pub mod lua_interop;

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
    #[error("Win32 Error: {0}")]
    WindowsError(#[from] windows::core::Error),
}
