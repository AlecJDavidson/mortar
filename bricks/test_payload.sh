#!/bin/bash

# Check if PAYLOAD is sent and print it
if [ -n "$PAYLOAD" ]; then
    echo "JSON Payload received:"
    echo "$PAYLOAD"
else
    echo "No JSON Payload received."
fi
