use std::collections::HashSet;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use chrono::offset::Utc;
use chrono::{DateTime, TimeZone};
use chrono_tz::US::Central;
use clap::Parser;
use terminal_size::{terminal_size, Width};
use tracing::Level;
use tracing::{debug, error};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;
use tracing_subscriber::fmt::format;

fn main() {
    run_ls();
}

/// list directory contents
#[derive(Parser)]
#[command(author = "Andrew Willette", version, about, long_about = None)]
struct Cli {
    #[arg(short = 'H', help = "Print size in human-readable format")]
    human_readable: bool,

    #[arg(short = 'l', help = "List files in the long format")]
    long: bool,

    #[arg(default_value = ".", help = "The directory to list contents of")]
    filepath: String,
}

fn run_ls() {
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
fn display_files(files: Vec<LsFile>, cli: Cli) {
    if cli.long {
        for file in files {
            println!(
                "{}",
                format!(
                    "{} {} {} {}",
                    file.privileges.unwrap(),
                    file.filename,
                    file.size.unwrap(),
                    file.last_edit_time.unwrap()
                )
            );
        }
    } else {
        display_default(files);
    }
}

fn display_default(files: Vec<LsFile>) -> HashSet<String> {
    let mut console_out_rows: HashSet<String> = HashSet::new();
    let mut row = String::from("");
    let size = terminal_size();
    let padded_length = get_padding_length(&files);
    if let Some((Width(w), _)) = size {
        files.into_iter().for_each(|file| {
            let current_row_length = row.chars().count() as u16;
            let new_word_w_padding =
                get_filename_display_with_padding(file.filename, padded_length);
            let current_line_plus_prospective_new =
                current_row_length + (new_word_w_padding.chars().count() as u16);
            // append to current line
            if current_line_plus_prospective_new <= w {
                row = format!("{}{}", row, new_word_w_padding);
            // write current line to output and begin next line
            } else {
                console_out_rows.insert(row.to_string());
                row = new_word_w_padding;
            }
        });
        if !console_out_rows.contains(&row) {
            console_out_rows.insert(row.to_string());
        }
        for line in console_out_rows.iter() {
            println!("{}", line);
        }
    } else {
        panic!("failed to get terminal width");
    }
    return console_out_rows;
}

/// returns appropriate space padding for output to console
/// by getting the longest filename and multiplying by 2
fn get_padding_length(files: &Vec<LsFile>) -> i8 {
    let mut longest_filename = 0;
    files.into_iter().for_each(|file| {
        let filename_length = file.filename.chars().count();
        if filename_length > longest_filename {
            longest_filename = filename_length;
        }
    });
    let padding_length = longest_filename + 4;
    return padding_length as i8;
}

/// returns the filename with spaces appended to reach the padded_length
fn get_filename_display_with_padding(filename: String, padded_length: i8) -> String {
    let mut result = filename;
    let mut filename_length = result.chars().count();
    let padded_length = padded_length as usize;
    while filename_length < padded_length {
        let updated_result = format!("{} ", result).to_string();
        result = updated_result;
        filename_length = result.chars().count();
    }
    return result;
}

/// returns a HashSet of LsFile structs
fn get_files_from_path(filepath: &Path) -> Result<Vec<LsFile>, Error> {
    let mut files: Vec<LsFile> = Vec::new();
    if let Some(file_path_string) = filepath.to_str() {
        if let Ok(dir_contents) = fs::read_dir(file_path_string) {
            for dir_entry in dir_contents {
                if let Ok(dir_entry_ok) = dir_entry {
                    let path = dir_entry_ok.path();
                    let ls_file = LsFile::new(path.to_str().unwrap().to_string());
                    files.push(ls_file);
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
                files.push(ls_file);
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

#[derive(PartialEq, Eq, Hash, Debug)]
struct LsFile {
    filename: String,
    privileges: Option<String>,
    last_edit_time: Option<String>,
    size: Option<u64>,
}

impl LsFile {
    fn new(filename: String) -> Self {
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
                            privileges: Some(get_rwx_from_st_mode(mode)),
                            last_edit_time: Some(format!("{}", tz_aware.format("%d/%m/%Y %T"))),
                            size: Some(meta.len()),
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
                                privileges: Some(get_rwx_from_st_mode(mode)),
                                last_edit_time: Some(format!("{}", tz_aware.format("%d/%m/%Y %T"))),
                                size: Some(meta.len()),
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

/// TESTS

#[test]
fn test_get_rwx_from_st_mode() {
    let result = get_rwx_from_st_mode(16877);
    assert_eq!("rwxr-xr-x", result);
    let result = get_rwx_from_st_mode(33188);
    assert_eq!("rw-r--r--", result);
}

#[test]
#[ignore = "Fails when contents of directory running in changes."]
fn test_get_directory_files() {
    // let expected_filenames: Vec<&str> = vec![
    //     "Cargo.toml",
    //     "src",
    //     ".git",
    //     ".gitignore",
    //     "target",
    //     "Cargo.lock",
    // ];
    let ls_files = match get_files_from_path(Path::new(".")) {
        Ok(files) => files,
        Err(_) => panic!("Error getting directory files"),
    };
    assert_eq!(ls_files.len(), 7);
    // for filename in expected_filenames {
    //     assert_eq!(
    //         true,
    //         ls_files
    //             .iter()
    //             .any(|lsfile| lsfile.filename.unwrap() == filename.to_string())
    //     );
    // }
}

#[test]
fn test_get_filename_display_with_padding() {
    let input = String::from("word");
    let input_padded = get_filename_display_with_padding(input, 10);
    assert_eq!("word      ", input_padded);

    // this behavior might not be desirable
    let input = String::from("word");
    let input_padded = get_filename_display_with_padding(input, 2);
    assert_eq!("word", input_padded);
}

#[test]
fn test_display_default() {
    let input: Vec<LsFile> = vec![
        LsFile {
            last_edit_time: Some(String::from("01/01/2020 00:00:00")),
            filename: String::from("Cargo.toml"),
            privileges: None,
            size: None,
        },
        LsFile {
            last_edit_time: Some(String::from("01/01/2020 00:00:00")),
            filename: String::from("example.toml"),
            privileges: None,
            size: None,
        },
        // "src",
        // ".git",
        // ".gitignore",
        // "target",
        // "Cargo.lock",
    ];
    _ = display_default(input)
}
