use std::fs::OpenOptions;
use std::io;

use crate::error::*;
use crate::object_table::ObjectToken;
use crate::types::*;

/// Platform-specific implementation of file open/create
pub fn file_open_impl(
    token: &ObjectToken,
    path: &str,
    flags: i32,
    access_mode: i32,
) -> Result<(), i32> {
    let mut options = OpenOptions::new();

    // Set access mode
    match access_mode {
        OS_READ_ONLY => {
            options.read(true);
        }
        OS_WRITE_ONLY => {
            options.write(true);
        }
        OS_READ_WRITE => {
            options.read(true).write(true);
        }
        _ => return Err(OS_ERROR),
    }

    // Set flags
    if (flags & OS_FILE_FLAG_CREATE) != 0 {
        options.create(true);
    }

    if (flags & OS_FILE_FLAG_TRUNCATE) != 0 {
        options.truncate(true);
    }

    // Try to open the file
    match options.open(path) {
        Ok(file) => {
            // Store the file handle in the stream record
            if let Ok(mut table) = crate::object_table::OBJECT_TABLE.lock() {
                let stream = table.get_stream_mut(token);
                stream.file = Some(file);
                Ok(())
            } else {
                Err(OS_ERROR)
            }
        }
        Err(e) => {
            // Map IO errors to OSAL errors
            match e.kind() {
                io::ErrorKind::NotFound => Err(OS_FS_ERR_PATH_INVALID),
                io::ErrorKind::PermissionDenied => Err(OS_ERROR),
                _ => Err(OS_ERROR),
            }
        }
    }
}
