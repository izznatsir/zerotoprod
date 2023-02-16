#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
	>&2 echo "Error: psql is not installed."
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	>&2 echo "Error: sqlx-cli is not installed."
	>&2 echo "Run:"
	>&2 echo "	cargo install sqlx-cli --version '~0.6' sqlx-cli --no-default-features --features rustls,postgres"
	>&2 echo "to install it."
	exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'neon'
DB_NAME="${POSTGRES_DB:=neon}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
docker run --name=zerotoprod_postgres \
	-e POSTGRES_USER=${DB_USER} \
	-e POSTGRES_PASSWORD=${DB_PASSWORD} \
	-e POSTGRES_DB=${DB_NAME} \
	-p "${DB_PORT}":5432 \
	-d postgres:15 \
	-c max_connections=1000
# ^ Increased maximum number of connections for testing purposes

# Wait until the postgres container is up and running.
export PGPASSWORD=${DB_PASSWORD}
until psql -h "localhost" -p "${DB_PORT}" -d "${DB_NAME}" -U "${DB_USER}" -c "\q"; do
	>&2 echo "Postgres is still unavailable - waiting."
	sleep 1
done

export DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations..."

sqlx database create
sqlx migrate run
