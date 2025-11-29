# LockSmith Architecture

## 🎯 Key Principles (TL;DR)

1. **1:1:1 Relationship**: Screen → ViewModel → UseCase (strict, never share)
2. **Reusable Logic → Repository**: All API calls, HTTP, retry, caching go in Repository
3. **UseCases are Thin**: Just orchestrate, delegate to Repository, screen-specific validation only
4. **Clear Dependencies**: Platform → Domain → Data → Network (one direction only)

## Core Principles

### 1. One-to-One Relationship: Screen → ViewModel → UseCase

**CRITICAL RULE:** Each screen in the application follows a strict 1:1:1 relationship:
- **1 Screen** = **1 ViewModel** = **1 UseCase**

This ensures:
- Clear ownership and responsibility
- No shared state between screens
- Easy to reason about and test
- Simple dependency flow
- Each screen has its own dedicated business logic

**Example:**
```
UserListScreen → UserListViewModel → GetUsersUseCase
UserDetailScreen → UserDetailViewModel → GetUserDetailUseCase
UserSearchScreen → UserSearchViewModel → SearchUsersUseCase
```

**Important:**
- ❌ **NEVER** share a UseCase across multiple screens
- ❌ **NEVER** have multiple UseCases for one screen
- ✅ Each screen gets its own dedicated UseCase
- ✅ Each ViewModel uses exactly one UseCase

### 2. All Reusable Logic MUST Live in Repository

**CRITICAL RULE:** All shared/reusable logic **MUST** be placed in the **Data Layer** (specifically in `RepositoryImpl`):
- ✅ API calls
- ✅ Data fetching
- ✅ Data transformation (raw API → Data Models)
- ✅ Caching strategies
- ✅ Network retry logic
- ✅ Common data operations
- ✅ HTTP request/response handling
- ✅ Error handling for network/data operations

**Why?**
- Single source of truth for data operations
- All UseCases can reuse the same logic
- Easy to test and mock
- Changes to data sources only affect one place
- Prevents code duplication across UseCases

**What Goes in Repository:**
```rust
// ✅ GOOD: All API logic in Repository
impl UserRepositoryImpl {
    async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
        // HTTP request logic
        // JSON parsing
        // Error handling
        // Retry logic
        // Caching
    }
}
```

**What Does NOT Go in Repository:**
```rust
// ❌ BAD: Business logic in Repository
impl UserRepositoryImpl {
    async fn get_users(&self) {
        if user_id == 0 { // ❌ Business rule - belongs in UseCase
            return Err(...);
        }
    }
}
```

## Layer Responsibilities

### Data Layer (`user_data` crate)
**Purpose:** Handle all data operations and API interactions

**Contains:**
- `UserRepository` trait - defines data operations contract
- `UserRepositoryImpl` - concrete implementation with ALL common logic
- `UserDataModel` - data transfer objects (DTOs) matching API structure
- Error handling for data operations

**Responsibilities:**
- ✅ Making HTTP requests
- ✅ Parsing JSON responses
- ✅ Handling network errors
- ✅ Data validation (API response structure)
- ✅ Retry logic
- ✅ Caching (if needed)

**Should NOT contain:**
- ❌ Business logic
- ❌ Domain model transformations
- ❌ Screen-specific logic

### Domain Layer (`user_domain` crate)
**Purpose:** Screen-specific business logic and domain models

**Contains:**
- `GetUsersUseCaseImpl` - thin orchestrator for UserListScreen (1:1 with ViewModel)
- `UserListDomainModel` - pure domain entities (no serialization concerns)
- `UserDomainModel` - UniFFI-exposed record for platforms
- `UserDataToDomainMapper` - converts Data → Domain models
- `ErrorDisplay` - error configuration for platforms

