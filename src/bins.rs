use std::io::prelude::*;
use std::net::TcpStream;

pub enum Bin {
    Clbin,
    Termbin,
}

impl Bin {
    pub fn post(&self, body: &str) -> std::result::Result<Paste, Box<dyn std::error::Error>> {
        use Bin::*;

        match self {
            Clbin => {
                let client = reqwest::Client::new();

                let params = [("clbin", body)];

                let mut res = client.post("https://clbin.com")
                    .form(&params)
                    .send()?;

                Ok(Paste(res.text()?.trim().to_string()))
            },
            Termbin => {
                let mut stream = TcpStream::connect("termbin.com:9999")?;

                stream.write_fmt(format_args!("{}", body))?;

                let mut res = String::new();
                stream.read_to_string(&mut res)?;
                
                Ok(Paste(res.trim_matches(char::from(0)).trim().to_string()))
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

#[cfg(test)]
mod tests {
    use crate::bins::Paste;

    #[test]
    fn paste_get_url() {
        let paste = Paste("https://fake-paste-bin.org/gjr8ge9rg8j".to_string());
        assert_eq!(paste.url(), "https://fake-paste-bin.org/gjr8ge9rg8j");
    }
}