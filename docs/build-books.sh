#!/bin/bash
# Builds and tests the books and copies them into `./out/` with the following structure: 
# ./out/
# |- ./out/gui/
# |- ./out/ref/
# |- ./out/rustdoc/
# Also copies static/* to ./out/
# This assumes you already have mdbook, cargo, and nightly rust installed, and are in /docs, relative to the repository root.
set -o xtrace
set -Eeuxo pipefail

echo "Clearing any previous artifacts..."

rm -rf ./out || exit 1
mkdir -p ./out || exit 1

echo "Building The Calypso Book..."

pushd ./guide
mdbook build . && mdbook test . || exit 1
popd

echo "Building The Calypso Reference..."

pushd ./reference
mdbook build . && mdbook test . || exit 1
popd

echo "Building rustdocs"

cargo +nightly rustdoc --workspace -- --document-private-items --enable-index-page || exit 1

echo "Moving files to the out directory"

mv -f ./guide/book ./out/gui || exit 1
mv -f ./reference/book ./out/ref || exit 1
mv -f ../target/doc ./out/rustdoc || exit 1

echo "Copying static files to the out directory"

cp -rf ./static/* ./out/ || exit 1

echo "Success!"