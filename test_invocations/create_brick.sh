#! /bin/bash

### UUID is temp ###

curl -X POST http://127.0.0.1:3000/api/brick\
     -H "Content-Type: application/json" \
     -d '{
           "id": "1",
           "name": "Hello Brick2",
           "language": "Bash",
           "source_path": "test_scripts/hello.sh"
         }'
