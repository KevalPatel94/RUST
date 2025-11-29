# UniFFI Async Methods - Cross-Platform Patterns

This document outlines the proper ways to expose async methods with UniFFI that work across all platforms (Android, iOS, Python).

## Current Implementation Analysis

Your current `GetUsersUseCaseImpl` uses:
- `#[derive(uniffi::Object)]` on a struct
- Async methods in an `impl` block with `#[uniffi::export]`
- ✅ Works perfectly for iOS/Swift
- ⚠️ Has Kotlin generation issues (but can be fixed)

## Pattern 1: Async Methods on Objects (Current - Recommended) ✅

**Best for:** Object-oriented APIs, stateful use cases, dependency injection

```rust
#[derive(uniffi::Object)]
pub struct GetUsersUseCaseImpl {
    base: BaseUseCase,
    user_repository: Arc<dyn UserRepository>,
}

#[uniffi::export]
impl GetUsersUseCaseImpl {
    #[uniffi::constructor]
    pub fn new() -> Result<Arc<Self>, DomainError> {
        // ... initialization
    }

    /// Async method - works across all platforms
    pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
        // ... implementation
    }
}
```

**Platform Support:**
- ✅ iOS/Swift: Works perfectly, generates `async func execute() async throws -> [UserDomainModel]`
- ✅ Android/Kotlin: Works perfectly in UniFFI 0.30.0+, generates `suspend fun execute(): List<UserDomainModel>`
- ✅ Python: Works, generates async methods

**Pros:**
- Object-oriented, maintains state
- Good for dependency injection
- Clean API surface
- Works well with error handling
- No workarounds needed in UniFFI 0.30.0+

**Note:** Requires UniFFI 0.30.0 or later for full Android Kotlin support

## Pattern 2: Standalone Async Functions (Alternative)

**Best for:** Stateless operations, simpler APIs, avoiding object overhead

```rust
/// Standalone async function - simpler, more reliable
#[uniffi::export]
pub async fn get_users(use_case: Arc<GetUsersUseCaseImpl>) -> Result<Vec<UserDomainModel>, DomainError> {
    use_case.execute().await
}

/// Or create a new instance internally
#[uniffi::export]
pub async fn get_all_users() -> Result<Vec<UserDomainModel>, DomainError> {
    let use_case = GetUsersUseCaseImpl::new()?;
    use_case.execute().await
}
```

**Platform Support:**
- ✅ iOS/Swift: Works perfectly
- ✅ Android/Kotlin: More reliable generation
- ✅ Python: Works perfectly

**Pros:**
- More reliable code generation
- Simpler API
- No object lifecycle management
- Better for stateless operations

**Cons:**
- Less object-oriented
- Can't maintain state easily
- May need to pass objects as parameters

## Pattern 3: Trait-Based Interface (Advanced)

**Best for:** Multiple implementations, polymorphism, testing

```rust
#[uniffi::export]
pub trait GetUsersUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError>;
}

#[derive(uniffi::Object)]
pub struct GetUsersUseCaseImpl {
    // ... fields
}

#[uniffi::export]
impl GetUsersUseCase for GetUsersUseCaseImpl {
    async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
        // ... implementation
    }
}
```

**Platform Support:**
- ✅ iOS/Swift: Works
- ⚠️ Android/Kotlin: Interface generation works, implementation may need fixes
- ✅ Python: Works

**Pros:**
- Polymorphism support
- Multiple implementations
- Better for testing (mockable)

**Cons:**
- More complex
- Similar Kotlin issues as Pattern 1

## Recommended Approach for Your Use Case

### Option A: Keep Current Pattern (Recommended) ✅

**Why:**
- Your current design is good (object-oriented, maintains state)
- Works perfectly for iOS
- Works perfectly for Android in UniFFI 0.30.0+
- Maintains clean API

**Implementation:**
1. Keep your current `GetUsersUseCaseImpl` structure
2. Use UniFFI 0.30.0+ (bug is fixed!)
3. Document the pattern for team

**Pros:**
- No code changes needed
- Maintains current architecture
- No workarounds needed
- Works on all platforms

