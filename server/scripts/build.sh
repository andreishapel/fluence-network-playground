#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

DIRNAME=$(pwd)
ARTIFACTS_FOLDER=artifacts
SERVICE_NAME=$1

mkdir -p $ARTIFACTS_FOLDER

cd src/services/${SERVICE_NAME}
cargo update --aggressive
marine build --release
cp target/wasm32-wasi/release/*.wasm $DIRNAME/$ARTIFACTS_FOLDER
rm -rf target
wget https://github.com/fluencelabs/sqlite/releases/download/v0.14.0_w/sqlite3.wasm
mv sqlite3.wasm $DIRNAME/$ARTIFACTS_FOLDER
cd $DIRNAME
