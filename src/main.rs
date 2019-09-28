mod bins;

use std::fs;
use std::fmt;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

use bins::Bin;

#[derive(StructOpt)]
#[structopt(
    name = "pb",
    about = "Share code or text without leaving the command line.",
    setting = structopt::clap::AppSettings::ColoredHelp,
)]
struct Opt {
    /// File to process.
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// The pastebin implementation to use.
    #[structopt(short = "b", long = "bin", default_value = "termbin")]
    bin: String,
}

fn main() {
    if let Err(err) = run_pb() {
        #[cfg(debug)]
        eprintln!("Error: {:?}", err);

        #[cfg(not(debug))]
        eprintln!("Error: {}", err);

        std::process::exit(1);
    }
}

fn run_pb() -> std::result::Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();

    let bin = match &*opt.bin {
        "termbin" => Bin::Termbin,
        "clbin" => Bin::Clbin,
        _ => return Err(Box::new(InvalidPastebinError::new(opt.bin))),
    };

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

#[derive(Debug)]
struct InvalidPastebinError {
    bin_name: String,
}

impl InvalidPastebinError {
    fn new(bin_name: String) -> InvalidPastebinError {
        InvalidPastebinError {
            bin_name,
        }
    }
}

impl fmt::Display for InvalidPastebinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid pastebin `{}`", self.bin_name)
    }
}

impl std::error::Error for InvalidPastebinError {}