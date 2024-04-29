## RUN RBNServer in localhost

### Prepare
rbnserver require sqlite3, please install by yourself. and manual copy `rbrdata.db` into the execute path near `rbrserver`

### Build and Run

1. build and pack files to target dir;
use ./package.sh version debug|release to generate working environment.
```
./package.sh [version_str] [build_type]
```

2. after copy test `rbndata.db` into `target/[build_type]` like `target/debug`
```
cd rbnserver
export RUST_LOG=info
cargo run
```

3. after server running, access web via localhost:
web: http://127.0.0.1:23555/

4. once need to reload html templates. kill and restart rbnserver.