# Out-of-the-Box Configuration

This project is configured to work on any machine without additional setup. All network configurations are pre-configured for Android, iOS, and Web platforms.

## ✅ Pre-Configured Settings

### 1. **iOS App Transport Security (ATS)**
- ✅ Configured in `Info.plist` to allow HTTPS connections to `dummyjson.com`
- ✅ Maintains security by only allowing specific domains
- ✅ No manual configuration needed

### 2. **Android Network Security**
- ✅ Internet permission already in `AndroidManifest.xml`
- ✅ Network security config added for HTTPS support
- ✅ Allows secure connections to `dummyjson.com`
- ✅ No manual configuration needed

### 3. **Network Client Configuration**
- ✅ Uses `rustls-tls` (pure Rust, no OpenSSL dependencies)
- ✅ Works across all platforms (Android, iOS, Web, Python)
- ✅ Configured with reasonable timeouts (30 seconds)
- ✅ Connection pooling enabled for better performance
- ✅ User agent set for compatibility

### 4. **API Endpoint**
- ✅ Defaults to `https://dummyjson.com` (public API, no authentication needed)
- ✅ Can be overridden via `USER_API_BASE_URL` environment variable if needed
- ✅ Works out of the box without any setup

### 5. **Error Handling**
- ✅ Improved error detection for "error sending request" errors
- ✅ Better error messages for users
- ✅ Detailed logging for debugging

## 🚀 Quick Start

### For Android:
```bash
just package android
# Run on device/emulator - network will work automatically
```

### For iOS:
```bash
just package ios
# Open in Xcode and run - network will work automatically
```

### For Web:
```bash
just package web
just web
# Network requests work in browser automatically
```

## 🔧 Optional: Custom API Endpoint

If you need to use a different API endpoint (not required):

```bash
export USER_API_BASE_URL="https://your-api.com"
# Then rebuild
just package android  # or ios, web, etc.
```

## 📋 What's Configured

### iOS (`Info.plist`)
- App Transport Security allows HTTPS to `dummyjson.com`
- Maintains security for all other domains
- No cleartext HTTP allowed (secure by default)

### Android (`AndroidManifest.xml` + `network_security_config.xml`)
- Internet permission enabled
- Network security config allows HTTPS to `dummyjson.com`
- System certificates trusted
- No cleartext HTTP allowed (secure by default)

### Network Client (`network/src/client.rs`)
- 30-second timeout (reasonable for mobile networks)
- Connection pooling (90-second idle timeout)
- User agent: "LockSmith/1.0"
- Uses rustls-tls (no native dependencies)

### Error Handling (`network/src/error.rs`)
- Detects "error sending request" errors
- Classifies connection errors properly
- Provides helpful error messages

## 🐛 Troubleshooting

### If network still doesn't work:

1. **Check device/emulator has internet:**
   - Open browser and visit `https://dummyjson.com/users`
   - Should return JSON data

2. **Check console/logs:**
   - Look for detailed error messages
   - Error messages now include specific failure reasons

3. **Verify time is correct:**
   - Incorrect device time can cause SSL certificate validation to fail
   - Check device date/time settings

4. **Try different network:**
   - Some corporate networks block external APIs
   - Try mobile hotspot or different WiFi

## 📝 Files Modified for Out-of-the-Box Setup

1. `platforms/ios/Sources/LockSmithExample/Resources/Info.plist`
   - Added App Transport Security configuration

2. `platforms/android/app/src/main/AndroidManifest.xml`
   - Added network security config reference

3. `platforms/android/app/src/main/res/xml/network_security_config.xml`
   - Created network security configuration

4. `network/src/client.rs`
   - Enhanced client configuration with timeouts and pooling

5. `network/src/error.rs`
   - Improved error detection and classification

6. `user_data/src/user_repository.rs`
   - Made base URL configurable (with good default)

7. `user_domain/src/get_users_use_case.rs`
   - Better error messages for users

## ✨ Benefits

- **No setup required**: Works immediately after cloning
- **Cross-platform**: Same configuration works on Android, iOS, Web
- **Secure by default**: HTTPS only, proper certificate validation
- **Robust error handling**: Better error messages help debug issues
- **Flexible**: Can override API endpoint if needed via environment variable

## 🎯 Summary

The project is now configured to work out of the box on any machine. Simply:
1. Clone the repository
2. Run `just package <platform>`
3. Run the app
4. Network requests will work automatically!

No manual configuration, environment setup, or network settings needed. Everything is pre-configured and ready to go.

