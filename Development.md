## RUN RBNServer in localhost

### 

1. build and pack files to target dir;
use ./package.sh version debug|release to generate working environment.
```
./package.sh [version_str] [build_type]
```

2. copy test `rbndata.db` into `target/[build_type]` like `target/debug`
```
export RUST_LOG=info
./rbnserver &
```

3. after server running, access web via localhost:
web: http://127.0.0.1:23555/

4. once need to reload html templates. kill and restart rbnserver.