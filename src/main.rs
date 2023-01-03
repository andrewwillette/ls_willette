use std::fs;

fn main() {
    print_current_files();
}

fn print_current_files() {
    for file in fs::read_dir(".").unwrap() {
        println!("{}", file.unwrap().file_name().to_str().unwrap());
    }
}

fn test_me() -> &'static str {
    return "swag";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = test_me();
        assert_eq!(result, "swag");
    }

    #[test]
    fn test_print_current_files() {
        print_current_files();
    }
}
