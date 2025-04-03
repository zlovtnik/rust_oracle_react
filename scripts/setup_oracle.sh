#!/bin/bash

# Set the new SYSTEM password
echo "Setting new SYSTEM password..."
docker exec oracle-db ./setPassword.sh oracle123

# Create our application user and grant permissions
echo "Creating application user and granting permissions..."
docker exec oracle-db sqlplus -S sys/oracle123@//localhost:1521/FREEPDB1 as sysdba << EOF
CREATE USER nfe_app IDENTIFIED BY nfe_app_pwd;
GRANT CREATE SESSION TO nfe_app;
GRANT CREATE TABLE TO nfe_app;
GRANT UNLIMITED TABLESPACE TO nfe_app;
ALTER SESSION SET CURRENT_SCHEMA = nfe_app;

-- Run our migration
@/opt/oracle/scripts/migrations/20240320000000_create_nfe_identifications.sql

EXIT;
EOF

# Verify the table was created
echo "Verifying table creation..."
docker exec oracle-db sqlplus -S nfe_app/nfe_app_pwd@//localhost:1521/FREEPDB1 << EOF
SELECT table_name FROM user_tables WHERE table_name = 'NFE_IDENTIFICATIONS';
EXIT;
EOF

echo "Database setup completed!" 