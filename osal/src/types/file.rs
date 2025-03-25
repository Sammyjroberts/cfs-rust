//! File-related type definitions for OSAL

/// File access modes
pub const OS_READ_ONLY: i32 = 0;
pub const OS_WRITE_ONLY: i32 = 1;
pub const OS_READ_WRITE: i32 = 2;

// File flags
pub const OS_FILE_FLAG_NONE: i32 = 0x00;
pub const OS_FILE_FLAG_CREATE: i32 = 0x01;
pub const OS_FILE_FLAG_TRUNCATE: i32 = 0x02;

// OSAL ID type
pub type OsalId = u32;
pub const OSAL_ID_UNDEFINED: OsalId = 0;
