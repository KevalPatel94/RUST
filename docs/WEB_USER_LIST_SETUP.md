# Web App User List Feature - Setup Guide

## Overview

The web app now includes a **User List** feature that uses a WASM-compatible implementation of `GetUsersUseCaseImpl`. This follows the same architecture patterns as Android and iOS but uses `wasm-bindgen` instead of UniFFI.

## Architecture

### WASM-Specific Implementation

Since `user_domain` is excluded from WASM builds (due to async runtime compatibility), we created a WASM-specific implementation:

- **Location**: `src/wasm.rs`
- **Class**: `WasmGetUsersUseCase`
- **Uses**: Browser's `fetch` API via `web-sys`
- **Returns**: `WasmUserDomainResultModel` (same pattern as native)

### Key Differences from Native

1. **No BaseUseCase**: Uses `wasm-bindgen-futures` for async operations
2. **Direct Fetch**: Uses browser's fetch API instead of `reqwest`
3. **wasm-bindgen**: Uses `wasm-bindgen` instead of UniFFI
4. **Same Patterns**: Still follows DomainResultModel pattern (Loaded/Empty/Error)

## Files Created/Modified

### Rust Files
- `src/wasm.rs` - Added `WasmGetUsersUseCase` and related types
- `Cargo.toml` - Added WASM dependencies (`wasm-bindgen-futures`, `web-sys`, `serde-wasm-bindgen`)

### React Files
- `src/App.tsx` - Updated to use React Router
- `src/DebugMenu.tsx` - New debug menu component
- `src/UserList.tsx` - New user list component
- `src/PasswordValidation.tsx` - Extracted from App.tsx
- `src/main.tsx` - Added BrowserRouter

### CSS Files
- `src/DebugMenu.css` - Debug menu styles
- `src/UserList.css` - User list styles
- `src/App.css` - Added back-link styles

## Setup Instructions

### 1. Install Dependencies

```bash
cd platforms/web/LockSmithExample
npm install
```

This will install `react-router-dom` and other dependencies.

### 2. Rebuild WASM Package

After making Rust changes, rebuild the WASM package:

```bash
# From repo root
just package web

# Or manually
cd platforms/web/LockSmithExample
cd ../..
wasm-pack build --target bundler --features js --release
mv pkg platforms/web/LockSmithExample/pkg
```

### 3. Run the App

```bash
cd platforms/web/LockSmithExample
npm run dev
```

## Features

### Debug Menu
- **Route**: `/`
- Shows menu with options:
  - LockSmith (Password Validation)
  - User List
  - Benchmark

### User List
- **Route**: `/users`
- Displays list of users from DummyJSON API
- Shows:
  - User avatar (with lazy loading)
  - Full name
  - Email (pre-formatted from Rust: "Email: user@example.com")
  - Age (pre-formatted from Rust: "X years old")
  - Phone number
- Handles three states:
  - **Loading**: Shows spinner
  - **Loaded**: Shows user cards
  - **Empty**: Shows empty state message
  - **Error**: Shows error message with retry button

### Password Validation
- **Route**: `/password`
- Same as before, now accessible via debug menu

### Benchmark
- **Route**: `/benchmark`
- Same as before, now accessible via debug menu

## Architecture Compliance

✅ **Follows `.cursorrules` patterns:**
- Uses DomainResultModel pattern (Loaded/Empty/Error)
- Pre-formats user-facing strings in Rust
- Uses repository-like pattern (fetch logic in Rust)
- Handles errors gracefully
- Clean separation of concerns

## TypeScript Types

The WASM bindings are typed in `UserList.tsx`:

```typescript
type WasmUserDomainModel = {
  id: number
  first_name: string
  last_name: string
  phone: string
  full_name: string
  image_url: string
  age_display: string  // Pre-formatted from Rust
  email_display: string  // Pre-formatted from Rust
}

type WasmUserDomainResultModel = 
  | { type: 'loaded'; data: WasmUserDomainModel[] }
  | { type: 'empty'; data: { title: string; subtitle: string; button_title: string } }
  | { type: 'error'; display: { title: string; subtitle: string } }
```

## Testing

1. **Start the dev server**: `npm run dev`
2. **Navigate to**: `http://localhost:5173`
3. **Click "User List"** from the debug menu
4. **Verify**:
   - Users load and display correctly
   - Images load (with fallback)
   - Pre-formatted strings display correctly
   - Empty/Error states work
   - Refresh button works

## Notes

- The WASM implementation uses the browser's fetch API, so CORS must be enabled on the API
- User avatars are loaded lazily with fallback to initials
- All user-facing strings are generated in Rust (following architecture rules)
- The implementation mirrors the Android/iOS pattern but uses WASM-compatible APIs

