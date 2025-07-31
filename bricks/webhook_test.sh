#!/bin/bash

# Read parameters and payload from environment variables
PARAM1=${PARAM1:-"default_value_1"}
PARAM2=${PARAM2:-"default_value_2"}
PAYLOAD=${PAYLOAD:-"{}"}  # Default to an empty JSON object if not set

echo "Received PARAM1: $PARAM1"
echo "Received PARAM2: $PARAM2"
echo "Processing payload: $PAYLOAD"

# Define the URL for the webhook
WEBHOOK_URL="https://webhook.site/1bc1e2f9-6295-4979-81eb-6ba6983c2559"

# Create a JSON object with parameters and payload
JSON_PAYLOAD=$(cat <<EOF
{
  "params": {
    "PARAM1": "$PARAM1",
    "PARAM2": "$PARAM2"
  },
  "payload": $PAYLOAD
}
EOF
)

echo "Sending data to webhook..."

# Send the JSON payload to the webhook URL using curl
curl -X POST -H "Content-Type: application/json" -d "$JSON_PAYLOAD" "$WEBHOOK_URL"

if [ $? -eq 0 ]; then
    echo "Data sent successfully!"
else
    echo "Failed to send data."
fi
