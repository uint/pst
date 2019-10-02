use pst::bins::Bin;

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "pst",
    about = "Share code or text without leaving the command line.",
    setting = structopt::clap::AppSettings::ColoredHelp,
)]
struct Opt {
    /// File to process.
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// The pastebin implementation to use.
    /// Available options: termbin, pastebin, clbin.
    #[structopt(short = "b", long = "bin", default_value = "termbin")]
    bin: String,
}

fn main() {
    if let Err(err) = run_app() {
        #[cfg(debug)]
        eprintln!("Error: {:?}", err);

        #[cfg(not(debug))]
        eprintln!("Error: {}", err);

        std::process::exit(1);
    }
}

fn run_app() -> std::result::Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();

    let bin = Bin::get_bin(&*opt.bin)?;

    let content = match opt.file {
        Some(filename) => fs::read_to_string(filename)?,
        None => {
            let mut result = String::new();
            io::stdin().read_to_string(&mut result)?;
            result
        },
    };

    let paste = bin.post(&content)?;

    #[cfg(debug)]
    println!("Debug representation of the URL: {:?}", paste.url());

    println!("{}", paste.url());

    Ok(())
}
