use crate::file_parsing::{get_files_from_path, get_rwx_from_st_mode};
use std::collections::HashSet;
use std::path::Path;

/// Values accessed from printing the modes in main().
#[test]
fn test_get_rwx_from_st_mode() {
    let result = get_rwx_from_st_mode(16877);
    assert_eq!("rwxr-xr-x", result);
    let result = get_rwx_from_st_mode(33188);
    assert_eq!("rw-r--r--", result);
}

#[test]
#[ignore = "Fails when contents of directory running in changes."]
fn test_get_directory_files() {
    let expected_filenames: HashSet<&str> = HashSet::from([
        "Cargo.toml",
        "src",
        ".git",
        ".gitignore",
        "target",
        "Cargo.lock",
    ]);
    let result = match get_files_from_path(Path::new(".")) {
        Ok(files) => files,
        Err(_) => panic!("Error getting directory files"),
    };
    assert_eq!(result.len(), 7);
    for filename in expected_filenames {
        assert_eq!(
            true,
            result.iter().any(|lsfile| lsfile.filename == filename)
        );
    }
}
