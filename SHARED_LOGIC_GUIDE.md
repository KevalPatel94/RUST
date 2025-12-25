# Shared Logic Guide - Quick Reference

## 🎯 Problem
Multiple UseCases need the same data processing logic. Where does it go?

## 🏗️ Architecture: Data Sources + Repository + Extensions

**Key Principle:**
- **Data Sources Layer** = Separate crate handling data fetching (network, local storage, cache, etc.)
- **Repository Layer** = Orchestrates data sources + **Shared processing extensions**
- **Repository Extensions** = Shared data processing logic used by multiple UseCases
- **UseCase** = Orchestrates Repository calls, uses extensions for processing

**Dependency Flow:**
```
UseCase → Repository → Data Sources → Network/Storage
         (extensions)
```

## ✅ Solutions by Scenario

### 1. Operations on Data FROM Repository (Functional Style)

**Single Domain:**
```rust
// user_domain/src/processors/user_filter.rs

/// Pure functions for filtering user data
pub mod user_filter {
    use crate::user_domain_model::UserDomainModel;

    /// Filter users by age range (pure function)
    pub fn filter_by_age(
        users: &[UserDomainModel],
        min: u32,
        max: u32,
    ) -> Vec<UserDomainModel> {
        users
            .iter()
            .filter(|user| {
                let age = parse_age(&user.age_display);
                age >= min && age <= max
            })
            .cloned()
            .collect()
    }

    /// Filter active users (pure function)
    pub fn filter_active(users: &[UserDomainModel]) -> Vec<UserDomainModel> {
        users
            .iter()
            .filter(|user| is_active(user))
            .cloned()
            .collect()
    }

    /// Compose multiple filters (function composition)
    pub fn filter_by_multiple(
        users: &[UserDomainModel],
        predicates: Vec<Box<dyn Fn(&UserDomainModel) -> bool>>,
    ) -> Vec<UserDomainModel> {
        users
            .iter()
            .filter(|user| predicates.iter().all(|pred| pred(user)))
            .cloned()
            .collect()
    }

    // Helper pure functions
    fn parse_age(age_display: &str) -> u32 {
        age_display
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0)
    }

    fn is_active(user: &UserDomainModel) -> bool {
        !user.email_display.is_empty() && !user.phone.is_empty()
    }
}
```

**Multiple Domains (Generic Functional):**
```rust
// repository_common/src/shared_processors.rs

/// Generic functional operations for any data type
pub mod data_filter {
    /// Filter by predicate (pure function)
    pub fn filter<T, P>(items: &[T], predicate: P) -> Vec<T>
    where
        T: Clone,
        P: Fn(&T) -> bool,
    {
        items.iter().filter(|item| predicate(item)).cloned().collect()
    }

    /// Filter by range using extractor function (higher-order function)
    pub fn filter_by_range<T, F>(items: &[T], extractor: F, min: f64, max: f64) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> f64,
    {
        filter(items, |item| {
            let val = extractor(item);
            val >= min && val <= max
        })
    }

    /// Compose multiple filters (function composition)
    pub fn filter_all<T, P>(items: &[T], predicates: Vec<P>) -> Vec<T>
    where
        T: Clone,
        P: Fn(&T) -> bool,
    {
        filter(items, |item| predicates.iter().all(|pred| pred(item)))
    }

    /// Chain filters (functional pipeline)
    pub fn chain_filters<T>(items: Vec<T>, filters: Vec<Box<dyn Fn(Vec<T>) -> Vec<T>>>) -> Vec<T> {
        filters.into_iter().fold(items, |acc, f| f(acc))
    }
}

pub mod data_calculator {
    /// Calculate average using extractor function (pure function)
    pub fn average<T, F>(items: &[T], extractor: F) -> f64
    where
        F: Fn(&T) -> f64,
    {
        if items.is_empty() {
            return 0.0;
        }
        let sum: f64 = items.iter().map(&extractor).sum();
        sum / items.len() as f64
    }

    /// Count items matching predicate (pure function)
    pub fn count<T, P>(items: &[T], predicate: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        items.iter().filter(|item| predicate(item)).count()
    }

    /// Group by key extractor (functional grouping)
    pub fn group_by<T, K, F>(items: &[T], key_extractor: F) -> std::collections::HashMap<K, Vec<T>>
    where
        T: Clone,
        K: std::hash::Hash + Eq,
        F: Fn(&T) -> K,
    {
        items.iter().fold(
            std::collections::HashMap::new(),
            |mut acc, item| {
                let key = key_extractor(item);
                acc.entry(key).or_insert_with(Vec::new).push(item.clone());
                acc
            },
        )
    }
}

pub mod data_transformer {
    /// Map items using transformer function (pure function)
    pub fn map<T, U, F>(items: &[T], transformer: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        items.iter().map(transformer).collect()
    }

    /// Sort by key extractor (pure function, returns new Vec)
    pub fn sort_by<T, K, F>(items: &[T], key_extractor: F) -> Vec<T>
    where
        T: Clone,
        K: Ord,
        F: Fn(&T) -> K,
    {
        let mut result: Vec<T> = items.to_vec();
        result.sort_by_key(|item| key_extractor(item));
        result
    }
}
```

