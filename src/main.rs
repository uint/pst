use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pb", about = "Share code or text without leaving the command line.")]
struct Opt {
    /// File to process.
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();
    let content = match opt.file {
        Some(filename) => fs::read_to_string(filename)?,
        None => {
            let mut result = String::new();
            io::stdin().read_to_string(&mut result)?;
            result
        },
    };
    println!("{}", post_to_clbin(&content)?);
    Ok(())
}

fn post_to_clbin(body: &str) -> reqwest::Result<String> {
    let client = reqwest::Client::new();

    let params = [("clbin", body)];

    let mut res = client.post("https://clbin.com")
        .form(&params)
        .send()?;

    res.text()
}