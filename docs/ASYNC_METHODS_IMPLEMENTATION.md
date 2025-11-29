# Async Methods Implementation Guide

## Current Implementation Status

Your `GetUsersUseCaseImpl` is **correctly implemented** and follows UniFFI best practices. The current structure works across all platforms with proper async support.

## Implementation Details

### Current Pattern: Async Methods on Objects ✅

```rust
#[derive(uniffi::Object)]
pub struct GetUsersUseCaseImpl {
    base: BaseUseCase,
    user_repository: Arc<dyn UserRepository>,
}

#[uniffi::export]
impl GetUsersUseCaseImpl {
    #[uniffi::constructor]
    pub fn new() -> Result<Arc<Self>, DomainError> { /* ... */ }

    pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> { /* ... */ }
    
    pub async fn execute_by_id(&self, id: u64) -> Result<UserDomainModel, DomainError> { /* ... */ }
}
```

### Why This Pattern Works

1. **Object-Oriented Design**: Maintains state, supports dependency injection
2. **UniFFI Compatible**: `#[derive(uniffi::Object)]` + `#[uniffi::export]` on async methods
3. **Error Handling**: `Result<T, E>` maps correctly to platform error handling
4. **Runtime Management**: `BaseUseCase` handles tokio runtime internally

## Platform-Specific Behavior

### iOS/Swift ✅

**Generated Code:**
```swift
open class GetUsersUseCaseImpl {
    func execute() async throws -> [UserDomainModel]
    func executeById(id: UInt64) async throws -> UserDomainModel
}
```

**Usage:**
```swift
let useCase = try GetUsersUseCaseImpl()
let users = try await useCase.execute()
let user = try await useCase.executeById(id: 123)
```

**Status:** ✅ Works perfectly, no issues

### Android/Kotlin ⚠️

**Generated Interface:**
```kotlin
interface GetUsersUseCaseImplInterface {
    suspend fun execute(): List<UserDomainModel>
    suspend fun executeById(id: ULong): UserDomainModel
}
```

**Generated Implementation:**
```kotlin
open class GetUsersUseCaseImpl : GetUsersUseCaseImplInterface {
    override suspend fun execute(): List<UserDomainModel> {
        // Proper implementation generated in UniFFI 0.30.0+
    }
}
```

**Usage:**
```kotlin
val useCase = GetUsersUseCaseImpl.new()
val users = useCase.execute() // suspend function
val user = useCase.executeById(123UL) // suspend function
```

**Status:** ✅ Fixed in UniFFI 0.30.0+ - Interface and implementation generated correctly

**Note:** Requires UniFFI 0.30.0 or later

### Python ✅

**Generated Code:**
```python
class GetUsersUseCaseImpl:
    async def execute(self) -> List[UserDomainModel]:
        # ...
    
    async def execute_by_id(self, id: int) -> UserDomainModel:
        # ...
```

**Usage:**
```python
use_case = GetUsersUseCaseImpl.new()
users = await use_case.execute()
user = await use_case.execute_by_id(123)
```

**Status:** ✅ Works perfectly, no issues

## Verification Checklist

After implementing async methods, verify:

### ✅ Code Structure
- [ ] Struct has `#[derive(uniffi::Object)]`
- [ ] Impl block has `#[uniffi::export]`
- [ ] Async methods are `pub async fn`
- [ ] Return type is `Result<T, E>`
- [ ] Constructor returns `Result<Arc<Self>, E>`

### ✅ Platform Generation
- [ ] iOS: Run `just package ios`, check Swift files
- [ ] Android: Run `just package android`, check Kotlin files
- [ ] Python: Run `just package python`, check Python files

### ✅ Platform Testing
- [ ] iOS: Test async methods in Xcode
- [ ] Android: Test suspend functions in Android Studio
- [ ] Python: Test async methods in Python script

## Best Practices

### 1. Error Types

Always use a custom error type that implements UniFFI traits:

```rust
#[derive(uniffi::Error)]
pub enum DomainError {
    // Variants
}
```

### 2. Return Types

Use `Result<T, E>` for all async methods:
```rust
pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError>
```

### 3. Object Lifecycle

Return `Arc<Self>` from constructors:
```rust
#[uniffi::constructor]
pub fn new() -> Result<Arc<Self>, DomainError> {
    Ok(Arc::new(Self { /* ... */ }))
}
```

### 4. Async Runtime

Use `BaseUseCase` pattern for runtime management:
- Internal tokio runtime
- Handles async execution
- Provides error conversion

## Troubleshooting

### Issue: Kotlin Implementation Not Generated

**Symptom:** "Sorry, the callable 'execute' isn't supported" in generated Kotlin

**Solution:**
1. **Upgrade to UniFFI 0.30.0+** - Bug is fixed!
2. Verify version in `Cargo.toml`: `uniffi = { version = "0.30.0" }`
3. Regenerate bindings: `just package android`

### Issue: Swift Compilation Errors

**Symptom:** "Cannot find type in scope" or async/await errors

**Solution:**
1. Ensure `#[uniffi::export]` is on the impl block
2. Check that async methods return `Result<T, E>`
3. Verify UniFFI version compatibility

### Issue: Python Import Errors

**Symptom:** "Module not found" or async method not available

**Solution:**
1. Regenerate bindings: `just package python`
2. Ensure Python 3.8+ with async support
3. Check that methods are properly exported

## Alternative Pattern: Standalone Functions

If you encounter persistent issues with object methods, you can use standalone functions:

```rust
#[uniffi::export]
pub async fn get_users(use_case: Arc<GetUsersUseCaseImpl>) -> Result<Vec<UserDomainModel>, DomainError> {
    use_case.execute().await
}
```

**Pros:**
- More reliable code generation
- Simpler API surface
- Better for stateless operations

**Cons:**
- Less object-oriented
- Need to pass object as parameter

## Migration Path

If you need to switch patterns:

1. **Add standalone functions** (keep object for now)
2. **Update platform code** to use functions
3. **Test on all platforms**
4. **Remove object methods** if desired

## Conclusion

Your current implementation is **correct and follows best practices**. With UniFFI 0.30.0+, the Kotlin async method bug is **fixed**:

1. ✅ **No workarounds needed** - Works out of the box
2. ✅ **All platforms supported** - iOS, Android, Python
3. ✅ **Clean implementation** - No manual fixes required

The pattern you're using is the **recommended approach** for object-oriented, stateful use cases across all platforms.

