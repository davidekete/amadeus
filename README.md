# Amadeus API Client

This project is a Rust-based client for interacting with the Amadeus API. It provides functionality to perform GET, POST, and DELETE requests for managing flight-related data.

## Features

- **GET Request**: Retrieve flight destinations based on origin and maximum price.
- **POST Request**: Cancel a transfer order using an order ID and confirmation number.
- **DELETE Request**: Delete a flight order using its ID.

## Project Structure

- `src/api_client.rs`: Contains the implementation of the API client functions (`get_request`, `post_request`, `delete_request`).
- `src/model.rs`: Defines the data models for deserializing API responses.
- `src/main.rs`: Entry point of the application, demonstrating how to use the API client.

## Prerequisites

- Rust (latest stable version)
- [dotenv](https://crates.io/crates/dotenv) crate for managing environment variables
- An Amadeus API access token stored in a `.env` file as `ACCESS_TOKEN`.

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/amadeus-api-client.git
   cd amadeus-api-client
   ```
2. Create a .env file in the root directory and add your Amadeus API access token:
    ```
    ACCESS_TOKEN=your_access_token_here
    ```
3. Install dependencies:
    ```bash
    cargo build
    ```
