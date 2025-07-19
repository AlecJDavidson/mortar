#! /bin/bash

curl -X POST http://127.0.0.1:3000/invoke-brick \
     -H "Content-Type: application/json" \
     -d '{
           "id": "your-test-brick-id",
           "name": "Test Brick",
           "creation_time": "2025-07-19T14:30:00Z",
           "last_invocation": "2025-07-19T15:30:00Z",
           "language": "Bash",
           "source_path": "test_scripts/hello.sh",
           "active": true
         }'
