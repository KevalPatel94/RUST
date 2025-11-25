## LockSmith

Cross-platform sample with a Rust core and bindings for Android, iOS, and Python (via UniFFI) and Web (via WASM).

### Prerequisites
- Rust toolchain: `rustup` with latest stable (`rustup update`)
- just (optional but recommended): `cargo install just` or via package manager
- Android:
  - Android Studio, SDK, and NDK installed
  - Java 17+ (Android Gradle Plugin requirement)
  - `cargo-ndk`: `cargo install cargo-ndk`
- iOS: Xcode 15+ with command line tools
- Python: Python 3.8+ and `pip`

## Quick start: run and test with just

Use these convenient commands for each platform:

```bash
# Android
just package android
just android-test            # runs connected tests (emulator/device required)

# iOS
just package ios
just ios                     # opens the Xcode project

# Web
just package web
just web                     # starts Vite dev server
just web-test                # runs benchmark tests headlessly

# Python
just package python
just python                  # runs the demo
just python-test             # runs the benchmark
```

Notes:
- Packaging is required before first run on Web/Python to generate artifacts and bindings.
- Android tests require a running emulator or a connected device.

## Architecture

This project uses a single Rust core and exposes it to each platform via FFI (Android/iOS/Python) or WebAssembly (Web). The diagram below compares where Rust and Kotlin Multiplatform (KM) primarily operate:

Note on comparison: Kotlin Multiplatform (KM) is a widely adopted cross‑platform approach that promotes sharing business logic while keeping fully native UIs on Android and iOS. Because it addresses a similar need—single source of truth for core logic across native platforms—we use KM as a relevant baseline when illustrating Rust’s role, portability, and performance characteristics in this project.

![Rust vs KM architecture](docs/KM_RUST_Plateform_Support.svg)

#### Where each plays its role
- Rust: shared core/system layer behind FFI/WASM; one implementation reused across Android, iOS, Web, and Python.
- KM: shared language-level module primarily across Android and iOS; Web/Python usually require separate implementations.

#### Why we use Rust for the core in this project
- Broad reuse: one core across mobile, web, and Python.
- Strong performance and safety: native code with Rust’s ownership model and no GC pauses.
- Consistency: identical behavior across platforms can reduce divergence bugs.
- Note: KM remains a solid choice for many teams, especially when prioritizing shared mobile logic with native UIs.

## Performance and compile layers

How each compiles and runs can influence latency and throughput:

- Rust: AOT to native with a minimal runtime and no GC, which supports predictable performance.
- Kotlin on Android: runs on ART with JIT/AOT and GC, which can introduce warm‑up and GC pauses depending on workload.
- Kotlin/Native on iOS: AOT native with a managed runtime and GC, which add overhead relative to Rust in some scenarios.

The following diagram illustrates typical additional runtime layers present in KM-based stacks relative to a minimal Rust core. Actual performance depends on workload characteristics and implementation details.

![KM overhead layers vs Rust](docs/KM_RUST_Operating_Architecture.svg)

Typical sources of overhead in KM-based stacks:
- Managed runtime features (e.g., class loading, verification, exceptions).
- JIT warm‑up and tiered compilation on Android; GC activity on both Android and iOS.
- Interop/bridging and reflection (JNI, Swift/Obj‑C bridges), and incidental boxing/indirections.

#### Definitions
- ART (Android Runtime): Android’s managed runtime that executes DEX bytecode and handles class loading, verification, JIT/AOT, and GC.
- GC (Garbage Collection): Automatic memory reclamation; reduces manual management but can introduce pauses and barriers.
- JIT (Just‑In‑Time compilation): Compiles hot code at runtime using profiling; improves steady‑state but adds warm‑up and potential pauses.
- AOT (Ahead‑Of‑Time compilation): Compiles to native machine code before execution; predictable startup/latency, fewer runtime specializations.

## Rust

Build, lint, and test:
```bash
just release
just lint
just test
```

Run the example:
```bash
cargo run --example locksmith_example
```

---

## Android

Package native libs and generate Kotlin bindings:
```bash
just package android
```
This places `.so` files in `platforms/android/lib/src/main/jniLibs/` and updates generated Kotlin in `platforms/android/lib/src/main/kotlin/`.

Quick run/tests from the root:
```bash
just package android
just android-test
```

Run the demo app (Android Studio):
1. Open `platforms/android` in Android Studio.
2. Wait for Gradle sync to complete without errors.
3. From the run menu, choose the `LockSmithExample` app configuration.
4. Select a device (emulator or hardware) and click Run.

Run from command line (optional):
```bash
cd platforms/android
./gradlew :app:installDebug
./gradlew :app:connectedDebugAndroidTest   # optional UI tests
```

