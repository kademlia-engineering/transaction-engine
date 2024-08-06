/*
handlers/src/lib.rs
6/30/24

This file defines the critical section of the codebase.
These handlers are used to orchestrate calls to lower level
crates and perform central logic.
*/
use models::{ Transactions, KnownCustomersArray, from_file };
use config::Config;
use database::{ DatabaseDriver, PostgresDriver };
use std::error::Error;

mod utils;

// This method creates a db connection, loads our input data,
// and uploads it to the database
pub async fn load_data(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut db_driver = PostgresDriver::new();
    match db_driver.connect(&config.db_connection_string).await {
        Ok(()) => {},
        Err(e) => return Err(e)
    }

    // Load Known Customers
    let known_customers = match from_file::<KnownCustomersArray>(&config.known_customers) {
        Ok(known_customers) => known_customers,
        Err(e) => return Err(e)
    };

    // Upload Known Customers to the db
    if let Err(e) = utils::insert_all_known_clients(&known_customers, &db_driver).await {
        return Err(e);
    }

    // Load Transaction Data
    let mut data = Transactions { transactions: Vec::new() };
    for file in &config.input_data {
        let mut input = match from_file::<Transactions>(&file) {
            Ok(input) => input,
            Err(e) => return Err(e)
        };
        data.transactions.append(&mut input.transactions);
    }

    // Upload Transactions to the db
    if let Err(e) = utils::insert_all_transactions(&data, &db_driver).await {
        return Err(e);
    }

    Ok(())
}

// This method creates a db connection, queries the transaction data
// for each known customer, and prints the result
pub async fn known_customer_deposits(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut db_driver = PostgresDriver::new();
    match db_driver.connect(&config.db_connection_string).await {
        Ok(()) => {},
        Err(e) => return Err(e)
    }

    // Load Known Customers
    let known_customers = match from_file::<KnownCustomersArray>(&config.known_customers) {
        Ok(known_customers) => known_customers,
        Err(e) => return Err(e)
    };

    // Iterate through Known Clients
    for customer in known_customers.known_customers {
        // Query Balance
        let balance = match db_driver.known_wallet_deposit_amount(&customer.address).await {
            Ok(Some(amount)) => amount,
            Ok(None) => return Err(Box::from("No Deposits Found")),
            Err(e) => return Err(e)
        };

        // Query Transactions
        let txn_count = match db_driver.known_wallet_transaction_count(&customer.address).await {
            Ok(amount) => amount,
            Err(e) => return Err(e)
        };

        // Log Output
        println!("Deposited for {0}: count={1} sum={2}", customer.name, txn_count, balance);
    }

    Ok(())
}

// This method creates a db connection, queries the transaction data
// for each unknown customer, and prints the result
pub async fn unknown_customer_deposits(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut db_driver = PostgresDriver::new();
    match db_driver.connect(&config.db_connection_string).await {
        Ok(()) => {},
        Err(e) => return Err(e)
    }

    // Query Balance
    let balance = match db_driver.unknown_wallet_deposit_amount().await {
        Ok(Some(amount)) => amount,
        Ok(None) => return Err(Box::from("No Deposits Found")),
        Err(e) => return Err(e)
    };

    // Query Transactions
    let txn_count = match db_driver.unknown_wallet_transaction_count().await {
        Ok(Some(amount)) => amount,
        Ok(None) => return Err(Box::from("No Transactions Found")),
        Err(e) => return Err(e)
    };

    // Log Output
    println!("Deposited without reference: count={0} sum={1}", txn_count, balance);

    Ok(())
}

// This method creates a db connection, queries the transaction data
// for the smallest and largest deposits
pub async fn calculate_range(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut db_driver = PostgresDriver::new();
    match db_driver.connect(&config.db_connection_string).await {
        Ok(()) => {},
        Err(e) => return Err(e)
    }

    // Query Min Balance
    let min = match db_driver.get_smallest_confirmed_amount().await {
        Ok(Some(amount)) => amount,
        Ok(None) => return Err(Box::from("No Min Found")),
        Err(e) => return Err(e)
    };

    // Log Min Deposit
    println!("Smallest valid deposit: {}", min);

    // Query Max Balance
    let max = match db_driver.get_max_confirmed_amount().await {
        Ok(Some(amount)) => amount,
        Ok(None) => return Err(Box::from("No Max Found")),
        Err(e) => return Err(e)
    };

    // Log Max Deposit
    println!("Largest valid deposit: {}", max);

    Ok(())
}
