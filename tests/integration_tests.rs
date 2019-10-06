use pst::bins::Bin;
use reqwest::Client;

const TEST_STR: &'static str = "test string 123fd93f324";

#[test]
fn test_all_the_bins() {
    for (_, bin) in Bin::bin_iter() {
        assert!(test_bin(bin));
    }
}

fn test_bin(bin: &Bin) -> bool {
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
