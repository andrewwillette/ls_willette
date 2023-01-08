use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tracing::{trace, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;
use tracing_subscriber::fmt::format;

pub fn run_ls() {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "/Users/andrewwillette/git/ls_willette/target/logs",
        "ls_willette.log",
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::TRACE)
        .event_format(format().pretty())
        .init();

    let args: Vec<String> = env::args().collect();
    let dir: &Path;
    if args.len() > 1 {
        dir = Path::new(&args[1]);
    } else {
        dir = Path::new(".");
    }
    match get_files_from_path(dir) {
        Ok(files) => {
            display_files(files);
        }
        Err(e) => println!("Error running get_files_from_path: {}", e),
    }
}

/// displays the files to the console
pub fn display_files(files: HashSet<LsFile>) {
    for file in files {
        println!("{}", format!("{} {}", file.privileges, file.filename));
    }
}

/// returns a HashSet of LsFile structs
pub fn get_files_from_path(filepath: &Path) -> Result<HashSet<LsFile>, Error> {
    // let filepath_str = filepath.to_str().unwrap();
    let mut files: HashSet<LsFile> = Default::default();
    if let Some(file_path_string) = filepath.to_str() {
        if let Ok(dir_contents) = fs::read_dir(file_path_string) {
            for dir_entry in dir_contents {
                let filepath = dir_entry?.path();
                let symlink_followed = fs::canonicalize(&filepath)?;
                trace!("symlink_followed: {}", symlink_followed.display());
                // debug!("symlink_followed: {:?}", symsdf:qa
                // link_followed);
                if let Some(filename) = symlink_followed.to_str() {
                    let ls_file = LsFile::new(filename.to_string());
                    files.insert(ls_file);
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Failed to convert filepath to string.",
                    ));
                };
            }
            return Ok(files);
        } else {
            if let Ok(file) = std::fs::File::open(filepath) {
                let ls_file = LsFile::new(filepath.to_str().unwrap().to_string());
                files.insert(ls_file);
            } else {
                return Err(Error::new(ErrorKind::Other, "Failed to open filepath."));
            }
        }
    }
    return Ok(files);
}

/// Convert the base10 OS st_mode to rwx format.
/// Declaring the parameter base_10 is somewhat redundant
/// given it is of type u32.
pub fn get_rwx_from_st_mode(base_10_st_mode: u32) -> String {
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

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct LsFile {
    pub filename: String,
    pub privileges: String,
    pub last_edit_time: String,
    pub size: u64,
}

impl LsFile {
    fn new(filename: String) -> LsFile {
        if let Ok(meta) = fs::metadata(&filename) {
            let permissions = meta.permissions();
            let mode = permissions.mode();
            let lsfile: LsFile = LsFile {
                filename: filename.to_string(),
                privileges: get_rwx_from_st_mode(mode),
                last_edit_time: "".to_string(),
                size: 0,
            };
            return lsfile;
        } else {
            todo!();
        }
    }
}