### 4. UseCase Using Repository Extensions

```rust
// user_domain/src/get_users_by_age_use_case.rs

use user_data::extensions::user_filter_ext;
use user_data::UserRepository;

impl GetUsersByAgeUseCaseImpl {
    pub async fn execute(&self, min_age: u32, max_age: u32) -> UserDomainResultModel {
        // 1. Get filtered data from Repository (uses extension internally)
        let filtered_users = self.user_repository
            .get_users_filtered(Some(min_age), Some(max_age))
            .await
            .map_err(|e| ErrorDisplay {
                title: "Error".to_string(),
                subtitle: format!("Failed to get users: {:?}", e),
            })?;
        
        // 2. Map Data → Domain
        let domain_users = UserDataToDomainMapper::vec_map(&filtered_users);
        
        // 3. Return result
        if domain_users.is_empty() {
            UserDomainResultModel::Empty { /* ... */ }
        } else {
            UserDomainResultModel::Loaded { data: domain_users }
        }
    }
}
```

### 5. Direct Extension Usage in UseCase

```rust
// user_domain/src/get_user_statistics_use_case.rs

use user_data::{UserRepository, extensions::user_calculator_ext};
use user_data::extensions::user_filter_ext;

impl GetUserStatisticsUseCaseImpl {
    pub async fn execute(&self) -> UserStatisticsResultModel {
        // 1. Get data from Repository
        let users_response = self.user_repository.get_users().await?;
        
        // 2. Use repository extensions directly for processing
        let active_users = user_filter_ext::filter_active(&users_response.users);
        let avg_age = user_calculator_ext::average_age(&active_users);
        let (young, adult, senior) = user_calculator_ext::count_by_age_group(&active_users);
        
        // 3. Map to domain model
        UserStatisticsResultModel::Loaded {
            average_age: avg_age,
            young_count: young,
            adult_count: adult,
            senior_count: senior,
        }
    }
}
```

**Multiple Domains:**
⚠️ **Avoid `domain_common`** (creates cyclic dependencies)
- Use `repository_common` if data-related
- Or keep in each domain if truly domain-specific

### 6. Repository Extensions Module Structure

```rust
// user_data/src/extensions/mod.rs

pub mod user_filter_ext;
pub mod user_calculator_ext;
pub mod user_transformer_ext;

// Re-export for convenience
pub use user_filter_ext::*;
pub use user_calculator_ext::*;
pub use user_transformer_ext::*;
```

## 📋 Quick Decision Table

| What | Single Domain | Multiple Domains |
|------|--------------|------------------|
| **Data sources (network/local/cache)** | `data_sources/` crate | `repository_common/` |
| **Data processing (filter/sort/calculate)** | `data/extensions/` | `repository_common/extensions/` |
| **Repository orchestration** | `data/repository.rs` | `repository_common/` |
| **Business validation** | `domain/utils/` | Avoid `domain_common` |

## 🏗️ Structure (Data Sources + Repository + Extensions)

