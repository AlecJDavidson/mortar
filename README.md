# Mortar

**Mortar** is a backend API designed to run functions/scripts on a server. It currently supports running Python and Bash scripts, with plans to extend support to Rust, C, Go, and Node.js in the future.

## Table of Contents

- [Technology Stack](#technology-stack)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Running with Docker Compose](#running-with-docker-compose)
  - [Running from Source](#running-from-source)
- [Usage](#usage)
  - [Create Brick](#create-brick)
  - [Delete Brick](#delete-brick)
  - [Patch Brick](#patch-brick)
  - [Put Brick](#put-brick)
  - [Invoke Brick](#invoke-brick)
- [Troubleshooting](#troubleshooting)

## Technology Stack

- **API**: Rust
- **Database**: PostgreSQL
- **Containerization**: Docker Compose

## Getting Started

### Prerequisites

Make sure you have the following installed on your system:
- Docker and Docker Compose
- Rust (if running from source)
- PostgreSQL CLI tools (if running from source)

### Running with Docker Compose

1. Clone this repository:

   ```bash
   git clone https://github.com/AlecJDavidson/mortar.git
   cd mortar
   ```

2. Start the services using Docker Compose:
    
   ```bash
   docker-compose up -d
   ```

### Running from Source 

1. Ensure PostgreSQL is running. 

2. Run database migrations: 

   ```bash
   cargo install sqlx-cli
   ~/.cargo/bin/sqlx migrate add -r create_bricks_table
   ```

3. Create the bricks table by adding the following SQL to your migration files:

   ```bash
    -- Up Migration
    CREATE TABLE IF NOT EXISTS bricks(
        id CHAR(36) PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL UNIQUE,
        language VARCHAR(255) NOT NULL,
        source_path VARCHAR(255) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_invoked TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    
    -- Down Migration
    DROP TABLE IF EXISTS bricks;
   ```

4. Apply Migrations:

   ```bash
    ~/.cargo/bin/sqlx migrate run
   ```

## Usage 

### All scripts you want to run need to be placed in the bricks directory.

## Create Brick

## Create a brick with the following command:

   ```bash
   curl -X POST http://127.0.0.1:3333/api/brick \
     -H "Content-Type: application/json" \
     -d '{
           "id": "1",
           "name": "Hello World!",
           "language": "Bash",
           "source_path": "/app/bricks/hello.sh"
         }'
   ```

### Ensure ./bricks/hello.sh exists with the following content:

   ```bash
   #! /bin/bash

   echo Hello World!
   ```

## Delete Brick

### Delete a brick using its ID: 

   ```bash
   curl -X DELETE http://127.0.0.1:3333/api/brick/<brick-id> \
     -H "Content-Type: application/json"
   ```

## Patch Brick

## Patch a brick using its ID: 
## Patch (update) an existing brick: 

   ```bash
   curl -X PATCH http://127.0.0.1:3333/api/brick/<brick-id> \
         -H "Content-Type: application/json" \
         -d '{
               "name": "Hello Brick Patched",
               "language": "Bash",
               "source_path": "/app/bricks/hello.sh"
             }'   
   ```

## Put Brick
### Put a brick using its ID: 
### Replace an existing brick with new data:  

   ```bash
   curl -X PUT http://127.0.0.1:3333/api/brick/<brick-id> \
         -H "Content-Type: application/json" \
         -d '{
               "name": "Hello Brick Put",
               "language": "Bash",
               "source_path": "/app/bricks/hello.sh"
             }'
   ```

## Invoke Brick

### Invoke a brick to run the corresponding script and send a JSON payload: 

   ```bash
   curl -X POST http://127.0.0.1:3333/api/brick/invoke/<brick-id> \
         -H "Content-Type: application/json" \
         -d '{
               "payload": "Test Payload",
             }'   
   ```
### Invoke a brick and send query parameters: 

   ```bash
   curl -X POST "http://localhost:3000/api/brick/invoke/b5753051-20f4-4921-8c8a-84968f90e397?param1=value1&param2=value2" \
     -H "Content-Type: application/json"
   ```

## Troubleshooting

### Docker Build Error on Certain Linux Distros 
### If you encounter an error while building the Docker image, try: 

   ```bash
   sudo setenforce 0
   ```
