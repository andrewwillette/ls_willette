use std::fs;
use std::path::Path;

fn main() {
    print_current_files();
}

fn print_current_files() {
    for file in fs::read_dir(".").unwrap() {
        println!("{}", file.unwrap().file_name().to_str().unwrap());
    }
}

fn get_directory_files(filepath: &Path) -> &'static str {
    for file in fs::read_dir(filepath.to_str().unwrap()).unwrap() {
        // println!("{}", file.unwrap().file_name().to_str().unwrap());
        let filepath: String = file.unwrap().file_name().to_str().unwrap().to_string();
        return &filepath.clone();
    }
    return &"hi";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = "swag";
        assert_eq!(result, "swag");
    }

    #[test]
    fn test_print_current_files() {
        print_current_files();
    }
}
