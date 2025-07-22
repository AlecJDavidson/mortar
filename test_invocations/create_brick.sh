curl -X POST http://127.0.0.1:3000/create-brick \
     -H "Content-Type: application/json" \
     -d '{
           "name": "Hello Brick",
           "language": "Bash",
           "source_path": "test_scripts/hello.sh"
         }'
