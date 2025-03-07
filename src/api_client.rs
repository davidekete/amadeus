use crate::model::{CancellationResponse, FlightResponse};
use dotenv::var;
use once_cell::sync::Lazy;
use reqwest::{header, Client, Error};
use std::collections::HashMap;

const BASE_URL: &str = "https://test.api.amadeus.com/v1/";

static ACCESS_TOKEN: Lazy<String> =
    Lazy::new(|| var("ACCESS_TOKEN").unwrap_or_else(|_| "value".to_string()));

pub async fn get_request(origin: &str, max_price: &str) -> Result<FlightResponse, Error> {
    // Construct the API endpoint URL
    let url = format!("{}/shopping/flight-destinations", BASE_URL);

    // Create query parameters for the request
    let mut query_params = HashMap::new();
    query_params.insert("origin", &origin); // Set the departure location
    query_params.insert("maxPrice", &max_price); // Set the maximum price limit

    // Create a new HTTP client
    let client = Client::new();

    // Send a GET request with the Authorization header and query parameters
    let response = client
        .get(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN)) // Attach the access token
        .query(&query_params) // Add query parameters to the request
        .send()
        .await?; // Await the response asynchronously

    // Handle potential request errors
    if !response.status().is_success() {
        eprintln!("{:?} {}", response, "An error occurred"); // Log an error message if the request fails
    }

    // Deserialize the JSON response into a FlightResponse struct
    let flight_response: FlightResponse = response.json().await?;

    // Return the parsed response
    Ok(flight_response)
}

pub async fn post_request(
    order_id: String,
    confirm_nbr: String,
) -> Result<CancellationResponse, Error> {
    // Construct the API endpoint URL dynamically using the order_id
    let url = format!(
        "{}/ordering/transfer-orders/{}/transfers/cancellation",
        BASE_URL, order_id
    );

    let mut query_params = HashMap::new();
    query_params.insert("confirmNbr", confirm_nbr);

    let client = Client::new();

    let response = client
        .post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
        .query(&query_params)
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("{:?} {}", response, "An error occurred")
    }

    let data: CancellationResponse = response.json().await?;
    Ok(data)
}

pub async fn delete_request(flight_order_id: String) {
    let url = format!("{}/booking/flight-orders/{}", BASE_URL, flight_order_id);

    let client = Client::new();

    let response = client
        .delete(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
        .send()
        .await;

    match response {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            eprintln!("{:?} {}", err, "An error occurred")
        }
    }
}

// eJzTd9f3s4gKC%2FMEAAt8Ans%3D
