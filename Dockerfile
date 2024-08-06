# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Install PostgreSQL client for running migrations
RUN apt-get update && apt-get install -y postgresql-client

# Grant execution permissions to the migration script
RUN chmod +x ./run_migrations.sh

# Build the application
RUN cargo build --release

# Set the startup command to run your application
CMD ["./target/release/kobayashi-maru"]
