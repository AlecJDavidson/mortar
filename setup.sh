#!/bin/sh
# Wait for PostgreSQL service to be available (if running via Docker Compose)
echo "Waiting for PostgreSQL to be available..."
until pg_isready -h localhost -p 5432; do
  echo "Waiting for PostgreSQL to be ready..."
  sleep 1
done

echo "PostgreSQL is available"

# Run database migrations using sqlx-cli
export DATABASE_URL=postgres://mortar:mortar@localhost:5432/mortar

# Ensure the bricks table exists (as a fallback)
psql $DATABASE_URL -c "
CREATE TABLE IF NOT EXISTS bricks(
    id CHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    language VARCHAR(255) NOT NULL,
    source_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_invoked TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"
echo "Table check completed"

# Run the mortar binary explicitly from /usr/local/bin/
# target/release/mortar
# /usr/local/bin/mortar#
