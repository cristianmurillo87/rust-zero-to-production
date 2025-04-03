#!/usr/bin/env bash
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# prevents from running docker run if the container is already running (if the SKIP_DOCKER env variable is set to true)
# run: SKIP_DOCKER=true ./scripts/init_db.sh
if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        --name rust-zero-to-production-postgres \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}:5432" \
        -d postgres \
        postgres -N 1000
fi

# keep trying to connect to postgres until connection is successful
until docker exec -it -e PG_PASSWORD="${DB_PASSWORD}" rust-zero-to-production-postgres psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export PG_PASSWORD="${DB_PASSWORD}"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@127.0.0.1:${DB_PORT}/${DB_NAME}
export RUST_LOG=trace

sqlx database create
sqlx migrate run
cargo sqlx prepare