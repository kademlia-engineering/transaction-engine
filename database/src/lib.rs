/*
database/src/lib.rs
6/29/24

This file defines the database interface and provides
an implementation of a postgres client
*/
use tokio_postgres::{Client, NoTls, Row};
use async_trait::async_trait;
use models::{KnownCustomers, Transaction};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::error::Error;

// This trait defines the programmatic interface with the database
#[async_trait]
pub trait DatabaseDriver {
    fn new() -> Self where Self: Sized;
    async fn connect(&mut self, connection_str: &str) -> Result<(), Box<dyn Error>>;
    async fn known_wallet_deposit_amount(&self, address: &str) -> Result<Option<f64>, Box<dyn Error>>;
    async fn known_wallet_transaction_count(&self, address: &str) -> Result<i32, Box<dyn Error>>;
    async fn unknown_wallet_deposit_amount(&self) -> Result<Option<f64>, Box<dyn Error>>;
    async fn unknown_wallet_transaction_count(&self) -> Result<Option<i32>, Box<dyn Error>>;
    async fn get_smallest_confirmed_amount(&self) -> Result<Option<f64>, Box<dyn Error>>;
    async fn get_max_confirmed_amount(&self) -> Result<Option<f64>, Box<dyn Error>>;
    async fn insert_known_client(&self, known_customer: &KnownCustomers) -> Result<(), Box<dyn Error>>;
    async fn insert_transaction(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>>;
    fn close(&mut self);
}

// This struct defines the Postgres client
pub struct PostgresDriver {
    client: Option<Client>,
}

// Postgres implementation of the Database Driver trait
#[async_trait]
impl DatabaseDriver for PostgresDriver {
    // Client constructor
    fn new() -> Self {
        Self { client: None }
    }

    // Create a database connection for the Client
    async fn connect(&mut self, connection_str: &str) -> Result<(), Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(connection_str, NoTls).await?;

        // Spawn the connection in a new child process
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        self.client = Some(client);
        Ok(())
    }

    // Execute get_total_confirmed_amount stored procedure
    async fn known_wallet_deposit_amount(&self, address: &str) -> Result<Option<f64>, Box<dyn Error>> {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&address];
        let procedure = "SELECT get_total_confirmed_amount($1)";
        if let Some(client) = &self.client {
            let row: Row = client.query_one(procedure, params).await?;
            let total_amount: Option<Decimal> = row.try_get(0)?;
            let total_amount_f64 = total_amount.map(|d| d.to_f64()
                .ok_or("Failed to convert Decimal to f64")).transpose()?;
            return Ok(total_amount_f64);
        }
        Ok(None)
    }

    // Execute known_wallet_transaction_count stored procedure
    async fn known_wallet_transaction_count(&self, address: &str) -> Result<i32, Box<dyn Error>> {
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&address];
        let procedure = "SELECT get_confirmed_transaction_count($1)";
        if let Some(client) = &self.client {
            let row = client.query_one(procedure, params).await?;
            let count: i32 = row.get(0);
            Ok(count)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No database client available")))
        }
    }

    // Execute unknown_wallet_deposit_amount stored procedure
    async fn unknown_wallet_deposit_amount(&self) -> Result<Option<f64>, Box<dyn Error>> {
        let procedure = "SELECT get_total_confirmed_amount_excluding_known_clients()";
        if let Some(client) = &self.client {
            let row: Row = client.query_one(procedure, &[]).await?;
            let total_amount: Option<Decimal> = row.try_get(0)?;
            let total_amount_f64 = total_amount.map(|d| d.to_f64()
                .ok_or("Failed to convert Decimal to f64")).transpose()?;
            return Ok(total_amount_f64);
        }
        Ok(None)
    }

    // Execute get_confirmed_transaction_count_excluding_known_clients stored procedure
    async fn unknown_wallet_transaction_count(&self) -> Result<Option<i32>, Box<dyn Error>> {
        let procedure = "SELECT get_confirmed_transaction_count_excluding_known_clients()";
        if let Some(client) = &self.client {
            let row: Row = client.query_one(procedure, &[]).await?;
            let transaction_count: Option<i32> = row.try_get(0)?;
            return Ok(transaction_count);
        }
        Ok(None)
    }

    // Execute get_smallest_confirmed_amount stored procedure
    async fn get_smallest_confirmed_amount(&self) -> Result<Option<f64>, Box<dyn Error>> {
        let procedure = "SELECT get_smallest_confirmed_amount()";
        if let Some(client) = &self.client {
            let row: Row = client.query_one(procedure, &[]).await?;
            let min_amount: Option<Decimal> = row.try_get(0)?;
            let min_amount_f64 = min_amount.map(|d| d.to_f64()
                .ok_or("Failed to convert Decimal to f64")).transpose()?;
            return Ok(min_amount_f64);
        }
        Ok(None)
    }

    // Execute get_max_confirmed_amount stored procedure
    async fn get_max_confirmed_amount(&self) -> Result<Option<f64>, Box<dyn Error>> {
        let procedure = "SELECT get_max_confirmed_amount()";
        if let Some(client) = &self.client {
            let row: Row = client.query_one(procedure, &[]).await?;
            let max_amount: Option<Decimal> = row.try_get(0)?;
            let max_amount_f64 = max_amount.map(|d| d.to_f64()
                .ok_or("Failed to convert Decimal to f64")).transpose()?;
            return Ok(max_amount_f64);
        }
        Ok(None)
    }

    // Execute insert_known_client stored procedure
    async fn insert_known_client(&self, known_customer: &KnownCustomers) -> Result<(), Box<dyn Error>> {
        let procedure = "CALL insert_known_client($1, $2)";
        if let Some(client) = &self.client {
            client.execute(procedure, &[&known_customer.name, &known_customer.address]).await?;
        }
        Ok(())
    }

    // Execute insert_transaction stored procedure
    async fn insert_transaction(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        let procedure = "CALL insert_transaction($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)";
        if let Some(client) = &self.client {
            let amount = Decimal::from_f64(transaction.amount).ok_or("Invalid amount")?;
            client.execute(procedure, &[
                &transaction.involves_watchonly,
                &transaction.account,
                &transaction.address,
                &transaction.category,
                &amount,
                &transaction.label,
                &transaction.confirmations,
                &transaction.blockhash,
                &transaction.blockindex,
                &transaction.blocktime,
                &transaction.txid,
                &transaction.vout,
                &transaction.walletconflicts,
                &transaction.time,
                &transaction.timereceived,
                &transaction.bip125_replaceable,
            ]).await?;
        }
        Ok(())
    }

    // Class deconstructor
    fn close(&mut self) {
        self.client = None;
    }
}
