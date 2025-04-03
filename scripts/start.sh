#!/bin/bash

# Start Oracle Database
echo "Starting Oracle Database..."
docker-compose up -d oracle-db

# Run the setup script
echo "Running database setup..."
./scripts/setup_oracle.sh

# Start the Rust application
echo "Starting Rust application..."
cargo run 