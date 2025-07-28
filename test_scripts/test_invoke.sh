#! /bin/bash

curl -X POST http://127.0.0.1:3000/api/brick/invoke\
     -H "Content-Type: application/json" \
     -d '{
           "payload": "Test Payload",
         }'
