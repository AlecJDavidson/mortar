### Start the DB with docker compose ###

docker compose up -d

### Connect using psql ###

psql -h localhost -p 5432 -U mortar -W mortar

### Create the Bricks Table ###

# TODO: Fix last invoked 

CREATE TABLE bricks(
    id CHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    language VARCHAR(255) NOT NULL,
    source_path VARCHAR(255) NOT NULL,
    active BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_invoked TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

### Create Migrations ###

# CLI For migration
cargo install sqlx-cli

# create a migration
~/.cargo/bin/sqlx migrate add -r create_bricks_table


# Update the up/down/revert
-- Add up migration script here

CREATE TABLE IF NOT EXISTS bricks(
    id CHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    language VARCHAR(255) NOT NULL,
    source_path VARCHAR(255) NOT NULL,
    active BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_invoked TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

-- Add down migration script here

DROP TABLE IF EXISTS bricks;

# perform migration up
~/.cargo/bin/sqlx migrate run

# (perform migration down/revert)
~/.cargo/bin/sqlx migrate revert

### Creating a CRUD app using this tutorial ###

https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130


### Notes ###

Query with and/or without Macro

Sqlx have query macro and query function (without macro), you can pick one best for your case.

Example with Macro:

sqlx::query_as!(
    BrickModel,
    r#"SELECT * FROM bricks ORDER by id LIMIT ? OFFSET ?"#,
    limit as i32,
    offset as i32
)

Example without Macro:

sqlx::query_as::<_, BrickModel>(
    r#"SELECT * FROM bricks ORDER by id LIMIT ? OFFSET ?"#
)
