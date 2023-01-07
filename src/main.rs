use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Debug)]
struct LsFile {
    filename: String,
    privileges: String,
    last_edit_time: String,
    size: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filepath_arg = &args[1];
        let dir = Path::new(filepath_arg);
        if let Ok(files) = get_files_from_path(dir) {
            display_files(files);
        } else {
            println!("Failed to run in {} directory", dir.display())
        }
    } else {
        if let Ok(files) = get_files_from_path(Path::new(".")) {
            display_files(files);
        } else {
            println!("Failed to run in \".\" directory")
        }
    }
}

/// displays the files to the console
fn display_files(files: HashSet<LsFile>) {
    for file in files {
        println!("{:?}", file);
    }
}

/// returns a HashSet of LsFile structs
fn get_files_from_path(filepath: &Path) -> Result<HashSet<LsFile>, Error> {
    let mut files: HashSet<LsFile> = Default::default();
    if let Some(file_path_string) = filepath.to_str() {
        if let Ok(dir_contents) = fs::read_dir(file_path_string) {
            for dir_entry in dir_contents {
                let filepath = dir_entry?;
                if let Some(filename) = filepath.file_name().to_str() {
                    let meta = fs::metadata(&filename.to_string())?;
                    let permissions = meta.permissions();
                    let mode = permissions.mode();
                    let lsfile: LsFile = LsFile {
                        filename: filename.to_string(),
                        privileges: get_rwx_from_st_mode(mode),
                        last_edit_time: "".to_string(),
                        size: 0,
                    };
                    files.insert(lsfile);
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Failed to convert filepath to string.",
                    ));
                };
            }
            return Ok(files);
        } else {
            if let Ok(_) = std::fs::File::open(filepath) {
                let lsfile: LsFile = LsFile {
                    filename: filepath.to_str().unwrap().to_string(),
                    privileges: "".to_string(),
                    last_edit_time: "".to_string(),
                    size: 0,
                };
                files.insert(lsfile);
                return Ok(files);
            } else {
                return Err(Error::new(ErrorKind::Other, "Failed to open filepath."));
            }
        }
    };
    return Ok(files);
}

/// Convert the base10 OS st_mode to rwx format.
/// Declaring the parameter base_10 is somewhat redundant
/// given it is of type u32.
fn get_rwx_from_st_mode(base_10_st_mode: u32) -> String {
    let mode_octal: String = format!("{:o}", base_10_st_mode);
    let last_three_digits: String = mode_octal.chars().rev().take(3).collect();
    // have to .rev() again because chars().rev().take() "pops"
    // the last three values, the "popping" makes them backwards for our use.
    let last_three_digits_reversed: String = last_three_digits.chars().rev().collect();
    let mut rwx_repr: String = "".to_string();
    for char in last_three_digits_reversed.chars() {
        match char.to_string().as_str() {
            "0" => rwx_repr.push_str("---"),
            "1" => rwx_repr.push_str("--x"),
            "3" => rwx_repr.push_str("-wx"),
            "4" => rwx_repr.push_str("r--"),
            "5" => rwx_repr.push_str("r-x"),
            "6" => rwx_repr.push_str("rw-"),
            "7" => rwx_repr.push_str("rwx"),
            _ => rwx_repr.push_str("???"),
        }
    }
    return rwx_repr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Values accessed from printing the modes in main().
    fn test_get_rwx_from_st_mode() {
        let result = get_rwx_from_st_mode(16877);
        assert_eq!("rwxr-xr-x", result);
        let result = get_rwx_from_st_mode(33188);
        assert_eq!("rw-r--r--", result);
    }

    #[test]
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
}
