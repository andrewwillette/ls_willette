use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let files: HashSet<LsFile>;
    if args.len() > 1 {
        let filepath_arg = &args[1];
        if filepath_arg != "" {
            files = get_directory_files(Path::new(filepath_arg));
        } else {
            files = get_directory_files(Path::new("."));
        }
    } else {
        files = get_directory_files(Path::new("."));
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

fn get_directory_files(filepath: &Path) -> HashSet<LsFile> {
    let mut files: HashSet<LsFile> = Default::default();
    let readdir: fs::ReadDir = match fs::read_dir(filepath.to_str().unwrap()) {
        Ok(readdir) => readdir,
        Err(error) => panic!("Invalid directory provided. {}", error),
    };
    for file in readdir {
        let filepath: String = file.unwrap().file_name().to_str().unwrap().to_string();
        let lsfile: LsFile = LsFile {
            filename: filepath,
            privileges: "".to_string(),
            last_edit_time: "".to_string(),
            size: 0,
        };
        files.insert(lsfile);
    }
    return files;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_directory_files() {
        let set = HashSet::from([
            "Cargo.toml",
            "src",
            ".git",
            ".gitignore",
            "target",
            "Cargo.lock",
        ]);
        let result = get_directory_files(Path::new("."));
        assert_eq!(result.len(), 6);
        for filename in set {
            assert_eq!(
                true,
                result.iter().any(|lsfile| lsfile.filename == filename)
            );
        }
    }
}
