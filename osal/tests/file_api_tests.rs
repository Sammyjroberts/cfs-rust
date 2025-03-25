#[cfg(test)]
mod tests {
    use cfs_rust_osal::{
        OS_FILE_FLAG_CREATE, OS_FILE_FLAG_NONE, OS_FILE_FLAG_TRUNCATE, OS_READ_ONLY, OS_READ_WRITE,
        OS_SUCCESS, OSAL_ID_UNDEFINED, OsalId, os_close, os_opencreate,
    };

    #[test]
    fn test_create_and_close_file() {
        let filename = "/drive0/Filename1";
        let mut filedes: OsalId = OSAL_ID_UNDEFINED;

        // Create a file
        let status = os_opencreate(
            &mut filedes,
            filename,
            OS_FILE_FLAG_CREATE | OS_FILE_FLAG_TRUNCATE,
            OS_READ_WRITE,
        );

        assert_eq!(
            status, OS_SUCCESS,
            "File creation failed with status: {}",
            status
        );
        assert_ne!(
            filedes, OSAL_ID_UNDEFINED,
            "File descriptor is undefined after creation"
        );

        // Close the file
        let status = os_close(filedes);
        assert_eq!(
            status, OS_SUCCESS,
            "File close failed with status: {}",
            status
        );
    }

    #[test]
    fn test_reopen_file() {
        let filename = "/drive0/Filename1";
        let mut filedes: OsalId = OSAL_ID_UNDEFINED;

        // First create the file
        let status = os_opencreate(
            &mut filedes,
            filename,
            OS_FILE_FLAG_CREATE | OS_FILE_FLAG_TRUNCATE,
            OS_READ_WRITE,
        );
        assert_eq!(status, OS_SUCCESS);

        // Close it
        let status = os_close(filedes);
        assert_eq!(status, OS_SUCCESS);

        // Reopen the file
        let status = os_opencreate(&mut filedes, filename, OS_FILE_FLAG_NONE, OS_READ_WRITE);

        assert_eq!(
            status, OS_SUCCESS,
            "File reopen failed with status: {}",
            status
        );
        assert_ne!(
            filedes, OSAL_ID_UNDEFINED,
            "File descriptor is undefined after reopen"
        );

        // Clean up
        let status = os_close(filedes);
        assert_eq!(status, OS_SUCCESS);
    }

    #[test]
    fn test_close_already_closed_file() {
        let filename = "/drive0/Filename1";
        let mut filedes: OsalId = OSAL_ID_UNDEFINED;

        // Create and close a file
        let status = os_opencreate(
            &mut filedes,
            filename,
            OS_FILE_FLAG_CREATE | OS_FILE_FLAG_TRUNCATE,
            OS_READ_WRITE,
        );
        assert_eq!(status, OS_SUCCESS);

        let fd_to_reuse = filedes;

        let status = os_close(filedes);
        assert_eq!(status, OS_SUCCESS);

        // Try to close it again - should fail
        let status = os_close(fd_to_reuse);
        assert_ne!(
            status, OS_SUCCESS,
            "Closing already closed file should fail"
        );
    }

    #[test]
    fn test_close_invalid_handle() {
        // Try to close an invalid handle
        let status = os_close(OSAL_ID_UNDEFINED);
        assert_ne!(status, OS_SUCCESS, "Closing invalid handle should fail");
    }

    #[test]
    fn test_open_nonexistent_file() {
        let mut filedes: OsalId = OSAL_ID_UNDEFINED;

        // Try to open a file that doesn't exist
        let status = os_opencreate(
            &mut filedes,
            "/drive0/FileNotHere",
            OS_FILE_FLAG_NONE,
            OS_READ_ONLY,
        );

        assert_ne!(status, OS_SUCCESS, "Opening non-existent file should fail");
        assert_eq!(
            filedes, OSAL_ID_UNDEFINED,
            "File descriptor should remain undefined after failed open"
        );
    }
}
