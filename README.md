# Rust Todo MCV

This is a fullstack Rust project, for learning Rust.

## Dev Test

```sh
### Test for Model (Data)
cd backend
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'

### Test for Web (Rest)
cd backend
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'

### Test for the FrontEnd
cd frontend
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```

## DB

```sh
### Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

### optional psql (other terminal)
docker exec -it -u postgres pg psql
```
