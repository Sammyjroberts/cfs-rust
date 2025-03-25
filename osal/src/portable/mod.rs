//! Platform-specific implementations of OSAL functionality
//!
//! This module contains code that may vary between different operating systems.

#[cfg(any(target_family = "unix"))]
pub mod posix;

// Future platform support
// #[cfg(target_family = "windows")]
// pub mod windows;
