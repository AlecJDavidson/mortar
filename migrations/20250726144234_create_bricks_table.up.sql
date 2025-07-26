-- Add up migration script here

CREATE TABLE IF NOT EXISTS poke(
    id CHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    language VARCHAR(255) NOT NULL,
    source_path VARCHAR(255) NOT NULL,
    active BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_invoked TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

