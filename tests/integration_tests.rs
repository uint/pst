// This function can be used to test pretty much any backend. I don't want to
// test on someone else's production servers, though. Maybe in the future
// we'll use this to test against mocks or test instances of pastebin servers.

// use pst::bins::Bin;
// use reqwest::Client;

// const TEST_STR: &'static str = "test string 123fd93f324";

// type BoxError = Box<dyn std::error::Error>;

// fn test_bin<B: Bin>(bin: B) -> bool {
//     let paste = bin.post(TEST_STR).unwrap();

//     println!("Got the API URL {:?} when testing {:?}", paste.api_url(), bin);

//     let client = Client::new();

//     let mut res = client.get(paste.api_url())
//         .send()
//         .expect("Couldn't post to bin");

//     let res_text = res.text().unwrap();

//     println!("{}", res_text);

//     // Verify we can find the string we posted if we follow the link.
//     res_text.find(TEST_STR).is_some()
// }