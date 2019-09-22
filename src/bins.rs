pub enum Bin {
    Clbin,
}

impl Bin {
    pub fn post(&self, body: &str) -> std::result::Result<Paste, Box<dyn std::error::Error>> {
        match self {
            Bin::Clbin => {
                let client = reqwest::Client::new();

                let params = [("clbin", body)];

                let mut res = client.post("https://clbin.com")
                    .form(&params)
                    .send()?;

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

#[cfg(test)]
mod tests {
    use crate::bins::Paste;

    #[test]
    fn paste_get_url() {
        let paste = Paste("https://fake-paste-bin.org/gjr8ge9rg8j".to_string());
        assert_eq!(paste.url(), "https://fake-paste-bin.org/gjr8ge9rg8j");
    }
}