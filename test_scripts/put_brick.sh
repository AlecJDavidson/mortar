#! /bin/bash

curl -X PUT http://127.0.0.1:3000/api/brick/b53e2423-efbe-4d5f-b3b2-7439bc2a2986\
     -H "Content-Type: application/json" \
     -d '{
           "name": "Hello Brick Big Money",
           "language": "Bash",
           "source_path": "test_scripts/hello.sh"
         }'
