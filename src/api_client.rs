use crate::model::FlightResponse;
use dotenv::var;
use once_cell::sync::Lazy;
use reqwest::{header, Client, Error};
use serde::de::value;
use std::collections::HashMap;

const BASE_URL: &str = "https://test.api.amadeus.com/v1/";

static ACCESS_TOKEN: Lazy<String> =
    Lazy::new(|| var("ACCESS_TOKEN").unwrap_or_else(|_| "value".to_string()));

pub async fn get_request() -> Result<FlightResponse, Error> {
    let url = format!("{}/shopping/flight-destinations", BASE_URL);

    let mut query_params = HashMap::new();
    query_params.insert("origin", "PAR");
    query_params.insert("maxPrice", "200");

    let client = Client::new();

    let response = client
        .get(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", &*ACCESS_TOKEN))
        .query(&query_params)
        .send()
        .await?;

    //Handle possible error
    if !response.status().is_success() {
        eprintln!("{:?} {}", response, "An error occurred")
    }

    let flight_response: FlightResponse = response.json().await?;
    Ok(flight_response)
}

pub async fn post_request(order_id: String, confirmNbr: String) {
    let url = format!(
        "{}/ordering/transfer-orders/{}/transfers/cancellation",
        BASE_URL, order_id
    );

    let mut query_params = HashMap::new();
    query_params.insert("confirmNbr", confirmNbr);

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
}

pub async fn put_request() {}
