# Run posgres
```
docker-compose up
```
# Run indexer
```
cargo run --bin indexer
```
# Reset DB
```
diesel migration redo
```
# Create DB
```
diesel migration run
```
