// src/api/file.rs

use crate::error::*;
use crate::object_table::{OBJECT_TABLE, ObjectType};
use crate::portable::posix;
use crate::{OS_FS_ERR_PATH_INVALID, OS_FS_ERR_PATH_TOO_LONG, types::*};
// Constants
const OS_MAX_LOCAL_PATH_LEN: usize = 264; // Same as in C OSAL

/// Open or create a file
///
/// Implements the same behavior as the C OSAL OS_OpenCreate function
///
/// # Arguments
/// * `path` - File name to create or open
/// * `flags` - The file permissions
/// * `access_mode` - Intended access mode
///
/// # Returns
/// * On success, returns OS_SUCCESS and sets the file descriptor
/// * On failure, returns an appropriate error code
pub fn os_opencreate(filedes: &mut OsalId, path: &str, flags: i32, access_mode: i32) -> i32 {
    // Check parameters
    if path.is_empty() {
        return OS_INVALID_POINTER;
    }

    // Initialize file descriptor
    *filedes = OSAL_ID_UNDEFINED;

    // Check for a valid access mode
    if access_mode != OS_WRITE_ONLY && access_mode != OS_READ_ONLY && access_mode != OS_READ_WRITE {
        return OS_ERROR;
    }

    // Translate the path
    let local_path = match os_translate_path(path) {
        Ok(p) => p,
        Err(e) => return e,
    };

    // Get access to the object table
    let mut object_table = match OBJECT_TABLE.lock() {
        Ok(table) => table,
        Err(_) => return OS_ERROR,
    };

    // Allocate a new object ID
    let (token, return_code) = match object_table.allocate_id(ObjectType::Stream, None) {
        Ok(token) => (token, OS_SUCCESS),
        Err(e) => return e,
    };

    if return_code == OS_SUCCESS {
        // Get the stream record
        let stream = object_table.get_stream_mut(&token);

        // Initialize the stream entry and save the name
        stream.name = path.to_string();
        stream.in_use = true;

        // Call the OS-specific implementation
        let impl_result = posix::file_open_impl(&token, &local_path, flags, access_mode);

        // Check result, finalize record, and return
        match impl_result {
            Ok(_) => {
                // Success - set the file descriptor
                *filedes = token.to_osal_id();
                OS_SUCCESS
            }
            Err(e) => {
                // Implementation failed - clean up
                object_table.free_id(&token);
                e
            }
        }
    } else {
        return_code
    }
}

/// Close a file descriptor
///
/// This function closes a file descriptor that was returned by os_opencreate
///
/// # Arguments
/// * `filedes` - The file descriptor to close
///
/// # Returns
/// * OS_SUCCESS if successful
/// * Appropriate error code if not
pub fn os_close(filedes: OsalId) -> i32 {
    use crate::error::*;

    // Get access to the object table
    let mut object_table = match OBJECT_TABLE.lock() {
        Ok(table) => table,
        Err(_) => return OS_ERROR,
    };

    // Validate the file descriptor
    let token = match object_table.id_to_token(filedes) {
        Ok(token) => token,
        Err(e) => return e,
    };

    // Make sure it's a stream object
    if token.obj_type != ObjectType::Stream {
        return OS_ERR_INVALID_ID;
    }

    // Get the stream record
    let stream = object_table.get_stream_mut(&token);

    // Close the file if it's open
    if let Some(file) = stream.file.take() {
        // File will be closed when dropped
        drop(file);
    }

    // Free the ID
    object_table.free_id(&token);

    OS_SUCCESS
}

/// Translate a path from logical to physical
fn os_translate_path(path: &str) -> Result<String, i32> {
    // Check path length
    if path.len() >= OS_MAX_LOCAL_PATH_LEN {
        return Err(OS_FS_ERR_PATH_TOO_LONG);
    }

    // Validate path
    if path.is_empty() {
        return Err(OS_FS_ERR_PATH_INVALID);
    }

    // In a real implementation, this would do more complex path translation
    // based on mount points, etc. For now, we just return the path as-is.
    Ok(path.to_string())
}
