use lazy_static::lazy_static;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fmt::{self, Debug};
use std::collections::{hash_map, HashMap};

lazy_static! {
    static ref BINS: HashMap<&'static str, Bin> = {
        let mut m = HashMap::new();
        m.insert("clbin", Bin::Clbin);
        m.insert("termbin", Bin::Termbin);
        m.insert("pastebin", Bin::Pastebin);
        m
    };
}

#[derive(Debug)]
pub enum Bin {
    Clbin,
    Termbin,
    Pastebin,
}

impl Bin {
    pub fn get_bin(name: &str) -> Result<&Bin, InvalidBinError> {
        BINS.get(name)
            .ok_or(InvalidBinError::new(String::from(name)))
    }

    pub fn bin_iter() -> hash_map::Iter<'static, &'static str, Bin> {
        BINS.iter()
    }

    pub fn post(&self, body: &str) -> std::result::Result<Paste, Box<dyn std::error::Error>> {
        use Bin::*;

        match self {
            Clbin => {
                let client = reqwest::Client::new();

                let params = [("clbin", body)];

                let mut res = client.post("https://clbin.com")
                    .form(&params)
                    .send()?;

                #[cfg(debug)]
                eprintln!("Status code received: {}", res.status());

                Ok(Paste(res.text()?.trim().to_string()))
            },
            Termbin => {
                let mut stream = TcpStream::connect("termbin.com:9999")?;

                stream.write_fmt(format_args!("{}", body))?;

                let mut res = String::new();
                stream.read_to_string(&mut res)?;
                
                Ok(Paste(res.trim_matches(char::from(0)).trim().to_string()))
            },
            Pastebin => {
                let client = reqwest::Client::new();

                let params = [
                    ("api_dev_key", "f44aba454f63e16cef1a46d58477481b"),
                    ("api_option", "paste"),
                    ("api_paste_code", body),
                ];

                let mut res = client.post("https://pastebin.com/api/api_post.php")
                    .form(&params)
                    .send()?;
                
                #[cfg(debug)]
                eprintln!("Status code received: {}", res.status());
                
                Ok(Paste(res.text()?.trim().to_string()))
            },
        }
    }
}

pub struct Paste(String);

impl Paste {
    pub fn url(&self) -> &str {
        let Paste(url) = self;
        &url
    }
}

#[derive(Debug)]
pub struct InvalidBinError {
    bin_name: String,
}

impl InvalidBinError {
    fn new(bin_name: String) -> InvalidBinError {
        InvalidBinError {
            bin_name,
        }
    }
}

impl fmt::Display for InvalidBinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid pastebin `{}`", self.bin_name)
    }
}

impl std::error::Error for InvalidBinError {}

#[cfg(test)]
mod tests {
    use crate::bins::Paste;

    #[test]
    fn paste_get_url() {
        let paste = Paste("https://fake-paste-bin.org/gjr8ge9rg8j".to_string());
        assert_eq!(paste.url(), "https://fake-paste-bin.org/gjr8ge9rg8j");
    }
}