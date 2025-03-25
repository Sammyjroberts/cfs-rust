//! OSAL (Operating System Abstraction Layer) for cfs-rust
//!
//! This crate provides a Rust implementation of NASA's OSAL API

pub mod api;
pub mod error;
pub mod object_table;
pub mod portable;
pub mod types;

// Export specific API elements for easier access
pub use api::file::{os_close, os_opencreate};
pub use error::*;
pub use types::file::{
    OS_FILE_FLAG_CREATE, OS_FILE_FLAG_NONE, OS_FILE_FLAG_TRUNCATE, OS_READ_ONLY, OS_READ_WRITE,
    OS_WRITE_ONLY,
};
pub use types::file::{OSAL_ID_UNDEFINED, OsalId};
