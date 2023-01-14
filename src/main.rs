mod file_parsing;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    // let matches = cli().get_matches();
    // match matches.subcommand() {
    //     Some(("", _)) => {
    //         println!("here");
    //     }
    //     _ => {
    //         println!("didn't get");
    //     }
    // }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'f')]
    eff: bool,

    #[arg(short = 'p', value_name = "PEAR")]
    pea: Option<String>,

    #[arg(last = true)]
    slop: Vec<String>,
}

// fn cli() -> Command {
//     Command::new("ls_willette")
//         .about("ls implemented in rust")
//         .subcommand_required(false)
//         .arg(
//             Arg::new("includehidden")
//                 .short('a')
//                 .help("include hidden directories"),
//         )
// }
