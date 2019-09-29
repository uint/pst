use pst::bins::Bin;
use reqwest::Client;

const TEST_STR: &'static str = "test string 123\n324";

#[test]
fn clbin() {
    assert!(test_bin(Bin::Clbin));
}

#[test]
fn termbin() {
    assert!(test_bin(Bin::Termbin));
}

#[test]
fn pastebin() {
    assert!(test_bin(Bin::Pastebin));
}

fn test_bin(bin: Bin) -> bool {
    let paste = bin.post(TEST_STR).unwrap();

    println!("Got the URL {:?} when testing {:?}", paste.url(), bin);

    let client = Client::new();

    let mut res = client.get(paste.url())
        .send()
        .expect("Couldn't post to bin");

    // Verify we can find the string we posted if we follow the link.
    res.text().unwrap().find(TEST_STR).is_some()
}
