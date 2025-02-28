use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FlightResponse {
    pub data: Vec<FlightDestination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlightDestination {
    #[serde(rename = "type")]
    pub flight_type: String, // "type" is a reserved word in Rust, so we rename it

    pub origin: String,
    pub destination: String,
    pub departureDate: String,
    pub returnDate: String,

    pub price: FlightPrice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlightPrice {
    pub total: String,
}
