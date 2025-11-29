set export

# The display/package name used by scripts
LOCKSMITH_DISPLAY_NAME := "locksmith"

# The location where the completed Android library will be placed.
LOCKSMITH_ANDROID_OUT := justfile_directory() + "/platforms/android/lib/src/main/jniLibs"

# The location where the completed iOS framework will be placed.
LOCKSMITH_IOS_OUT := justfile_directory() + "/platforms/ios/Frameworks"

# The location where the generated Python package/artifacts will be placed.
LOCKSMITH_PYTHON_OUT := justfile_directory() + "/platforms/python/LockSmithExample"

# Lists the available recipes, omitting itself from the list.
default:
  @just --color always --justfile {{justfile()}} --list |  grep -E -v "^\s+ default "

# # Removes intermediate build files and binaries
# clean: reset
#   cargo clean

# # Removes application data files
# reset:
#   rm -rf {{dev-project-dir}}

# Builds the release version of the library
release:
  cargo build --lib --release

# Packages the library and moves it into place.
[no-exit-message]
package platform: release
  #!/usr/bin/env bash

  case '{{platform}}' in
    And | Android | Droid | Google | and | android | droid | google)
      ./scripts/package_android.sh
      ;;
    Apple | IOS | apple | iOS | ios)
      ./scripts/package_ios_separate.sh
      ;;
    Py | Python | py | python)
      ./scripts/package_python.sh
      ;;
    Web | WEB | web)
      ./scripts/package_web.sh
      ;;
    *)
      printf "{{RED}}error:{{NORMAL}} "
      printf "{{BOLD}}platform “"{{platform}}"” not in list of supported platforms: "
      printf "android, ios, python, web\n{{NORMAL}}"
      exit 1
      ;;
  esac

# Lints the project using Cargo's Clippy tool
lint:
  cargo clippy

# Run the project's tests
test: 
  cargo test

# Prints a curated project tree
tree:
  tree --gitignore -I Cargo.toml

# Runs the web demo (requires prior `just package web`)
web:
  npm --prefix ./platforms/web/LockSmithExample run dev

web-test:
  cd platforms/web/LockSmithExample && npm install && npm run benchmark

python:
  cd platforms/python && PYTHONPATH="$(pwd)" python3 -m LockSmithExample.demo --locale es

python-test:
  cd platforms/python && PYTHONPATH="$(pwd)" python3 -m LockSmithExample.benchmark_password_validator --rounds 10000 --locale en-US

android-test:
  cd platforms/android && ./gradlew :app:connectedDebugAndroidTest

ios:
  killall Xcode 2>/dev/null || true
  sleep 1
  cd platforms/ios && mint run xcodegen generate && open Example.xcodeproj