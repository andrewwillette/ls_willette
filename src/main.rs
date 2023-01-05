use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filepath_arg = &args[1];
        let dir = Path::new(filepath_arg);
        if let Ok(files) = get_files_from_path(dir) {
            println!("{:?}", files);
        } else {
            println!("Failed to run in {} directory", dir.display())
        }
    } else {
        if let Ok(files) = get_files_from_path(Path::new(".")) {
            println!("{:?}", files);
        } else {
            println!("Failed to run in \".\" directory")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct LsFile {
    filename: String,
    privileges: String,
    last_edit_time: String,
    size: u64,
}

#[derive(Debug)]
pub enum WilletteLsError {
    ProvidedDirectoryInvalid,
    FilepathToStringConversion,
    DirEntryErr,
    FilePathErr,
}

fn get_files_from_path(filepath: &Path) -> Result<HashSet<LsFile>, WilletteLsError> {
    let mut files: HashSet<LsFile> = Default::default();
    if let Some(file_path_string) = filepath.to_str() {
        if let Ok(dir_contents) = fs::read_dir(file_path_string) {
            for dir_entry in dir_contents {
                if let Ok(filepath) = dir_entry {
                    if let Some(filename) = filepath.file_name().to_str() {
                        let lsfile: LsFile = LsFile {
                            filename: filename.to_string(),
                            privileges: "".to_string(),
                            last_edit_time: "".to_string(),
                            size: 0,
                        };
                        files.insert(lsfile);
                    } else {
                        return Err(WilletteLsError::FilepathToStringConversion);
                    };
                } else {
                    return Err(WilletteLsError::DirEntryErr);
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
                return Err(WilletteLsError::FilePathErr);
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
