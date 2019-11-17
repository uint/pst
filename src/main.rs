use std::error::Error;

use pst::bins::Bin;
use pst::backends::Backend;
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
    let cfg = config::pst_config()?;
    let default_bin = cfg.bin();

    let backends = Backend::backend_iter()
        .map(|x| {
            *x.0
        })
        .collect::<Vec<&str>>()
        .join(", ");

    let backend_help = format!(
        "The backend to use.\n\
        Available options: {}",
        backends
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
                    .arg(Arg::with_name("backend")
                        .short("b")
                        .long("backend")
                        .default_value(default_bin)
                        .help(&backend_help))
                    .arg(Arg::with_name("write-config")
                        .short("w")
                        .long("write-config")
                        .help("Write the default config to disk."))
                    .get_matches();

    if opts.is_present("write-config") {
        return config::write_default_cfg()
    }

    let backend_name = opts.value_of("backend").unwrap();
    let cfg = config::bin_config(backend_name)?;
    let bin = Bin::from_str(
        backend_name,
        &cfg,
    )?;

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
