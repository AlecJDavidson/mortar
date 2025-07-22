curl -X POST http://127.0.0.1:3000/create-brick \
     -H "Content-Type: application/json" \
     -d '{
           "id": 0,
           "name": "Hello Brick",
           "creation_time": null,
           "last_invocation": null,
           "language": "Bash",
           "source_path": "test_scripts/hello.sh",
           "active": true
         }'
