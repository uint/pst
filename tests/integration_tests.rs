use pst::bins::Bin;
use pst::config::ConfigStore;
use pst::backends::Backend;
use reqwest::Client;

const TEST_STR: &'static str = "test string 123fd93f324";

type BoxError = Box<dyn std::error::Error>;

#[test] #[ignore] // needs a better approach
fn test_all_the_bins() -> Result<(), BoxError> {
    let cfg_store = ConfigStore::new()?;

    for backend in Backend::backends_iter() {
        let backend_name = backend.to_string();
        let cfg = cfg_store.bin_config(&backend_name).expect("");
        assert!(test_bin(Bin::new(
            backend,
            &cfg,
        )));
    };
    Ok(())
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