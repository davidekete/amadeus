use api_client::{delete_request, get_request, post_request};
use std::error::Error;
use tokio;
mod api_client;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Calling GET request
    match get_request("PAR", "200").await {
        Ok(response) => println!("{:?}", response),
        Err(err) => eprintln!("GET request failed: {:?}", err),
    }

    // Calling POST request
    let order_id = "12345".to_string();
    let confirm_nbr = "CONFIRM123".to_string();
    match post_request(order_id, confirm_nbr).await {
        Ok(response) => println!("{:?}", response),
        Err(err) => eprintln!("POST request failed: {:?}", err),
    }

    // Calling DELETE request
    let flight_order_id = "eJzTd9f3s4gKC%2FMEAAt8Ans%3D".to_string();
    delete_request(flight_order_id).await;

    Ok(())
}
