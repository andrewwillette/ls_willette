mod file_parsing;
use clap::{Arg, Command};

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("", _)) => {
            println!("here");
        }
        _ => {
            println!("didn't get");
        }
    }
}

fn cli() -> Command {
    Command::new("ls_willette")
        .about("ls implemented in rust")
        .subcommand_required(false)
        .arg(
            Arg::new("includehidden")
                .short('a')
                .help("include hidden directories"),
        )
}
