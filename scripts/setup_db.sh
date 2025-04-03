#!/bin/bash

# Check if SQL*Plus is installed
if ! command -v sqlplus &> /dev/null; then
    echo "SQL*Plus is not installed. Please install Oracle Client first."
    exit 1
fi

# Get database credentials from .env file
source .env
DB_USER=$(echo $DATABASE_URL | sed -n 's/oracle:\/\/\([^:]*\):.*@.*/\1/p')
DB_PASS=$(echo $DATABASE_URL | sed -n 's/oracle:\/\/[^:]*:\([^@]*\)@.*/\1/p')
DB_HOST=$(echo $DATABASE_URL | sed -n 's/oracle:\/\/[^@]*@\([^:]*\):.*/\1/p')
DB_PORT=$(echo $DATABASE_URL | sed -n 's/oracle:\/\/[^@]*@[^:]*:\([^\/]*\)\/.*/\1/p')
DB_SERVICE=$(echo $DATABASE_URL | sed -n 's/oracle:\/\/[^@]*@.*\/\(.*\)/\1/p')

# Run migrations using SQL*Plus
echo "Running database migrations..."
sqlplus -S "${DB_USER}/${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_SERVICE}" @migrations/20240320000000_create_nfe_identifications.sql

echo "Database setup complete!" 