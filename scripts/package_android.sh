#!/usr/bin/env bash
set -euo pipefail

# basic styling (fallback-safe)
BOLD=${BOLD:-"\e[1m"}
RESET=${RESET:-"\e[0m"}

# create a path for the directory that receives artifacts defaulting to `.`
mkdir -p "${LOCKSMITH_ANDROID_OUT:?}"

# the created shared library name used to generate kotlin
library_name="lib$(echo "${LOCKSMITH_DISPLAY_NAME}" | tr '[:upper:]' '[:lower:]').dylib"

# where we will place generated Kotlin bindings
generated=./platforms/android/lib/src/main/kotlin/

# save off the location of tmp for further use
tmp=$(mktemp -d "${TMPDIR:?}$(echo "${LOCKSMITH_DISPLAY_NAME}" | tr '[:upper:]' '[:lower:]')-android.XXXX")

# We call this at the end of the run, error or otherwise
function clean {
  if [ $? -eq 0 ]; then
    printf "\n→ %s\n" "Cleaning up..."
    printf "\t%s\n"  "Deleting $tmp"
    rm -rf "$tmp" 2> /dev/null || true
  fi
}
trap clean EXIT

function package {
  printf "${BOLD}%s${RESET}\n\n" "Releasing ${LOCKSMITH_DISPLAY_NAME}"

  # Ensure required Rust targets are installed
  declare -a rust_targets
  rust_targets=(
    aarch64-linux-android
    armv7-linux-androideabi
    i686-linux-android
    x86_64-linux-android
  )
  for t in "${rust_targets[@]}"; do
    if ! rustup target list --installed | grep -q "^$t$"; then
      printf "→ Installing Rust target %s\n" "$t"
      rustup target add "$t"
    fi
  done

  # API level for NDK builds (default 21 supports modern devices)
  api_level="${ANDROID_API_LEVEL:-21}"

  # ABI to Rust target mapping (use pairs to support macOS bash 3.2)
  declare -a abis
  abis=(
    "arm64-v8a:aarch64-linux-android"
    "armeabi-v7a:armv7-linux-androideabi"
    "x86:i686-linux-android"
    "x86_64:x86_64-linux-android"
  )

  # build each target
  for entry in "${abis[@]}"; do
    IFS=: read -r abi target_triple <<< "$entry"
    printf "\n→ %s ${BOLD}%s${RESET}\n" "Compiling for" "$abi"
    # the correct subdirectory will be created in $tmp
    cargo ndk \
      --platform "$api_level" \
      --target "$target_triple" \
      --output-dir "$tmp/jniLibs" \
      build --release
    # Strip debug symbols from shared libraries to reduce size
    printf "   → Stripping debug symbols from $abi library...\n"
    find "$tmp/jniLibs/$abi" -name "*.so" -exec strip --strip-debug --strip-unneeded {} \; 2>/dev/null || true
    # a hack to remedy someone's set and forget
    echo -e "${RESET}"
  done

  # start fresh with the jniLibs (we'll copy vendored *.so in shortly)
  rm -rf "${LOCKSMITH_ANDROID_OUT:?}" && mkdir -p "${LOCKSMITH_ANDROID_OUT:?}"
  # the glob was problematic and not quoting the star worked ¯\_(ツ)_/¯
  cp -R "$tmp/jniLibs/"* "${LOCKSMITH_ANDROID_OUT:?}"

  # Optionally copy JNA native libs from a vendored AAR if present; otherwise rely on Gradle dependency
  if compgen -G "./platforms/android/vendor/jna-*.aar" > /dev/null; then
    vendor_aar="$(ls ./platforms/android/vendor/jna-*.aar | head -n1)"
    for entry in "${abis[@]}"; do
      IFS=: read -r abi _ <<< "$entry"
      printf "\t%s ${BOLD}%s${RESET}\n\t" "Moving shared library for" "$abi"
      unzip \
        -j "$vendor_aar" "jni/$abi/libjnidispatch.so" \
        -d "./platforms/android/lib/src/main/jniLibs/$abi/"
    done
  else
    printf "\t%s\n" "No vendored JNA AAR found; skipping native copy and relying on Gradle dependency."
  fi

  # use the shared library to generate bindings
  printf "\n→ %s\n" "Generating Kotlin..."
  cargo run --bin uniffi-bindgen --features bindgen-cli generate \
     "target/release/$library_name" \
    --library \
    --language kotlin \
    --out-dir "$tmp/bindings"

  # Move generated Kotlin bindings
  rm -rf "$generated"
  mkdir -p "$generated"
  cp -R "$tmp/bindings/"* "${generated}"
}

package
