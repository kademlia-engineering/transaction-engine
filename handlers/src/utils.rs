/*
handlers/src/utils.rs
7/2/24
*/
use models::{ Transactions, KnownCustomersArray };
use database::DatabaseDriver;
use std::error::Error;

// Delegate call to upload known clients
pub async fn insert_all_known_clients<D: DatabaseDriver>(known_customers: &KnownCustomersArray, db_driver: &D)
-> Result<(), Box<dyn Error>> {
    for customer in &known_customers.known_customers {
        db_driver.insert_known_client(&customer).await?;
    }
    Ok(())
}

// Delegate call to upload transactions
pub async fn insert_all_transactions<D: DatabaseDriver>(transactions: &Transactions, db_driver: &D)
-> Result<(), Box<dyn Error>> {
    for transaction in &transactions.transactions {
        db_driver.insert_transaction(&transaction).await?;
    }
    Ok(())
}
