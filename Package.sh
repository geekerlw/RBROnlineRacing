#!/bin/sh

ROOT_PATH="$(cd "$(dirname "$0")" && pwd)"
echo "Current Path: $ROOT_PATH"

if [ "$1" == "release" ]; then
        TARGET_DIR="$ROOT_PATH/target/release"
else
        TARGET_DIR="$ROOT_PATH/target/debug"
fi

cd "$ROOT_PATH/rbnserver"

if [ "$1" == "release" ]; then
        cargo build --release
else
        cargo build
fi

cp -rvf "$ROOT_PATH/rbnserver/migrations" "$TARGET_DIR"
cp -rvf "$ROOT_PATH/rbnserver/rsfdata" "$TARGET_DIR"
cp -rvf "$ROOT_PATH/rbnserver/templates" "$TARGET_DIR"

echo "Package over."

exit