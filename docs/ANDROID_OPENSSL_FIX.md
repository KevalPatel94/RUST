# Android OpenSSL Cross-Compilation Fix

## Problem

When building for Android targets (e.g., `aarch64-linux-android`), you may encounter this error:

```
error: failed to run custom build command for `openssl-sys v0.9.111`
Could not find directory of OpenSSL installation
```

## Root Cause

The `network` crate was using `native-tls` feature of `reqwest`, which depends on OpenSSL. When cross-compiling for Android:

- OpenSSL needs to be compiled for the Android target architecture
- The build system can't find OpenSSL libraries for Android
- Cross-compilation setup for OpenSSL is complex and error-prone

## Solution

Switch from `native-tls` to `rustls-tls`:

**Before (❌ causes OpenSSL dependency):**
```toml
reqwest = { version = "0.12", default-features = false, features = ["native-tls", "json"] }
```

**After (✅ pure Rust, no OpenSSL):**
```toml
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
```

## Why rustls-tls?

1. **Pure Rust** - No C dependencies, no cross-compilation issues
2. **Cross-platform** - Works seamlessly for Android, iOS, and other targets
3. **Modern** - Uses modern TLS 1.3, actively maintained
4. **Smaller** - Can result in smaller binaries
5. **Secure** - Well-audited Rust implementation

## Platform Compatibility

### ✅ Android
- **Works perfectly** - Pure Rust, no OpenSSL needed
- No cross-compilation setup required
- This was the main issue we fixed

### ✅ iOS  
- **Works perfectly** - Pure Rust, no platform-specific dependencies
- No additional configuration needed
- Works for both device and simulator targets

### ✅ Python (via UniFFI)
- **Works perfectly** - Pure Rust implementation
- No system OpenSSL required
- Works on all platforms Python runs on (Linux, macOS, Windows)

### ⚠️ Web/WASM
- **Requires special handling** - WASM uses browser's fetch API, not rustls
- For WASM builds, you may need to use `reqwest` with `wasm` feature instead
- However, if your network crate isn't used in WASM builds, this won't affect you
- **Note**: Your project uses conditional compilation (`#[cfg(all(target_arch = "wasm32", feature = "js"))]`), so the network crate may not even be included in WASM builds

## Benefits

- ✅ No OpenSSL installation needed
- ✅ Works out-of-the-box for Android cross-compilation
- ✅ Works for iOS builds (device + simulator)
- ✅ Works for Python bindings on all platforms
- ✅ Simpler build process
- ✅ Better for mobile app size

## Trade-offs

- ⚠️ Slightly different TLS implementation (but fully compatible)
- ⚠️ May have different performance characteristics (usually negligible)
- ⚠️ For WASM, you'd need browser fetch API (but your network crate may not be used in WASM)

## Verification

After making this change, rebuild:

```bash
# Clean previous build artifacts
cargo clean -p network

# Try building for Android
just package android
```

The build should now succeed without OpenSSL errors.

## Related

- See `Cargo.toml` workspace dependencies - already configured for `rustls-tls`
- This fix aligns `network/Cargo.toml` with workspace best practices

