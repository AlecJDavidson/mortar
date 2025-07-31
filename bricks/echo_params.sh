#!/bin/bash

# Print all environment variables (optional, just for debugging)
echo "Environment Variables:"
env | grep -i '^PARAM_'

# Check if PAYLOAD is set and print it
if [ -n "$PAYLOAD" ]; then
    echo "JSON Payload received:"
    echo "$PAYLOAD"
fi

# Example of accessing specific parameters (if any)
echo "Accessing individual params:"
for param in $(env | grep '^PARAM_' | cut -d= -f1); do
    echo "$param: ${!param}"
done
