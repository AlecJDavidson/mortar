#! /bin/bash

curl -X POST http://127.0.0.1:3000/api/brick\
     -H "Content-Type: application/json" \
     -d '{
           "id": "1",
           "name": "Test Payload",
           "language": "Bash",
           "source_path": "../bricks/test_payload.sh"
         }'
