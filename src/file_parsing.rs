use chrono::offset::Utc;
use chrono::{DateTime, TimeZone};
use chrono_tz::US::Central;
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tracing::Level;
use tracing::{debug, error};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;
use tracing_subscriber::fmt::format;

/// list directory contents
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'H', help = "Print size in human-readable format")]
    human_readable: bool,

    #[arg(short = 'l', help = "List files in the long format")]
    long: bool,

    #[arg(default_value = ".", help = "The directory to list contents of")]
    filepath: String,
}

pub fn run_ls() {
    let args = Cli::parse();
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

    match get_files_from_path(Path::new(&args.filepath)) {
        Ok(files) => {
            display_files(files, args);
        }
        Err(e) => println!("Error running get_files_from_path: {}", e),
    }
}

/// displays the files to the console
pub fn display_files(files: HashSet<LsFile>, cli: Cli) {
    if cli.long {
        for file in files {
            println!(
                "{}",
                format!(
                    "{} {} {} {}",
                    file.privileges, file.filename, file.size, file.last_edit_time
                )
            );
        }
    } else {
        for file in files {
            println!("{}", format!("{}", file.filename));
        }
    }
}

/// returns a HashSet of LsFile structs
pub fn get_files_from_path(filepath: &Path) -> Result<HashSet<LsFile>, Error> {
    let mut files: HashSet<LsFile> = Default::default();
    if let Some(file_path_string) = filepath.to_str() {
        if let Ok(dir_contents) = fs::read_dir(file_path_string) {
            for dir_entry in dir_contents {
                if let Ok(dir_entry_ok) = dir_entry {
                    let path = dir_entry_ok.path();
                    let ls_file = LsFile::new(path.to_str().unwrap().to_string());
                    files.insert(ls_file);
                } else {
                    error!("Error getting dir_entry_ok");
                    panic!("Error getting dir_entry_ok");
                };
            }
            return Ok(files);
        } else {
            debug!(
                "filepath provided to get_files_from_path failed read_dir call, probably a file"
            );
            if let Ok(_) = std::fs::File::open(filepath) {
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

            let filepath = Path::new(&filename);
            if let Some(filename) = filepath.file_name() {
                if let Some(filename_string) = filename.to_str() {
                    if let Ok(last_edit_time) = meta.modified() {
                        let datetime = DateTime::<Utc>::from(last_edit_time);
                        let tz_aware = Central.from_utc_datetime(&datetime.naive_utc());
                        return LsFile {
                            filename: filename_string.to_string(),
                            privileges: get_rwx_from_st_mode(mode),
                            last_edit_time: format!("{}", tz_aware.format("%d/%m/%Y %T")),
                            size: meta.len(),
                        };
                    } else {
                        panic!("Failed to get last_edit_time");
                    };
                } else {
                    panic!("failed to convert filename to string")
                }
            } else {
                panic!("failed to get filename from filepath")
            }
        } else {
            if let Ok(meta) = fs::symlink_metadata(&filename) {
                let permissions = meta.permissions();
                let mode = permissions.mode();
                let file_path = Path::new(&filename);
                if let Some(filename) = file_path.file_name() {
                    if let Some(filename_string) = filename.to_str() {
                        if let Ok(last_edit_time) = meta.modified() {
                            let datetime: DateTime<Utc> = last_edit_time.into();
                            let tz_aware = Central.from_utc_datetime(&datetime.naive_utc());
                            return LsFile {
                                filename: filename_string.to_string(),
                                privileges: get_rwx_from_st_mode(mode),
                                last_edit_time: format!("{}", tz_aware.format("%d/%m/%Y %T")),
                                size: meta.len(),
                            };
                        } else {
                            panic!("Failed to get last_edit_time from symlink");
                        };
                    } else {
                        panic!("failed to convert filename to string")
                    }
                } else {
                    panic!("failed on call file_path.file_name")
                }
            } else {
                panic!("failed on call fs::symlink_metadata")
            }
        }
    }
}
