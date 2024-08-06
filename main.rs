/*
main.rs
6/29/24

This file is the main entrypoint for the executable.
It calls each handler and will log the execution time on completion.
If any errors occur at lower levels in the callstack, they are
propigated and logged by the main function.
The Tokio runtime is also managed at this level.
*/
use std::time::Instant;
use logger::{Logger, LogLevel, log};
use config::Config;

#[tokio::main]
async fn main() {
    // Initialize logger
    let logger = Logger::new(LogLevel::Info, "log.txt");
    log!(logger, info, "Starting Service");

    // Load configuration file into program memory
    let config = match Config::from_env() {
        Ok(config) => {
            log!(logger, info, "ENV Loaded");
            config
        },
        Err(e) => {
            log!(logger, info, "Failed to parse ENV: {}", e);
            return;
        }
    };

    // Upload the input data to the db
    let load_time = Instant::now();
    match handlers::load_data(&config).await {
        Ok(_) => {
            let load_time_elapsed = load_time.elapsed();
            log!(logger, info, "Load Data Execution Time: {:?}",
                load_time_elapsed);
        },
        Err(e) => log!(logger, info, "Error load_data: {}", e),
    }

    // Query for known customer deposits
    let known_customer_time = Instant::now();
    match handlers::known_customer_deposits(&config).await {
        Ok(_) => {
            let known_customer_time_elapsed = known_customer_time.elapsed();
            log!(logger, info, "Known Customer Deposits Execution Time: {:?}",
                known_customer_time_elapsed);
        },
        Err(e) => log!(logger, info, "Error known_customer_deposits: {}", e),
    }

    // Query for unknown customer deposits
    let unknown_customer_time = Instant::now();
    match handlers::unknown_customer_deposits(&config).await {
        Ok(_) => {
            let unknown_customer_time_elapsed = unknown_customer_time.elapsed();
            log!(logger, info, "Unknown Customer Deposits Execution Time: {:?}",
                unknown_customer_time_elapsed);
        },
        Err(e) => log!(logger, info, "Error unknown_customer_deposits: {}", e),
    }

    // Calculate the range of deposits (min & max)
    let calculate_range_time = Instant::now();
    match handlers::calculate_range(&config).await {
        Ok(_) => {
            let calculate_range_time_elapsed = calculate_range_time.elapsed();
            log!(logger, info, "Calculate Range Execution Time: {:?}",
                calculate_range_time_elapsed);
        },
        Err(e) => log!(logger, info, "Error calculate_range: {}", e),
    }
}
