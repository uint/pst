fn main() {
    println!("{}", post_to_clbin("Hello from Rust!").expect("Failed to post to clbin!"));
}

fn post_to_clbin(body: &str) -> reqwest::Result<String> {
    let client = reqwest::Client::new();

    let form = reqwest::multipart::Form::new()
        .file("clbin", "main.rs")
        .expect("Bleh.");
    let params = [("clbin", body)];
    
    let mut res = client.post("https://clbin.com")
        .form(&params)
        .send()?;

    res.text()
}