// Sending HTTP GET requests asynchronously in Rust

use std::collections::HashMap;

// The runtime usage is defined on top of the main function of your application
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip") // will return the type Future
        .await? // wait for the future to be in a finished state before we move on
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(()) // Ok returns a result, an empty one in this case
}
