use api_client::get_request;
use dotenv::var;
use reqwest::Client;
use serde_json::json;
use serde_urlencoded;
use std::error::Error;
use tokio;
mod api_client;
mod model;
// mod error_handler;

#[tokio::main]
async fn get_access_token() -> Result<(), Box<dyn Error>> {
    let api_key = var("API_KEY")?;
    let api_secret = var("API_SECRET")?;

    let url = "https://test.api.amadeus.com/v1/security/oauth2/token";

    let mut data = serde_urlencoded::to_string([
        ("grant_type", "client_credentials"),
        ("client_id", &api_key),
        ("client_secret", &api_secret),
    ])?;

    let client = Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // match get_access_token() {
    //     Ok(t) => t,
    //     Err(e) => eprintln!("{}", e),
    // }
    match get_request().await {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())

}
