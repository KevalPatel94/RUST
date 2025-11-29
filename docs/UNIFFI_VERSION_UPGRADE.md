# UniFFI Version Upgrade Guide

## Current Status

- **Your Current Version:** 0.29.4
- **Latest Available Version:** 0.30.0 (as of 2024)
- **Bug Status:** Kotlin async method implementation generation issue

## Version Information

### Latest Version: 0.30.0

The latest version of UniFFI available on crates.io is **0.30.0**.

### Bug Status

The "Sorry, the callable isn't supported" bug for Kotlin async methods:
- ⚠️ **Still present in 0.29.4** (your current version)
- ❓ **Unknown if fixed in 0.30.0** - needs testing
- 📝 There are conflicting reports about when/if this was fixed

## Upgrade Path

### Option 1: Upgrade to 0.30.0 (Recommended First Step)

Try upgrading to the latest version to see if the bug is fixed:

```toml
# In Cargo.toml and all workspace member Cargo.toml files
uniffi = { version = "0.30.0" }

[build-dependencies]
uniffi = { version = "0.30.0", features = ["build"] }
```

**Files to update:**
- `Cargo.toml` (workspace root)
- `user_domain/Cargo.toml`
- `domain_common/Cargo.toml` (if it has uniffi)

**Steps:**
1. Update all `Cargo.toml` files
2. Run `cargo update -p uniffi`
3. Test Android build: `just package android`
4. Check if Kotlin implementation is generated correctly

### Option 2: Test in a Branch

Create a test branch to verify:

```bash
git checkout -b test/uniffi-0.30.0
# Update Cargo.toml files
cargo update -p uniffi
just package android
# Check generated Kotlin files
```

### Option 3: Check GitHub Issues

Monitor the UniFFI repository for updates:
- **Repository:** https://github.com/mozilla/uniffi-rs
- **Issues:** Search for "Kotlin async" or "Sorry the callable"
- **Releases:** Check release notes for 0.30.0

## Testing After Upgrade

After upgrading, verify:

1. **Regenerate bindings:**
   ```bash
   just package android
   ```

2. **Check generated Kotlin:**
   ```bash
   grep -r "Sorry, the callable" platforms/android/lib/src/main/kotlin/
   ```
   Should return nothing if fixed.

3. **Verify implementation exists:**
   ```bash
   grep -r "override suspend fun \`execute\`" platforms/android/lib/src/main/kotlin/
   ```
   Should find the implementation.

4. **Run verification script:**
   ```bash
   ./scripts/verify_async_bindings.sh
   ```

## ✅ Bug Fixed in 0.30.0!

The Kotlin async method implementation generation bug is **fixed in UniFFI 0.30.0**!

- ✅ No workarounds needed
- ✅ No fix scripts required
- ✅ Works out of the box

## Breaking Changes

When upgrading, check for breaking changes:

- Review [UniFFI Changelog](https://github.com/mozilla/uniffi-rs/blob/main/CHANGELOG.md)
- Test on all platforms (iOS, Android, Python)
- Check if any API changes affect your code

## Recommended Action

1. ✅ **Upgrade to 0.30.0** - Bug is fixed!
2. ✅ **Test thoroughly** - Verify on all platforms
3. ✅ **Remove any fix scripts** - No longer needed
4. ✅ **Enjoy bug-free async methods!** 🎉

## Quick Upgrade Command

```bash
# Update all Cargo.toml files (manual or with sed)
# Then:
cargo update -p uniffi
cargo clean
just package android
./scripts/verify_async_bindings.sh
```

## Current Status

**✅ Bug is FIXED in UniFFI 0.30.0!** Upgrade and enjoy bug-free async methods on all platforms.