For details and troubleshooting, see `platforms/android/README.md`.

---

## iOS

Package the XCFramework and generated Swift:
```bash
just package ios
```
This updates `platforms/ios/Frameworks/locksmithFFI.xcframework` and generated Swift sources under `platforms/ios/Sources/`.

Quick open in Xcode from the root:
```bash
just package ios
just ios
```

Run from command line (optional, Simulator example):
```bash
cd platforms/ios
mint run xcodegen 
```

Run the demo app (Xcode):
1. Open `platforms/ios/Example.xcodeproj` in Xcode.
2. Select the `LockSmithExample` scheme.
3. Choose a Simulator (e.g., iPhone 15) or a connected device.
4. Press Run (Cmd+R).

### Features

The iOS demo app includes:
- **Localization**: Switch between English, Spanish, and French
- **Password Validation**: Real-time password validation with localized error messages
- **Demo Values**: Pre-filled passwords to test different validation scenarios
- **Benchmark**: Compare Swift vs Rust performance
  - Tap "Run Benchmark" to compare password validation speed
  - Tests 10,000 rounds of validation across 7 different passwords
  - Shows per-operation timing in nanoseconds and speedup multiplier


---

## Web

Run the web demo that uses the Rust core compiled to WebAssembly.

### Prerequisites

- Node.js 18+ and npm
- wasm-pack: `cargo install wasm-pack`
- Rust target: `rustup target add wasm32-unknown-unknown`

### Build the WASM package

```bash
just package web
```

This generates the JS/WASM artifacts in `platforms/web/pkg/`.

### Start the web demo

```bash
# Option A: from repo root
just web

# Option B: from the app folder
cd platforms/web/LockSmithExample
npm i            # first time or after rebuilding pkg
npm run dev
```

Open the URL Vite prints (e.g., `http://localhost:5173`).

Quick run/tests from the root:
```bash
just package web
just web
just web-test
```

### Features

The web demo includes:
- **Localization**: Switch between English, Spanish, and French
- **Password Validation**: Real-time validation with localized error messages
- **Demo Values**: Pre-filled passwords to test validation
- **Interactive Benchmark**: Compare JavaScript vs Rust (WASM) performance

### Running Tests

Run benchmarks in a controlled test environment for accurate performance measurements:

```bash
cd platforms/web/LockSmithExample

# Install dependencies (first time)
npm install

# Run benchmark test (RECOMMENDED for accurate results)
npm run benchmark

# Run all tests
npm test

# Watch mode (auto-runs on changes)
npm run test:watch
```

### Notes

- The app depends on `locksmith` from `file:../pkg`. Re-run `just package web` after any Rust changes.
- We disabled wasm-opt in `Cargo.toml`. For extra WASM optimizations, install Binaryen (`brew install binaryen`) and enable wasm-opt in the wasm-pack metadata.
- For accurate benchmarks, use `npm run benchmark` instead of the UI button.

---

## Python

Build/generate Python bindings and copy the shared library:

Generate Python bindings
```bash
just package python
```

Artifacts are placed in `platforms/python/LockSmithExample/`:
- `locksmith.py` (generated UniFFI bindings)
- `liblocksmith.*` (compiled shared library; `.dylib` on macOS)

Quick run/tests from the root:
```bash
just package python
just python
just python-test
```

Run the demo without installing:
```bash
cd /Users/kevapatel/Developer/paypal/LockSmith/platforms/python
PYTHONPATH="$(pwd)" 
python3 -m LockSmithExample.demo --locale es
```

Run Python benchmarks (no install):
```bash
cd /Users/kevapatel/Developer/paypal/LockSmith/platforms/python
PYTHONPATH="$(pwd)" 
python3 -m LockSmithExample.benchmark_password_validator --rounds 10000 --locale en-US
```
Notes:
- The benchmark compares a pure-Python loop against the Rust batch API `validate_passwords_score_repeated` called via FFI.
- Increase `--rounds` for more stable measurements; `--locale` controls localized messages used by the Rust validator.

Notes:
- The packaging step must run first so `locksmith.py` and `liblocksmith.*` exist next to `LockSmithExample/`.
- Locale can be selected with `--locale <code>` or `MY_SAMPLE_LOCALE=<code>` (defaults to `en-US`). Available locales are under `src/locales/` (e.g., `en-US`, `es`, `fr`).

---

## Common issues
- Library load errors in Python or Android:
  - Re-run the corresponding `just package <platform>` to regenerate and place artifacts next to the bindings.
- Missing Android Rust targets:
  - The Android packaging script will prompt-install Rust targets; you can also add them manually with `rustup target add <triple>`.
- Xcode build failures:
  - Ensure you opened the provided project and are using the correct scheme and a supported Xcode version.


