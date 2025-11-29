#!/usr/bin/env bash

bold="\e[0;32m"
reset="\e[0m"

# create a path for the directory that receives artifacts defaulting to `.`
output_dir=${LOCKSMITH_IOS_OUT}
mkdir -p "$output_dir"

# Framework type: static (default) or dynamic
FRAMEWORK_TYPE="${LOCKSMITH_FRAMEWORK_TYPE:-dynamic}"

# save off the location of tmp for further use
tmp=$(mktemp -d)

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

# Function to package a single crate
function package_crate {
  local crate_name=$1
  local display_name=$2
  
  printf "\n${bold}%s${reset}\n" "Packaging ${display_name} (${FRAMEWORK_TYPE} framework)"
  
  export IPHONEOS_DEPLOYMENT_TARGET="15.0"
  export RUSTFLAGS="-C link-arg=-Wl,-application_extension"
  
  declare -a targets
  targets=(
    aarch64-apple-ios-sim
    aarch64-apple-ios
    x86_64-apple-ios
  )
  
  for arch in "${targets[@]}"; do
    printf "\n→ %s\n" "Compiling ${crate_name} for $arch..."
    
    if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
      # Build as cdylib for dynamic framework
      cargo rustc --lib --package "${crate_name}" --target "$arch" --release -- --crate-type=cdylib
      
      # Strip debug symbols from .dylib
      printf "   → Stripping debug symbols from $arch library...\n"
      xcrun strip -S -x "./target/$arch/release/lib${crate_name}.dylib"
    else
      # Build as staticlib (default)
      cargo build --lib --package "${crate_name}" --target "$arch" --release
      
      # Strip debug symbols from .a
      printf "   → Stripping debug symbols from $arch library...\n"
      xcrun strip -S -x "./target/$arch/release/lib${crate_name}.a"
    fi
  done
  
  mkdir -p "$tmp"/target/universal-ios/release
  
  # Create universal libraries
  if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
    printf "\n→ Creating universal dynamic library for ${crate_name}...\n"
    lipo -create \
        ./target/aarch64-apple-ios-sim/release/lib"${crate_name}".dylib \
        ./target/x86_64-apple-ios/release/lib"${crate_name}".dylib \
        -output "$tmp/target/universal-ios/release/lib${crate_name}.dylib"
    
    # Strip the universal library as well
    printf "   → Stripping universal library...\n"
    xcrun strip -S -x "$tmp/target/universal-ios/release/lib${crate_name}.dylib"
  else
    printf "\n→ Creating universal static library for ${crate_name}...\n"
    lipo -create \
        ./target/aarch64-apple-ios-sim/release/lib"${crate_name}".a \
        ./target/x86_64-apple-ios/release/lib"${crate_name}".a \
        -output "$tmp/target/universal-ios/release/lib${crate_name}.a"
    
    # Strip the universal library as well
    printf "   → Stripping universal library...\n"
    xcrun strip -S -x "$tmp/target/universal-ios/release/lib${crate_name}.a"
  fi
  
  printf "\n→ %s\n" "Generating Swift code for ${crate_name}"
  
  # Generate swift bindings from the compiled library
  # For proc-macro based crates (like user_domain), UniFFI generates scaffolding
  # automatically, so we generate bindings from the .dylib directly
  cargo run --bin uniffi-bindgen --package locksmith --features bindgen-cli generate \
      ./target/aarch64-apple-ios-sim/release/lib"${crate_name}".dylib \
      --library \
      --language swift \
      --no-format \
      --out-dir "$tmp"/bindings-"${crate_name}"
  
  # Create XCFramework
  local framework_name="${display_name}FFI.xcframework"
  local generated_dir="$tmp/bindings-${crate_name}"
  local include_dir="$tmp/include-${crate_name}"
  
  mkdir -p "$include_dir"
  # Move only the crate-specific headers (filter out dependencies)
  if [ "$crate_name" = "user_domain" ]; then
    mv "$generated_dir"/user_domain*.h "$include_dir" 2>/dev/null || true
    mv "$generated_dir"/domain_common*.h "$include_dir" 2>/dev/null || true
    # Create modulemap from user_domain modulemap
    if [ -f "$generated_dir"/user_domain.modulemap ]; then
      cp "$generated_dir"/user_domain.modulemap "$include_dir/module.modulemap" 2>/dev/null || true
    fi
  else
    mv "$generated_dir"/*.h "$include_dir" 2>/dev/null || true
    cat "$generated_dir"/*.modulemap > "$include_dir/module.modulemap" 2>/dev/null || true
  fi
  
  if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
    printf "\n→ Creating dynamic XCFramework for ${display_name}...\n"
    xcodebuild -create-xcframework \
       -library ./target/aarch64-apple-ios/release/lib"${crate_name}".dylib \
       -headers "$include_dir" \
       -library "$tmp/target/universal-ios/release/lib${crate_name}.dylib" \
       -headers "$include_dir" \
       -output  "$framework_name"
  else
    printf "\n→ Creating static XCFramework for ${display_name}...\n"
    xcodebuild -create-xcframework \
       -library ./target/aarch64-apple-ios/release/lib"${crate_name}".a \
       -headers "$include_dir" \
       -library "$tmp/target/universal-ios/release/lib${crate_name}.a" \
       -headers "$include_dir" \
       -output  "$framework_name"
  fi
  
  # Move framework to output directory
  printf "\n→ %s ${bold}%s${reset}...\n" "Placing framework in" "$output_dir"
  mv "$framework_name" "$output_dir"
  
  # Show size summary
  printf "\n${bold}✓ ${display_name} build complete!${reset}\n"
  printf "  Type: ${FRAMEWORK_TYPE}\n"
  printf "  Location: $output_dir/$framework_name\n"
  
  # Helper function to convert bytes to MB
  bytes_to_mb() {
    local bytes=$1
    if command -v bc >/dev/null 2>&1; then
      echo "scale=2; $bytes / 1024 / 1024" | bc
    else
      awk "BEGIN {printf \"%.2f\", $bytes / 1048576}"
    fi
  }
  
  printf "\n${bold}Binary Sizes for ${display_name}:${reset}\n"
  
  if [ "$FRAMEWORK_TYPE" = "dynamic" ]; then
    device_binary="./target/aarch64-apple-ios/release/lib${crate_name}.dylib"
    if [ -f "$device_binary" ]; then
      device_size=$(ls -lh "$device_binary" 2>/dev/null | awk '{print $5}')
      device_bytes=$(stat -f%z "$device_binary" 2>/dev/null || stat -c%s "$device_binary" 2>/dev/null)
      device_mb=$(bytes_to_mb "$device_bytes")
      printf "  ${bold}Device (arm64):${reset}     %s (%s MB)\n" "$device_size" "$device_mb"
    fi
    
    sim_universal_binary="$tmp/target/universal-ios/release/lib${crate_name}.dylib"
    if [ -f "$sim_universal_binary" ]; then
      sim_universal_size=$(ls -lh "$sim_universal_binary" | awk '{print $5}')
      sim_universal_bytes=$(stat -f%z "$sim_universal_binary" 2>/dev/null || stat -c%s "$sim_universal_binary" 2>/dev/null)
      sim_universal_mb=$(bytes_to_mb "$sim_universal_bytes")
      printf "  ${bold}Simulator (universal):${reset} %s (%s MB)\n" "$sim_universal_size" "$sim_universal_mb"
    fi
    
    xcframework_size=$(du -sh "$output_dir/$framework_name" | cut -f1)
    xcframework_kb=$(du -sk "$output_dir/$framework_name" | cut -f1)
    xcframework_mb=$(bytes_to_mb "$((xcframework_kb * 1024))")
    printf "  ${bold}XCFramework (total):${reset} %s (%s MB)\n" "$xcframework_size" "$xcframework_mb"
  fi
}

# Package both crates
package_crate "locksmith" "locksmith"
package_crate "user_domain" "userDomain"

printf "\n${bold}✓ All packages complete!${reset}\n"
printf "  Frameworks in: $output_dir\n"

