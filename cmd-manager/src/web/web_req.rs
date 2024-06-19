use std::collections::HashMap;

pub fn main() {
    println!("Rust test request");
    get_price_ergo().expect("Failed to process request.");
}

#[tokio::main]
async fn get_price_ergo() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
