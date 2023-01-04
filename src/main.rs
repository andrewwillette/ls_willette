use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath_arg = &args[1];
    println!("filepath_arg: {}", filepath_arg);
    let p = Path::new(filepath_arg);
    let pb = p.to_str().unwrap();
    println!("Path: {}", pb);
    if filepath_arg != "" {
        get_directory_files(Path::new(filepath_arg));
    } else {
        get_directory_files(Path::new("."));
    }
}

fn get_directory_files(filepath: &Path) -> HashSet<String> {
    let mut files: HashSet<String> = Default::default();
    for file in fs::read_dir(filepath.to_str().unwrap()).unwrap() {
        let filepath: String = file.unwrap().file_name().to_str().unwrap().to_string();
        files.insert(filepath.clone());
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
            assert_eq!(true, result.contains(filename));
        }
    }
}
