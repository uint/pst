use std::error::Error;

use pst::bins::Bin;
use pst::config;

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

fn run_app() -> Result<(), Box<dyn Error>> {
    let pst = config::pst_from_cfg()?;
    let default_bin = pst.default_bin_name();

    let bins = pst.bin_names()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let bin_help = format!(
        "The bin to use.\n\
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
                        .default_value(default_bin)
                        .help(&bin_help))
                    .arg(Arg::with_name("write-config")
                        .short("w")
                        .long("write-config")
                        .help("Write the default config to disk."))
                    .get_matches();

    if opts.is_present("write-config") {
        return config::write_default_cfg()
    }

    let bin_name = opts.value_of("bin").unwrap();
    let bin = pst.bin(bin_name)?;

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
