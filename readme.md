# openlaundry-backend

1. Create .env file which contains
```sh
DATABASE_URL=openlaundry-backend.sqlite3
```

2. Run
```sh
cargo run
```

3. Cross compiling for MUSL x86_64 ([cross](https://github.com/rust-embedded/cross))
```sh
cross build --target x86_64-unknown-linux-musleabihf --release
```
