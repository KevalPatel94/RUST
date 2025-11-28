# LockSmith Architecture

## Core Principles

### 1. One-to-One Relationship: Screen → ViewModel → UseCase

Each screen in the application follows a strict 1:1:1 relationship:
- **1 Screen** = **1 ViewModel** = **1 UseCase**

This ensures:
- Clear ownership and responsibility
- No shared state between screens
- Easy to reason about and test
- Simple dependency flow

**Example:**
```
UserListScreen → UserListViewModel → GetUsersUseCase
UserDetailScreen → UserDetailViewModel → GetUserDetailUseCase
```

### 2. Common Logic Lives in Data Layer

All shared/reusable logic should be placed in the **Data Layer** (specifically in `RepositoryImpl`):
- API calls
- Data fetching
- Data transformation (raw API → Data Models)
- Caching strategies
- Network retry logic
- Common data operations

**Why?**
- Single source of truth for data operations
- All UseCases can reuse the same logic
- Easy to test and mock
- Changes to data sources only affect one place

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
**Purpose:** Business logic and domain models

**Contains:**
- `GetUsersUseCase` - thin orchestrator for user list screen
- `UserListDomainModel` - pure domain entities (no serialization concerns)
- `UserDomainModel` - UniFFI-exposed record for platforms
- `UserDataToDomainMapper` - converts Data → Domain models
- `ErrorDisplay` - error configuration for platforms

**Responsibilities:**
- ✅ Business logic validation (e.g., "ID must be > 0")
- ✅ Orchestrating repository calls
- ✅ Mapping Data models → Domain models
- ✅ Error transformation (Data errors → Domain errors)

**Should NOT contain:**
- ❌ API call logic (delegates to Repository)
- ❌ Screen-specific presentation logic
- ❌ Platform-specific code

### Presentation Layer (Platform: iOS/Android)
**Purpose:** UI and user interaction

**Contains:**
- ViewModels (one per screen)
- Views/ViewControllers
- UI state management

**Responsibilities:**
- ✅ UI state management (loading, loaded, error, empty)
- ✅ User interaction handling
- ✅ Calling UseCase methods
- ✅ Transforming Domain models → Presentation models (if needed)
- ✅ UI-specific logic (formatting, display)

**Should NOT contain:**
- ❌ Business logic (delegates to UseCase)
- ❌ Data fetching logic (delegates to UseCase)

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
// Add to existing UserRepository trait (common logic stays here)
async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError>;
// This is already implemented in UserRepositoryImpl - reuse it!
```

**Step 2: Domain Layer** (`user_domain/src/get_user_detail_use_case.rs`)
```rust
// New UseCase for User Detail screen
#[derive(uniffi::Object)]
pub struct GetUserDetailUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
    runtime: Arc<Runtime>,
}

impl GetUserDetailUseCaseImpl {
    pub async fn execute(&self, id: u64) -> Result<UserDomainModel, DomainError> {
        // Business logic validation
        if id == 0 {
            return Err(DomainError::Error {
                display: ErrorDisplay::business_logic("Invalid user ID".to_string()),
            });
        }
        
        // Delegate to repository (common logic)
        let user_data = self.user_repository
            .get_user_by_id(id)
            .await
            .map_err(|_| ErrorDisplay::repository())?;
        
        // Map to domain model
        Ok(UserDomainModel::from(UserDataToDomainMapper::map(&user_data)))
    }
}
```

**Step 3: Platform Layer** (iOS)
```swift
// New ViewModel for User Detail screen
class UserDetailViewModel {
    private let useCase: GetUserDetailUseCaseImpl
    
    func loadUser(id: UInt64) async {
        // Call UseCase, handle UI state
    }
}

// New ViewController
class UserDetailViewController {
    private let viewModel: UserDetailViewModel
    // UI implementation
}
```

## Current Structure Analysis

### ✅ What's Good

1. **Data Layer (`UserRepositoryImpl`)**:
   - Contains all API call logic
   - Handles HTTP requests, parsing, errors
   - Can be reused by multiple UseCases

2. **Domain Layer (`GetUsersUseCaseImpl`)**:
   - Thin orchestrator
   - Delegates data fetching to Repository
   - Handles business logic (ID validation)
   - Maps Data → Domain models

3. **Platform Layer**:
   - ViewModels use UseCases directly
   - No business logic in ViewModels
   - Clean separation

### 🔄 Potential Improvements

1. **Multiple UseCases can share the same Repository**:
   - ✅ `GetUsersUseCase` uses `UserRepository`
   - ✅ `GetUserDetailUseCase` can also use `UserRepository`
   - ✅ Common logic (API calls) stays in `UserRepositoryImpl`

2. **UseCase should be thin**:
   - Current: ✅ Delegates to Repository
   - Current: ✅ Only contains screen-specific business logic
   - Good pattern to follow for future UseCases

3. **Repository should contain ALL common logic**:
   - Current: ✅ API calls are in Repository
   - Future: Add caching, retry logic, etc. to Repository
   - All UseCases benefit from these improvements

## Best Practices

### ✅ DO

1. **Put common logic in RepositoryImpl**:
   ```rust
   // Good: Common logic in Repository
   impl UserRepositoryImpl {
       async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
           // All API logic here
       }
   }
   ```

2. **Keep UseCases thin**:
   ```rust
   // Good: UseCase just orchestrates
   pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
       let data = self.repository.get_users().await?; // Delegate
       Ok(mapper.map(&data)) // Transform
   }
   ```

3. **One UseCase per screen**:
   ```rust
   // Good: Clear ownership
   GetUsersUseCase → UserListScreen
   GetUserDetailUseCase → UserDetailScreen
   ```

### ❌ DON'T

1. **Don't duplicate logic across UseCases**:
   ```rust
   // Bad: Duplicated API logic
   impl GetUsersUseCase {
       async fn execute(&self) {
           // API call logic here ❌
       }
   }
   impl GetUserDetailUseCase {
       async fn execute(&self) {
           // Same API call logic here ❌
       }
   }
   ```

2. **Don't put business logic in Repository**:
   ```rust
   // Bad: Business logic in Data layer
   impl UserRepositoryImpl {
       async fn get_users(&self) {
           if some_business_rule { // ❌ Should be in UseCase
               // ...
           }
       }
   }
   ```

3. **Don't share UseCases across screens**:
   ```rust
   // Bad: One UseCase for multiple screens
   GetUsersUseCase → UserListScreen + UserSearchScreen ❌
   
   // Good: Separate UseCases
   GetUsersUseCase → UserListScreen ✅
   SearchUsersUseCase → UserSearchScreen ✅
   ```

## Summary

- **1:1:1 Relationship**: Screen → ViewModel → UseCase (strict ownership)
- **Common Logic in Data Layer**: All reusable logic in `RepositoryImpl`
- **UseCases are Thin**: Just orchestrate and apply business rules
- **Clear Dependencies**: Platform → Domain → Data → Network

This architecture ensures:
- ✅ Easy to add new features
- ✅ Easy to test (mock Repository, test UseCase)
- ✅ Easy to maintain (changes in one place)
- ✅ Clear responsibilities
