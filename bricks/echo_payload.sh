#!/bin/bash

# Print the JSON payload if it exists
if [ -n "$PAYLOAD" ]; then
    echo "JSON Payload received:"
    echo "$PAYLOAD"
else
    echo "No JSON Payload received."
fi
