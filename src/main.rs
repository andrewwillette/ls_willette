use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let files: HashSet<LsFile>;
    if args.len() > 1 {
        let filepath_arg = &args[1];
        // TODO: handle if provided argument is a file / directory
        files = match get_directory_files(Path::new(filepath_arg)) {
            Ok(files) => files,
            Err(_) => panic!("Error getting directory files"),
        };
    } else {
        files = match get_directory_files(Path::new(".")) {
            Ok(files) => files,
            Err(_) => panic!("Error getting directory files"),
        };
    }
    println!("{:?}", files);
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
}

fn get_directory_files(filepath: &Path) -> Result<HashSet<LsFile>, WilletteLsError> {
    let mut files: HashSet<LsFile> = Default::default();
    let readdir: fs::ReadDir = match fs::read_dir(filepath.to_str().unwrap()) {
        Ok(readdir) => readdir,
        Err(error) => panic!("Invalid directory provided. {}", error),
    };
    for file in readdir {
        let filepath: String = match file {
            Ok(file) => match file.file_name().to_str() {
                None => panic!("Invalid file name."),
                Some(filename) => filename.to_string(),
            },
            Err(err) => panic!("Invalid file provided. {}", err),
        };
        let lsfile: LsFile = LsFile {
            filename: filepath,
            privileges: "".to_string(),
            last_edit_time: "".to_string(),
            size: 0,
        };
        files.insert(lsfile);
    }
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
        let result = match get_directory_files(Path::new(".")) {
            Ok(files) => files,
            Err(_) => panic!("Error getting directory files"),
        };
        assert_eq!(result.len(), 6);
        for filename in expected_filenames {
            assert_eq!(
                true,
                result.iter().any(|lsfile| lsfile.filename == filename)
            );
        }
    }
}
