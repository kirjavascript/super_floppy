#!/bin/sh
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path"

FAST=false

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "build_web.sh [--fast]"
      echo "  --fast: skip optimization step"
      exit 0
      ;;
    --fast)
      shift
      FAST=true
      ;;
    *)
      break
      ;;
  esac
done

FOLDER_NAME=${PWD##*/}
CRATE_NAME=$FOLDER_NAME # assume crate name is the same as the folder name
CRATE_NAME_SNAKE_CASE="${CRATE_NAME//-/_}" # for those who name crates with-kebab-case
WASM_BIN="ui/public/${CRATE_NAME_SNAKE_CASE}_bg.wasm"

rm -f "${WASM_BIN}"

echo "Building rust…"
BUILD=release
cargo build -p "${CRATE_NAME}" --release --lib --target wasm32-unknown-unknown

# Get the output directory (in the workspace it is in another location)
TARGET=$(cargo metadata --format-version=1 | jq --raw-output .target_directory)

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME_SNAKE_CASE}.wasm"
WASM_PATH="${TARGET}/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}"
# using no-modules instead of the newer 'web' to interoperate with vite well
wasm-bindgen "${WASM_PATH}" --out-dir ui/public --target no-modules --no-typescript

cp prune.table ui/public/prune.table

if [[ "${FAST}" == false ]]; then
  echo "Optimizing wasm…"
  # to get wasm-opt:  apt/brew/dnf install binaryen
  # https://github.com/WebAssembly/binaryen/releases
  wasm-opt "${WASM_BIN}" -O2 --fast-math -o "${WASM_BIN}" # add -g to get debug symbols
fi
