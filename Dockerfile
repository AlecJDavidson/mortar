FROM ubuntu:latest

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libc6 \
    libgcc1 \
    libstdc++6 \
    libssl3 # Add libssl dependency here

# Create a directory for the bricks and mortar binary
RUN mkdir -p /app/bricks

# Copy the mortar binary into the container
COPY target/release/mortar /usr/local/bin/mortar

# Ensure proper permissions on the binary
RUN chmod +x /usr/local/bin/mortar

# Set entrypoint to run mortar with the environment variables
ENTRYPOINT ["mortar"]
