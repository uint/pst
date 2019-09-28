mod bins;

use std::fs;
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

    let content = match opt.file {
        Some(filename) => fs::read_to_string(filename)?,
        None => {
            let mut result = String::new();
            io::stdin().read_to_string(&mut result)?;
            result
        },
    };

    let bin = Bin::Termbin;
    let paste = bin.post(&content)?;

    #[cfg(debug)]
    println!("Debug representation of the URL: {:?}", paste.url());

    println!("{}", paste.url());

    Ok(())
}
