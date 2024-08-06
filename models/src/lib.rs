/*
models/src/lib.rs
6/29/24

This file defines common data structures used in the codebase.
Additionally, there is a generic parser for
loading a json file into a custom struct
*/
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

// Transaction structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "involvesWatchonly")]
    pub involves_watchonly: bool,
    pub account: String,
    pub address: String,
    pub category: String,
    pub amount: f64,
    pub label: String,
    pub confirmations: i32,
    pub blockhash: String,
    pub blockindex: i32,
    pub blocktime: i64,
    pub txid: String,
    pub vout: i32,
    pub walletconflicts: Vec<String>,
    pub time: i64,
    pub timereceived: i64,
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
}

// Vector of Transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

// Known Customer structure
#[derive(Serialize, Deserialize, Debug)]
pub struct KnownCustomers {
    pub name: String,
    pub address: String,
}

// Vector of Known Customers
#[derive(Serialize, Deserialize, Debug)]
pub struct KnownCustomersArray {
    pub known_customers: Vec<KnownCustomers>,
}

// Generic method for parsing a json file into a custom struct
pub fn from_file<T: DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn Error>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON into T
    let data = serde_json::from_reader(reader)?;

    Ok(data)
}
