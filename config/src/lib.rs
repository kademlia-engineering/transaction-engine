/*
config/src/lib.rs
6/29/24

This file is used to parse the .env and make its content
accessble throughout the codebase.
*/
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

// Configuration structure
#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_connection_string: String,
    pub log_file: String,
    pub known_customers: String,
    pub input_data: Vec<String>,
}

impl Config {
    // Parse .env file into a Configuration struct
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let db_connection_string = env::var("DB_CONNECTION_STRING")?;
        let log_file = env::var("LOG_FILE")?;
        let known_customers = env::var("KNOWN_CUSTOMERS")?;

        let input_data_str = env::var("INPUT_DATA")?;
        let input_data: Vec<String> = serde_json::from_str(&input_data_str)?;

        Ok(Config {
            db_connection_string,
            log_file,
            known_customers,
            input_data,
        })
    }
}
