use std::io::prelude::*;
use std::net::TcpStream;
use std::fmt::{self, Debug};
use std::error::Error;
use std::str::FromStr;

use serde::Deserialize;
use strum_macros::{EnumString, EnumIter, Display};
use strum::IntoEnumIterator;

#[derive(Debug, Display, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Backend {
    Clbin,
    Termbin,
    Pastebin,
    Hastebin,
}

impl Backend {
    pub fn get_backend(name: &str) -> Result<Backend, InvalidBackendError> {
        Backend::from_str(name).map_err(|_| InvalidBackendError::new(String::from(name)))
    }

    pub fn backends_iter() -> BackendIter {
        Backend::iter()
    }

    pub fn post(&self, body: &str, host: &str) -> Result<Paste, Box<dyn Error>> {
        use crate::backends::Backend::*;

        match self {
            Clbin => {
                let client = reqwest::Client::new();

                let params = [("clbin", body)];

                let mut res = client.post(host)
                                .form(&params)
                                .send()?;

                #[cfg(debug)]
                eprintln!("Status code received: {}", res.status());

                Ok(Paste::new(
                    res.text()?
                        .trim().
                        to_string(),
                    None,
                ))
            },
            Termbin => {
                let mut stream = TcpStream::connect(host)?;

                stream.write_fmt(format_args!("{}", body))?;

                let mut res = String::new();
                stream.read_to_string(&mut res)?;
                
                Ok(Paste::new(
                    res.trim_matches(char::from(0)).trim().to_string(),
                    None,
                ))
            },
            Pastebin => {
                let client = reqwest::Client::new();

                let params = [
                    ("api_dev_key", "f44aba454f63e16cef1a46d58477481b"),
                    ("api_option", "paste"),
                    ("api_paste_code", body),
                ];

                let mut res = client.post(host)
                    .form(&params)
                    .send()?;
                
                #[cfg(debug)]
                eprintln!("Status code received: {}", res.status());
                
                Ok(Paste::new(
                    res.text()?.trim().to_string(),
                    None,
                ))
            },
            Hastebin => {
                let client = reqwest::Client::new();

                let url = format!("{}documents/", host);

                let mut res = client.post(&url)
                                .body(body.to_string())
                                .send()?;

                #[cfg(debug)]
                eprintln!("Status code received: {}", res.status());

                #[derive(Deserialize)]
                struct Response {
                    key: String,
                }

                let res: Response = res.json()?;

                Ok(Paste::new(
                    format!("{}{}", host, res.key),
                    Some(format!("{}documents/{}", host, res.key)),
                ))
            },
        }
    }
}

#[derive(Debug)]
pub struct Paste {
    url: String,
    api_url: Option<String>,
}

impl Paste {
    pub fn new(url: String, api_url: Option<String>) -> Paste {
        Paste {
            url,
            api_url,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn api_url(&self) -> &str {
       match &self.api_url {
           Some(url) => url,
           None => &self.url,
       }
    }
}

#[derive(Debug)]
pub struct InvalidBackendError {
    backend_name: String,
}

impl InvalidBackendError {
    fn new(backend_name: String) -> InvalidBackendError {
        InvalidBackendError {
            backend_name,
        }
    }
}

impl fmt::Display for InvalidBackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid backend `{}`", self.backend_name)
    }
}

impl std::error::Error for InvalidBackendError {}

#[cfg(test)]
mod tests {
    use crate::backends::Paste;

    #[test]
    fn paste_get_url() {
        let paste = Paste::new(
            "https://fake-paste-bin.org/gjr8ge9rg8j".to_string(),
            None,
        );
        assert_eq!(paste.url(), "https://fake-paste-bin.org/gjr8ge9rg8j");
    }
}