```
user_data_sources/        # Data Sources layer (separate crate)
└── src/
    ├── network_user_source.rs    # Network data fetching
    ├── local_user_source.rs      # Local storage data fetching
    ├── cache_user_source.rs      # Cache data fetching
    └── user_data_source_trait.rs # Trait for data sources

user_data/                # Repository layer crate
└── src/
    ├── user_repository.rs        # Orchestrates data sources
    ├── user_data_model.rs
    └── extensions/               # ✅ Shared processing logic
        ├── mod.rs
        ├── user_filter_ext.rs
        ├── user_calculator_ext.rs
        └── user_transformer_ext.rs

user_domain/              # Domain layer crate
└── src/
    ├── get_users_use_case.rs     # Uses repository + extensions
    └── utils/                    # Business validation only
        └── user_validator.rs

repository_common/        # Shared across domains
└── src/
    ├── http_repository_helper.rs
    └── extensions/               # ✅ Cross-domain processing
        └── data_processor_ext.rs

network/                  # Network layer (lowest level)
└── src/
    └── client.rs
```

## ⚠️ Critical Rules

1. **Data Sources = Separate Layer**: Data sources (network/local/cache) are in separate crate
2. **Repository = Orchestrator + Extensions**: Repository orchestrates data sources + shared processing
3. **Extensions in repository layer**: Shared processing logic lives in `data/extensions/`
4. **No cycles**: Domain → Repository → Data Sources → Network (one way)
5. **UseCase uses extensions**: Either via Repository methods or directly
6. **Cross-domain extensions**: Put in `repository_common/extensions/`

## 💡 Real Example (Repository Extensions)

```rust
// user_domain/src/get_users_by_age_use_case.rs
use user_data::{UserRepository, extensions::user_filter_ext};

impl GetUsersByAgeUseCaseImpl {
    pub async fn execute(&self, min: u32, max: u32) -> UserDomainResultModel {
        // Option 1: Use Repository method that uses extension internally
        let filtered_users = self.user_repository
            .get_users_filtered(Some(min), Some(max))
            .await?;
        
        // Option 2: Get all users, then use extension directly
        // let response = self.user_repository.get_users().await?;
        // let filtered_users = user_filter_ext::filter_by_age(&response.users, min, max);
        
        // Map Data → Domain
        let domain_users = UserDataToDomainMapper::vec_map(&filtered_users);
        
        UserDomainResultModel::Loaded { data: domain_users }
    }
}
```

```rust
// user_data/src/user_repository.rs - Repository with extension methods

impl UserRepositoryImpl {
    /// Get users filtered by age (uses extension)
    pub async fn get_users_by_age(
        &self,
        min_age: u32,
        max_age: u32,
    ) -> Result<Vec<UserDataModel>, UserRepositoryError> {
        let response = self.get_users().await?;
        
        // Use repository extension for shared filtering logic
        Ok(user_filter_ext::filter_by_age(&response.users, min_age, max_age))
    }
    
    /// Get active users (uses extension)
    pub async fn get_active_users(&self) -> Result<Vec<UserDataModel>, UserRepositoryError> {
        let response = self.get_users().await?;
        Ok(user_filter_ext::filter_active(&response.users))
    }
}
```

## 🎨 Functional Programming Patterns in Extensions

1. **Pure Functions**: No side effects, same input → same output
2. **Higher-Order Functions**: Functions that take/return functions (predicates, extractors)
3. **Function Composition**: Chain operations together
4. **Immutability**: Operations return new data, don't mutate input
5. **Closures**: Capture context for predicates/extractors
6. **Iterator Chains**: `.iter().filter().map().collect()`

## 🎯 TL;DR

**Architecture:**
- **Data Sources Layer** = Separate crate for network/local/cache fetching
- **Repository Layer** = Orchestrates data sources + extensions for processing
- **Repository Extensions** = Shared data processing logic (functional style)
- **Single domain** → `user_data_sources/` + `user_data/extensions/`
- **Multiple domains** → `repository_common/extensions/`
- **UseCase** → Uses Repository (which uses data sources + extensions)
- **No duplication** → All shared processing in extensions
- **Functional style** → Pure functions, immutability, composition

**Dependency Flow:**
```
UseCase → Repository → Data Sources → Network/Storage
         (uses extensions)
```

## 🎯 TL;DR

- **Single domain** → Module in that domain crate
- **Multiple domains** → `repository_common`
- **Never** → Repository-dependent logic in `domain_common`

