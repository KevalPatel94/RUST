#!/usr/bin/env bash

bold="\e[0;32m"
reset="\e[0m"

# where to place generated Python bindings and the compiled library
output_dir=${LOCKSMITH_PYTHON_OUT:-"./platforms/python/LockSmithExample"}
mkdir -p "$output_dir"

# Name of the library with correct capitalization (must match Cargo.toml package/lib name)
display_name=${LOCKSMITH_DISPLAY_NAME:?}
package_name="${display_name}"
artifact_name="lib$package_name"

# temp workspace
tmp=$(mktemp -d)
mkdir -p "$tmp"/bindings

function clean {
  if [ $? -eq 0 ]; then
    printf "\n→ %s\n" "Cleaning up..."
    rm -rf "$tmp" 2> /dev/null || true
  fi
}
trap clean EXIT

function detect_ext {
  local uname_out
  uname_out="$(uname -s)"
  case "${uname_out}" in
    Darwin*) ext="dylib";;
    Linux*)  ext="so";;
    MINGW*|MSYS*|CYGWIN*) ext="dll";;
    *)       ext="so";;
  esac
}

function package {
  printf "\n${bold}%s${reset}\n" "Releasing ${display_name}"

  detect_ext

  printf "\n→ %s\n" "Compiling for host..."
  cargo build --lib --package "${package_name}" --release

  artifact_path="./target/release/${artifact_name}.${ext}"

  printf "\n→ %s\n" "Generating Python code"
  cargo run --bin uniffi-bindgen --features bindgen-cli generate \
      "${artifact_path}" \
      --library \
      --language python \
      --out-dir "$tmp/bindings"

  # Place generated bindings and the shared library together
  mkdir -p "$output_dir"
  printf "\n→ %s ${bold}%s${reset}\n" "Placing Python bindings and library in" "$output_dir"
  cp -R "$tmp"/bindings/* "$output_dir"/
  cp "${artifact_path}" "$output_dir"/
}

package


