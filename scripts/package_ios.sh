#!/usr/bin/env bash

bold="\e[0;32m"
reset="\e[0m"

# create a path for the directory that receives artifacts defaulting to `.`
output_dir=${LOCKSMITH_IOS_OUT}
mkdir -p "$output_dir"

# Name of the library with correct capitalization.
display_name=${LOCKSMITH_DISPLAY_NAME:?}

# lowercase the display name using Bash's shell parameter expansion
package_name="${display_name}"

# what we call the compiled library (both .a and .dylib)
artifact_name="lib$package_name"

# Framework type: static (default) or dynamic
# Set LOCKSMITH_FRAMEWORK_TYPE=dynamic to build dynamic framework
FRAMEWORK_TYPE="${LOCKSMITH_FRAMEWORK_TYPE:-dynamic}"

# the name of the created framework
framework="$display_name"FFI.xcframework

# where we will place generated Swift files (SPM target path)
generated=./platforms/ios/Sources/LockSmith

# save off the location of tmp for further use
tmp=$(mktemp -d)

# get our binding, headers, and include directories created
mkdir -p $tmp/{bindings,include}

# We call this at the end of the run, error or otherwise
function clean {
  printf "\n→ %s\n" "Cleaning up..."
  declare -a items
  items=("$tmp")
  for item in "${items[@]}"; do
     printf "\t%s\n" "Deleting $item..."
     rm -rf "$item" 2> /dev/null || true
  done;
}
trap clean EXIT

function package {
  printf "\n${bold}%s${reset}\n" "Releasing ${display_name} (${FRAMEWORK_TYPE} framework)"

  export IPHONEOS_DEPLOYMENT_TARGET="15.0"
  export RUSTFLAGS="-C link-arg=-Wl,-application_extension"

  declare -a targets
  targets=(
    aarch64-apple-ios-sim
    aarch64-apple-ios
    x86_64-apple-ios
  )

  for arch in "${targets[@]}"; do
    printf "\n→ %s\n" "Compiling for $arch..."
    
    if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
      # Build as cdylib for dynamic framework
      cargo rustc --lib --package "${package_name}" --target "$arch" --release -- --crate-type=cdylib
      
      # Strip debug symbols from .dylib
      printf "   → Stripping debug symbols from $arch library...\n"
      xcrun strip -S -x "./target/$arch/release/$artifact_name.dylib"
    else
      # Build as staticlib (default)
      cargo build --lib --package "${package_name}" --target "$arch" --release
      
      # Strip debug symbols from .a
      printf "   → Stripping debug symbols from $arch library...\n"
      xcrun strip -S -x "./target/$arch/release/$artifact_name.a"
    fi
  done

  mkdir -p "$tmp"/target/universal-ios/release

  # Create universal libraries
  if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
    printf "\n→ Creating universal dynamic library...\n"
    lipo -create \
        ./target/aarch64-apple-ios-sim/release/"$artifact_name".dylib \
        ./target/x86_64-apple-ios/release/"$artifact_name".dylib \
        -output $tmp/target/universal-ios/release/"$artifact_name".dylib
    
    # Strip the universal library as well
    printf "   → Stripping universal library...\n"
    xcrun strip -S -x "$tmp/target/universal-ios/release/$artifact_name.dylib"
  else
    printf "\n→ Creating universal static library...\n"
    lipo -create \
        ./target/aarch64-apple-ios-sim/release/"$artifact_name".a \
        ./target/x86_64-apple-ios/release/"$artifact_name".a \
        -output $tmp/target/universal-ios/release/"$artifact_name".a
    
    # Strip the universal library as well
    printf "   → Stripping universal library...\n"
    xcrun strip -S -x "$tmp/target/universal-ios/release/$artifact_name.a"
  fi

  printf "\n→ %s\n" "Generating Swift code"

  # Generate swift bindings (using bindgen-cli feature for the binary only)
  cargo run --bin uniffi-bindgen --features bindgen-cli generate \
      ./target/aarch64-apple-ios-sim/release/"$artifact_name".dylib \
      --library \
      --language swift \
      --no-format \
      --out-dir "$tmp"/bindings

  # Clean any previous generated sources to avoid stale references
  rm -rf "$generated"
  mkdir -p "$generated"
  # Remove any legacy generated path from earlier naming (best-effort)
  rm -rf ./platforms/ios/Sources/MySample 2>/dev/null || true
  # Move generated swift bindings
  mv "$tmp"/bindings/*.swift "$generated"

  # Massage the generated files to fit xcframework
  mkdir -p "$tmp"/include
  mv "$tmp"/bindings/*.h "$tmp"/include 2>/dev/null || true
  cat "$tmp"/bindings/*.modulemap > "$tmp"/include/module.modulemap 2>/dev/null || true

  # Create XCFramework based on type
  if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
    printf "\n→ Creating dynamic XCFramework...\n"
    xcodebuild -create-xcframework \
       -library ./target/aarch64-apple-ios/release/"$artifact_name".dylib \
       -headers "$tmp"/include \
       -library "$tmp"/target/universal-ios/release/"$artifact_name".dylib \
       -headers "$tmp"/include \
       -output  "$framework"
  else
    printf "\n→ Creating static XCFramework...\n"
    xcodebuild -create-xcframework \
       -library ./target/aarch64-apple-ios/release/"$artifact_name".a \
       -headers "$tmp"/include \
       -library "$tmp"/target/universal-ios/release/"$artifact_name".a \
       -headers "$tmp"/include \
       -output  "$framework"
  fi

  # remove previous releases (any old *FFI.xcframework, including legacy names)
  rm -rf "${output_dir:?}"/*FFI.xcframework

  printf "\n→ %s ${bold}%s${reset}...\n" "Placing framework in" "$output_dir"
  mv "$framework" "$output_dir"
  
  # Helper function to convert bytes to MB
  bytes_to_mb() {
    local bytes=$1
    if command -v bc >/dev/null 2>&1; then
      echo "scale=2; $bytes / 1024 / 1024" | bc
    else
      # Fallback: use awk for floating point division
      awk "BEGIN {printf \"%.2f\", $bytes / 1048576}"
    fi
  }
  
  # Show size summary
  printf "\n${bold}✓ Build complete!${reset}\n"
  printf "  Type: ${FRAMEWORK_TYPE}\n"
  printf "  Location: $output_dir/$framework\n"
}

package
