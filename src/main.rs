use pst::bins::Bin;

use std::fs;
use std::io::{self, Read};
use clap::{Arg, App, AppSettings};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    let bins = Bin::bin_iter()
        .map(|x| {
            *x.0
        })
        .collect::<Vec<&str>>()
        .join(", ");

    let pastebin_help = format!(
        "The pastebin implementation to use.\n\
        Available options: {}",
        bins
    );

    let opts = App::new("pst")
                    .setting(AppSettings::ColoredHelp)
                    .version(VERSION)
                    .author("Tomasz Kurcz <uint@lavabit.com>")
                    .about("Share code or text without leaving the command line.")
                    .arg(Arg::with_name("FILE")
                        .help("File to send.")
                        .required(false)
                        .index(1))
                    .arg(Arg::with_name("bin")
                        .short("b")
                        .long("bin")
                        .default_value("termbin")
                        .help(&pastebin_help))
                    .get_matches();

    let bin_name = opts.value_of("bin").unwrap();
    let bin = Bin::get_bin(bin_name)?;

    let content = match opts.value_of("FILE") {
        Some(filename) => fs::read_to_string(filename)?,
        None => {
            let mut result = String::new();
            io::stdin().read_to_string(&mut result)?;
            result
        },
    };

    let paste = bin.post(&content)?;

    #[cfg(debug)]
    println!("Debug representation of the Paste:\n{:?}", paste);

    println!("{}", paste.url());

    Ok(())
}
