use pst::bins::Bin;
use pst::backends::{Backend, InvalidBackendError};
use reqwest::Client;

const TEST_STR: &'static str = "test string 123fd93f324";

#[test]
fn test_all_the_backends() {
    for (_, backend) in Backend::backend_iter() {
        assert!(test_backend(backend));
    }
}

#[test]
fn test_all_the_bins() -> Result<(), InvalidBackendError> {
    for (backend_name, _) in Backend::backend_iter() {
        assert!(test_bin(Bin::from_str(backend_name)?));
    };
    Ok(())
}

fn test_backend(backend: &Backend) -> bool {
    let paste = backend.post(TEST_STR).unwrap();

    println!("Got the API URL {:?} when testing {:?}", paste.api_url(), backend);

    let client = Client::new();

    let mut res = client.get(paste.api_url())
        .send()
        .expect("Couldn't post to bin");

    let res_text = res.text().unwrap();

    println!("{}", res_text);

    // Verify we can find the string we posted if we follow the link.
    res_text.find(TEST_STR).is_some()
}

fn test_bin(bin: Bin) -> bool {
    let paste = bin.post(TEST_STR).unwrap();

    println!("Got the API URL {:?} when testing {:?}", paste.api_url(), bin);

    let client = Client::new();

    let mut res = client.get(paste.api_url())
        .send()
        .expect("Couldn't post to bin");

    let res_text = res.text().unwrap();

    println!("{}", res_text);

    // Verify we can find the string we posted if we follow the link.
    res_text.find(TEST_STR).is_some()
}