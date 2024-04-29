#!/bin/bash

ROOT_PATH="$(cd "$(dirname "$0")" && pwd)"
echo "Current Path: $ROOT_PATH"

if [ -z "$1" ] || [ -z "$2" ]; then
	echo "Error missing argument, pack failed."
	echo "Usage: ./package.sh [version] [debug|release]"
	echo "example: ./package.sh 1.1.0 release"
	exit 1
fi

if [ "$2" == "release" ]; then
        TARGET_DIR="$ROOT_PATH/target/release"
else
        TARGET_DIR="$ROOT_PATH/target/debug"
fi

cd "$ROOT_PATH/rbnserver"

if [ "$2" == "release" ]; then
        cargo build --release
else
        cargo build
fi

# backup rbndata.db file
if [ -f "$TARGET_DIR/rbndata.db" ]; then
	cp -rvf "$TARGET_DIR/rbndata.db" "$TARGET_DIR/rbndata.db.bak"
fi

# copy assets to target dir
cp -rvf "$ROOT_PATH/rbnserver/migrations" "$TARGET_DIR"
cp -rvf "$ROOT_PATH/rbnserver/rsfdata" "$TARGET_DIR"
cp -rvf "$ROOT_PATH/rbnserver/templates" "$TARGET_DIR"


if [ "$2" == "release" ]; then
	RELEASE_DIR="$ROOT_PATH/Release"
	RELEASE_TEMP="$RELEASE_DIR/RBNServer"
	RELEASE_FILE="RBNServer_$1.tar.gz"

	if [ -d "$RELEASE_DIR/RBNServer" ]; then
		rm -rvf "$RELEASE_TEMP"
	fi

	mkdir "$RELEASE_DIR/RBNServer"
	cp -rvf "$ROOT_PATH/rbnserver/migrations" "$RELEASE_TEMP"
	cp -rvf "$ROOT_PATH/rbnserver/rsfdata" "$RELEASE_TEMP"
	cp -rvf "$ROOT_PATH/rbnserver/templates" "$RELEASE_TEMP"
	cp -rvf "$TARGET_DIR/rbnserver" "$RELEASE_TEMP"

	cd "$RELEASE_DIR"
	tar -czf $RELEASE_FILE RBNServer
	cd -

	echo "release over, release file is: $RELEASE_FILE"
fi

# copy assets to release dir

echo "pack over, out dir is: $TARGET_DIR"

exit