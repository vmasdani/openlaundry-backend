# openlaundry-backend

1. Create .env file which contains
```sh
DATABASE_URL=openlaundry-backend.sqlite3
<<<<<<< HEAD
SERVER_PORT=8000
=======
SERVER_PORT=8086
>>>>>>> 0d8914fe2e0de4eebc460bf179dc2e3c2c4fc5e0
```

2. Run
```sh
cargo run
```

3. Cross compiling for MUSL x86_64 ([cross](https://github.com/rust-embedded/cross))
```sh
cross build --target x86_64-unknown-linux-musl --release
```