**Responsibilities:**
- ✅ **Screen-specific business logic validation** (e.g., "ID must be > 0" for this screen)
- ✅ **Orchestrating repository calls** (delegate, don't implement)
- ✅ **Mapping Data models → Domain models** (screen-specific transformation)
- ✅ **Error transformation** (Data errors → Domain errors)
- ✅ **Screen-specific orchestration** (combining multiple repository calls if needed)

**Should NOT contain:**
- ❌ **API call logic** (MUST delegate to Repository)
- ❌ **Reusable data operations** (MUST be in Repository)
- ❌ **Network retry logic** (MUST be in Repository)
- ❌ **Caching logic** (MUST be in Repository)
- ❌ **Screen-specific presentation logic** (belongs in ViewModel)
- ❌ **Platform-specific code** (belongs in Platform layer)

**UseCase Pattern:**
```rust
// ✅ GOOD: Thin UseCase - just orchestrates
pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
    // 1. Screen-specific business validation (if any)
    // 2. Delegate to Repository (reusable logic)
    let data = self.repository.get_users().await?;
    // 3. Map to Domain model
    Ok(mapper.map(&data))
}
```

### Presentation Layer (Platform: iOS/Android)
**Purpose:** UI and user interaction

**Contains:**
- ViewModels (one per screen, one per UseCase)
- Views/ViewControllers
- UI state management

**Responsibilities:**
- ✅ **UI state management** (loading, loaded, error, empty)
- ✅ **User interaction handling** (button clicks, navigation)
- ✅ **Calling UseCase methods** (1:1 relationship)
- ✅ **Transforming Domain models → Presentation models** (if needed)
- ✅ **UI-specific logic** (formatting, display, animations)

**Should NOT contain:**
- ❌ **Business logic** (MUST delegate to UseCase)
- ❌ **Data fetching logic** (MUST delegate to UseCase)
- ❌ **API call logic** (MUST delegate to UseCase → Repository)
- ❌ **Reusable data operations** (MUST be in Repository)

**ViewModel Pattern:**
```swift
// ✅ GOOD: ViewModel uses one UseCase
class UserListViewModel {
    private let useCase: GetUsersUseCaseImpl // 1:1 relationship
    
    func loadUsers() async {
        // Call UseCase, handle UI state
        do {
            let users = try await useCase.execute()
            // Update UI state
        } catch {
            // Handle error for UI
        }
    }
}
```

## Dependency Flow

```
Platform (iOS/Android)
    ↓ uses
Domain Layer (UseCase)
    ↓ uses
Data Layer (Repository)
    ↓ uses
Network Layer (HTTPClient)
```

**Key Rules:**
- ✅ Platform can only depend on Domain
- ✅ Domain can only depend on Data
- ✅ Data can only depend on Network
- ❌ No reverse dependencies
- ❌ No cross-layer dependencies

## Adding a New Feature

### Example: Adding a "User Detail" Screen

**Step 1: Data Layer** (`user_data/src/user_repository.rs`)
```rust
// ✅ Check if method already exists in Repository (reusable logic)
// If it exists, reuse it! If not, add it here.

#[async_trait]
pub trait UserRepository: Send + Sync {
    // This already exists - reuse it!
    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError>;
}

// Implementation already in UserRepositoryImpl - all API logic here
impl UserRepository for UserRepositoryImpl {
    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError> {
        // ✅ All reusable logic: HTTP request, parsing, error handling
        self.helper.get(&format!("users/{}", id)).await
    }
}
```

**Step 2: Domain Layer** (`user_domain/src/get_user_detail_use_case.rs`)
```rust
// ✅ New UseCase - 1:1 with UserDetailViewModel
#[derive(uniffi::Object)]
pub struct GetUserDetailUseCaseImpl {
    base: BaseUseCase,
    user_repository: Arc<dyn UserRepository>, // Reuse existing repository
}

#[uniffi::export]
impl GetUserDetailUseCaseImpl {
    #[uniffi::constructor]
    pub fn new() -> Result<Arc<Self>, DomainError> {
        // Initialize with BaseUseCase and Repository
    }

    // ✅ Thin orchestrator - delegates to Repository
    pub async fn execute(&self, id: u64) -> Result<UserDomainModel, DomainError> {
        // 1. Screen-specific business validation
        if id == 0 {
            return Err(DomainError::Error {
                display: ErrorDisplay::business_logic("Invalid user ID".to_string()),
            });
        }
        
        // 2. Delegate to Repository (reusable logic)
        let user_data = self.user_repository
            .get_user_by_id(id) // ✅ Reuses Repository logic
            .await
            .map_err(|_| ErrorDisplay::repository())?;
        
        // 3. Map to domain model
        Ok(UserDomainModel::from(UserDataToDomainMapper::map(&user_data)))
    }
}
```

**Step 3: Platform Layer** (iOS)
```swift
// ✅ New ViewModel - 1:1 with UserDetailScreen and GetUserDetailUseCase
class UserDetailViewModel {
    private let useCase: GetUserDetailUseCaseImpl // 1:1 relationship
    
    func loadUser(id: UInt64) async {
        do {
            // Call UseCase (delegates to Repository)
            let user = try await useCase.execute(id: id)
            // Update UI state
        } catch {
            // Handle error for UI
        }
    }
}

// ✅ New ViewController - 1:1 with ViewModel
class UserDetailViewController {
    private let viewModel: UserDetailViewModel // 1:1 relationship
    // UI implementation
}
```

**Key Points:**
- ✅ Repository method already exists - **reuse it!**
- ✅ New UseCase is thin - just orchestrates
- ✅ New ViewModel uses exactly one UseCase
- ✅ All reusable logic stays in Repository

## Current Implementation Example

### Real Code Example: `GetUsersUseCaseImpl`

**Repository (Reusable Logic):**
```rust
// ✅ user_data/src/user_repository.rs
// ALL reusable logic is here - can be used by ANY UseCase

impl UserRepository for UserRepositoryImpl {
    async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
        // ✅ HTTP request logic (reusable)
        // ✅ JSON parsing (reusable)
        // ✅ Error handling (reusable)
        self.helper.get("users").await
    }
    
    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError> {
        // ✅ HTTP request logic (reusable)
        self.helper.get(&format!("users/{}", id)).await
    }
}
```

**UseCase (Thin Orchestrator):**
```rust
// ✅ user_domain/src/get_users_use_case.rs
// Thin - just orchestrates, delegates to Repository

impl GetUsersUseCaseImpl {
    pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
        // 1. Delegate to Repository (reusable logic)
        let users_response = self.user_repository
            .get_users() // ✅ Reuses Repository method
            .await
            .map_err(|_| ErrorDisplay::repository())?;
        
        // 2. Map Data → Domain (screen-specific transformation)
        Ok(UserDataToDomainMapper::vec_map(&users_response.users)
            .into_iter()
            .map(UserDomainModel::from)
            .collect())
    }
    
    pub async fn execute_by_id(&self, id: u64) -> Result<UserDomainModel, DomainError> {
        // 1. Screen-specific business validation
        if id == 0 {
            return Err(DomainError::Error {
                display: ErrorDisplay::business_logic("User ID must be > 0".to_string()),
            });
        }
        
        // 2. Delegate to Repository (reusable logic)
        let user_data = self.user_repository
            .get_user_by_id(id) // ✅ Reuses Repository method
            .await
            .map_err(|_| ErrorDisplay::repository())?;
        
        // 3. Map Data → Domain
        Ok(UserDomainModel::from(UserDataToDomainMapper::map(&user_data)))
    }
}
```

**Key Observations:**
- ✅ UseCase is **very thin** - just orchestrates
- ✅ All API logic is in Repository (reusable)
- ✅ Multiple UseCases can reuse same Repository methods
- ✅ Screen-specific validation stays in UseCase
- ✅ Data → Domain mapping in UseCase (screen-specific)

### ✅ Architecture Compliance

**Current implementation follows the principles:**

1. **1:1:1 Relationship**: ✅
   - `GetUsersUseCaseImpl` is dedicated to UserListScreen
   - Each ViewModel uses exactly one UseCase
   - Clear ownership

2. **Reusable Logic in Repository**: ✅
   - All API calls in `UserRepositoryImpl`
   - HTTP handling in `HttpRepositoryHelper`
   - UseCases just delegate

3. **Thin UseCases**: ✅
   - `GetUsersUseCaseImpl` delegates to Repository
   - Only contains screen-specific validation and mapping
   - No reusable operations

### 📋 Guidelines for Future Development

1. **When adding a new screen:**
   - ✅ Create new UseCase (1:1 with ViewModel)
   - ✅ Reuse existing Repository methods if possible
   - ✅ Add new Repository methods only if truly reusable
   - ✅ Keep UseCase thin - just orchestrate

2. **When adding reusable logic:**
   - ✅ **ALWAYS** add to Repository (not UseCase)
   - ✅ All UseCases automatically benefit
   - ✅ Single source of truth
   - ✅ Examples: retry logic, caching, error handling, data transformation

3. **When adding business logic:**
   - ✅ Screen-specific validation → UseCase
   - ✅ Reusable data operations → Repository
   - ✅ UI logic → ViewModel

4. **Decision Tree:**
   ```
   Is the logic reusable across multiple UseCases?
   ├─ YES → Put in Repository ✅
   └─ NO → Is it screen-specific business logic?
       ├─ YES → Put in UseCase ✅
       └─ NO → Is it UI-related?
           ├─ YES → Put in ViewModel ✅
           └─ NO → Re-evaluate the requirement
   ```

## Best Practices

### ✅ DO

1. **Put ALL reusable logic in RepositoryImpl**:
   ```rust
   // ✅ GOOD: All API, network, caching logic in Repository
   impl UserRepositoryImpl {
       async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
           // ✅ HTTP request logic
           // ✅ JSON parsing
           // ✅ Error handling
           // ✅ Retry logic
           // ✅ Caching
           self.helper.get("users").await
       }
   }
   ```

2. **Keep UseCases VERY thin - just orchestrate**:
   ```rust
   // ✅ GOOD: UseCase just orchestrates and validates
   pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
       // 1. Screen-specific validation (if any)
       // 2. Delegate to Repository (reusable logic)
       let data = self.repository.get_users().await?;
       // 3. Map to Domain model
       Ok(mapper.map(&data))
   }
   ```

3. **Maintain 1:1:1 relationship strictly**:
   ```rust
   // ✅ GOOD: Clear 1:1:1 ownership
   UserListScreen → UserListViewModel → GetUsersUseCase
   UserDetailScreen → UserDetailViewModel → GetUserDetailUseCase
   ```

4. **Reuse Repository methods across UseCases**:
   ```rust
   // ✅ GOOD: Multiple UseCases share same Repository method
   GetUsersUseCase → uses → UserRepository.get_users()
   GetUserDetailUseCase → uses → UserRepository.get_user_by_id()
   SearchUsersUseCase → uses → UserRepository.search_users()
   // All API logic in one place (Repository)
   ```

### ❌ DON'T

1. **Don't duplicate reusable logic in UseCases**:
   ```rust
   // ❌ BAD: API logic duplicated in UseCases
   impl GetUsersUseCase {
       async fn execute(&self) {
           // HTTP request here ❌
           // JSON parsing here ❌
           // Should be in Repository!
       }
   }
   impl GetUserDetailUseCase {
       async fn execute(&self) {
           // Same HTTP request logic here ❌
           // Should reuse Repository method!
       }
   }
   
   // ✅ GOOD: Both UseCases reuse Repository
   impl GetUsersUseCase {
       async fn execute(&self) {
           self.repository.get_users().await? // ✅ Reuse
       }
   }
   impl GetUserDetailUseCase {
       async fn execute(&self) {
           self.repository.get_user_by_id(id).await? // ✅ Reuse
       }
   }
   ```

2. **Don't put business logic in Repository**:
   ```rust
   // ❌ BAD: Business logic in Data layer
   impl UserRepositoryImpl {
       async fn get_user_by_id(&self, id: u64) {
           if id == 0 { // ❌ Business rule - belongs in UseCase
               return Err(...);
           }
           // API logic...
       }
   }
   
   // ✅ GOOD: Business logic in UseCase, API logic in Repository
   impl GetUserDetailUseCase {
       async fn execute(&self, id: u64) {
           if id == 0 { // ✅ Business validation here
               return Err(...);
           }
           self.repository.get_user_by_id(id).await? // ✅ API logic in Repository
       }
   }
   ```

3. **Don't share UseCases across screens**:
   ```rust
   // ❌ BAD: One UseCase for multiple screens
   GetUsersUseCase → UserListScreen + UserSearchScreen ❌
   
   // ✅ GOOD: Separate UseCases (1:1:1)
   GetUsersUseCase → UserListScreen ✅
   SearchUsersUseCase → UserSearchScreen ✅
   ```

4. **Don't put reusable operations in UseCase**:
   ```rust
   // ❌ BAD: Reusable logic in UseCase
   impl GetUsersUseCase {
       async fn execute(&self) {
           // Retry logic here ❌
           // Caching logic here ❌
           // Should be in Repository!
       }
   }
   
   // ✅ GOOD: Reusable logic in Repository
   impl UserRepositoryImpl {
       async fn get_users(&self) {
           // Retry logic here ✅
           // Caching logic here ✅
           // All UseCases benefit
       }
   }
   ```

## Summary

### Core Principles

1. **1:1:1 Relationship**: Screen → ViewModel → UseCase (strict ownership)
   - Each screen has exactly one ViewModel
   - Each ViewModel uses exactly one UseCase
   - Never share UseCases across screens

2. **All Reusable Logic in Repository**: 
   - API calls, HTTP handling, retry logic, caching → Repository
   - Single source of truth for data operations
   - All UseCases benefit from improvements

3. **UseCases are Very Thin**: 
   - Just orchestrate (delegate to Repository)
   - Screen-specific business validation only
   - Map Data → Domain models

4. **Clear Dependencies**: 
   - Platform → Domain → Data → Network
   - No reverse dependencies
   - No cross-layer dependencies

### Benefits

This architecture ensures:
- ✅ **Easy to add new features** - Follow the pattern, reuse Repository
- ✅ **Easy to test** - Mock Repository, test UseCase in isolation
- ✅ **Easy to maintain** - Changes to data operations in one place (Repository)
- ✅ **Clear responsibilities** - Each layer has a single, well-defined purpose
- ✅ **No code duplication** - Reusable logic centralized in Repository
- ✅ **Scalable** - Add new screens without duplicating data logic

### Quick Reference

| Layer | Contains | Should NOT Contain |
|-------|----------|-------------------|
| **Repository** | API calls, HTTP, retry, caching, data operations | Business logic, UI logic, screen-specific code |
| **UseCase** | Screen-specific validation, orchestration, Data→Domain mapping | API calls, reusable operations, HTTP logic |
| **ViewModel** | UI state, user interactions, calling UseCase | Business logic, data fetching, API calls |
| **View** | UI rendering, user input | Business logic, state management, data operations |

### ✅ Checklist: Adding a New Screen

When adding a new screen, follow this checklist:

- [ ] **Create new UseCase** (1:1 with ViewModel)
  - [ ] UseCase name matches screen purpose
  - [ ] UseCase is thin - just orchestrates
  - [ ] Delegates to Repository (no API logic)

- [ ] **Check Repository for reusable methods**
  - [ ] Reuse existing Repository methods if possible
  - [ ] Only add new Repository methods if truly reusable
  - [ ] All API/HTTP logic stays in Repository

- [ ] **Create new ViewModel** (1:1 with UseCase)
  - [ ] ViewModel uses exactly one UseCase
  - [ ] ViewModel handles UI state only
  - [ ] No business logic in ViewModel

- [ ] **Verify architecture compliance**
  - [ ] 1:1:1 relationship maintained
  - [ ] No reusable logic in UseCase
  - [ ] All reusable logic in Repository
  - [ ] Clear dependency flow