### Option B: Hybrid Approach (Alternative - Not Needed)

**Why:**
- Provides both object-based and function-based APIs
- Function-based API is more reliable
- Object-based API for stateful operations

**Implementation:**
```rust
// Keep your object (for stateful operations)
#[derive(uniffi::Object)]
pub struct GetUsersUseCaseImpl { /* ... */ }

#[uniffi::export]
impl GetUsersUseCaseImpl {
    // Keep async methods for iOS/Python
    pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
        // ... implementation
    }
}

// Add standalone functions for Android (more reliable)
#[uniffi::export]
pub async fn get_users(use_case: Arc<GetUsersUseCaseImpl>) -> Result<Vec<UserDomainModel>, DomainError> {
    use_case.execute().await
}

#[uniffi::export]
pub async fn get_user_by_id(use_case: Arc<GetUsersUseCaseImpl>, id: u64) -> Result<UserDomainModel, DomainError> {
    use_case.execute_by_id(id).await
}
```

**Platform Usage:**
- iOS: Use object methods directly
- Android: Use standalone functions (more reliable)
- Python: Either works

## Best Practices

### 1. Error Handling

Always use `Result<T, E>` for async methods:
```rust
pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError>
```

This maps to:
- Swift: `async throws -> [UserDomainModel]`
- Kotlin: `suspend fun execute(): List<UserDomainModel>` (throws DomainException)
- Python: `async def execute() -> List[UserDomainModel]` (raises DomainError)

### 2. Object Lifecycle

Use `Arc<Self>` for objects passed across FFI:
```rust
#[uniffi::constructor]
pub fn new() -> Result<Arc<Self>, DomainError> {
    Ok(Arc::new(Self { /* ... */ }))
}
```

### 3. Async Runtime

Your `BaseUseCase` pattern is excellent:
- Manages tokio runtime internally
- Handles async execution
- Provides error conversion

### 4. Type Safety

Use concrete types, not trait objects in FFI:
```rust
// ✅ Good - concrete type
pub struct GetUsersUseCaseImpl { /* ... */ }

// ⚠️ Avoid - trait objects in FFI
pub struct UseCase(Box<dyn SomeTrait>);
```

## Platform-Specific Considerations

### iOS/Swift
- ✅ Async methods work perfectly
- ✅ Error handling via `throws`
- ✅ Object lifecycle managed automatically

### Android/Kotlin
- ✅ Interface generation works
- ✅ Implementation generation works in UniFFI 0.30.0+
- ✅ No workarounds needed
- ✅ Works out of the box

### Python
- ✅ Async methods work perfectly
- ✅ Uses Python's `async/await`
- ✅ Error handling via exceptions

## Migration Guide

If you want to switch from Pattern 1 to Pattern 2:

1. **Add standalone functions:**
```rust
#[uniffi::export]
pub async fn get_users(use_case: Arc<GetUsersUseCaseImpl>) -> Result<Vec<UserDomainModel>, DomainError> {
    use_case.execute().await
}
```

2. **Update platform code:**
   - iOS: Can keep using object methods
   - Android: Switch to standalone functions
   - Python: Either works

3. **Keep object for stateful operations:**
   - Keep `GetUsersUseCaseImpl` for initialization
   - Use standalone functions for operations

## Verification

After implementing, verify on each platform:

```bash
# iOS
just package ios
# Check generated Swift - should have async methods

# Android  
just package android
# Check generated Kotlin - should have suspend functions
# Verify no "Sorry, the callable" comments

# Python
just package python
# Check generated Python - should have async methods
```

## Conclusion

**For your current architecture, I recommend:**

1. **Keep Pattern 1** (async methods on objects) - it's well-designed
2. **Use UniFFI 0.30.0+** - Bug is fixed!
3. **Document the pattern** for your team
4. **No workarounds needed** - Works out of the box

This gives you:
- ✅ Clean, object-oriented API
- ✅ Works on all platforms
- ✅ Maintainable and scalable
- ✅ No workarounds or fixes needed

