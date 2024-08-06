#!/bin/sh

# Run the migration SQL file
psql $DB_CONNECTION_STRING -f ./migrations/0.sql
