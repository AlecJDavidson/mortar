# FROM debian:testing-slim
FROM ubuntu:latest

# Install any dependencies if needed
RUN apt-get update && apt-get install -y \
    libc6 \
    libgcc1 \
    libstdc++6 \
    libssl3 # Added libssl dependency

# Copy the mortar binary into the container
COPY mortar /usr/local/bin/mortar
# COPY bricks /usr/local/bin/bricks
COPY bricks /bricks

# Set entrypoint to run mortar with the environment variables
ENTRYPOINT ["mortar"]
