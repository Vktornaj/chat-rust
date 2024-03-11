# Chat API

Run
``` bash
docker compose -f ./compose/compose.yml up --build 
```

Install sqlx cli
``` bash
cargo install sqlx-cli
```
Usage
``` bash
sqlx migrate info --source ./entry/migrations
sqlx migrate run --source ./entry/migrations
sqlx migrate revert --source ./entry/migrations
```
