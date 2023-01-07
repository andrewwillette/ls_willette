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
                        privileges: format!("{mode:o}"),
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

#[cfg(test)]
mod tests {
    use super::*;
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
