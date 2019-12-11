# actix-todo
todo demo app power by actix-web

power by:

- [actix-web](https://actix.rs/)
- [diesel](http://diesel.rs/guides/getting-started/)

## install

install diesel-cli

```bash
cargo install diesel_cli --no-default-features --features sqlite --force
```

clone source code
```bash
git clone https://github.com/PrivateRookie/actix-todo.git

cd actix-todo

# edit config(optional)
# vi .env
# sqlite3 database file
DATABASE_URL=test.db
# log level, set it to error to get better perform
RUST_LOG="actix_web=info"
# listen host and port
SERVER_ADDR="0.0.0.0:8080"

# run migration
diesel migration run

# start app
cargo run

# or with release mode
cargo run --release
```

## usage

event api

```bash
# list all events
curl --request GET \
  --url http://localhost:8080/events/ \
  --header 'content-type: application/json'

# create event
curl --request POST \
  --url http://localhost:8080/events/ \
  --header 'content-type: application/json' \
  --data '{"content": "reading"}'

# update event
curl --request PUT \
  --url http://localhost:8080/events/ \
  --header 'content-type: application/json' \
  --data '{"uid": "d848e757-ee2a-4a37-a572-a47cd65e9044","finished": true,"content": "reading done"}'

# delete event
curl --request DELETE \
  --url http://localhost:8080/events/ \
  --header 'content-type: application/json' \
  --data '{"uid": "d848e757-ee2a-4a37-a572-a47cd65e9044"}'
```

## TODO 

- [ ] add unit test
- [ ] add gui
- [ ] add Dockerfile
