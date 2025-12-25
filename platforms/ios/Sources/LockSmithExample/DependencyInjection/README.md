# Dependency Injection in iOS

This directory contains the dependency injection infrastructure for the iOS app.

## Overview

The dependency injection system provides:
- **Centralized dependency management** via `DependencyContainer`
- **Protocol-based abstractions** for easier testing and mocking
- **Lazy initialization** of dependencies
- **Test-friendly** design with easy dependency substitution

## Architecture

### DependencyContainer

The `DependencyContainer` is a singleton that manages all app dependencies:

```swift
// Get a ViewModel with injected dependencies
let viewModel = DependencyContainer.shared.makeUserListViewModelSwiftUI()

// Or get the use case directly
let useCase = DependencyContainer.shared.getUsersUseCase
```

### UseCase Protocols

UseCases are abstracted via protocols (`GetUsersUseCaseProtocol`) to enable:
- **Easy mocking** for unit tests
- **Dependency substitution** without changing ViewModel code
- **Type safety** while maintaining flexibility

### ViewModel Initialization

ViewModels accept dependencies via their initializers:

```swift
// Production: Uses DependencyContainer
let viewModel = DependencyContainer.shared.makeUserListViewModelSwiftUI()

// Testing: Inject mock dependencies
let mockUseCase = MockGetUsersUseCase()
let viewModel = UserListViewModelSwiftUI(useCase: mockUseCase)
```

## Usage

### In Production Code

```swift
// SwiftUI View
struct UserListViewSwiftUI: View {
    @StateObject private var viewModel = DependencyContainer.shared.makeUserListViewModelSwiftUI()
    // ...
}

// UIKit ViewController
final class UserListViewControllerRx: UIViewController {
    init(viewModel: UserListViewModelRx? = nil) {
        self.viewModel = viewModel ?? DependencyContainer.shared.makeUserListViewModelRx()
        // ...
    }
}
```

### In Tests

```swift
// Create a mock use case
class MockGetUsersUseCase: GetUsersUseCaseProtocol {
    var result: UserDomainResultModel = .loaded(data: [])
    
    func execute() async -> UserDomainResultModel {
        return result
    }
}

// Inject mock into ViewModel
let mockUseCase = MockGetUsersUseCase()
let viewModel = UserListViewModelSwiftUI(useCase: mockUseCase)
```

### Resetting Dependencies

For testing scenarios where you need a clean state:

```swift
// Reset all dependencies
DependencyContainer.shared.reset()

// Or set a custom dependency
let customUseCase = CustomGetUsersUseCase()
DependencyContainer.shared.setGetUsersUseCase(customUseCase)
```

## Benefits

1. **Testability**: Easy to inject mocks for unit testing
2. **Maintainability**: Centralized dependency management
3. **Flexibility**: Can swap implementations without changing ViewModels
4. **Type Safety**: Protocol-based design ensures compile-time safety
5. **Lazy Loading**: Dependencies are created only when needed

## Adding New Dependencies

To add a new dependency:

1. Add a private property to `DependencyContainer`:
```swift
private var _newUseCase: NewUseCaseImpl?
```

2. Add a computed property with lazy initialization:
```swift
var newUseCase: NewUseCaseImpl {
    if let existing = _newUseCase {
        return existing
    }
    let useCase = try! NewUseCaseImpl()
    _newUseCase = useCase
    return useCase
}
```

3. Add factory methods if needed:
```swift
func makeNewViewModel() -> NewViewModel {
    return NewViewModel(useCase: newUseCase)
}
```

4. Update `reset()` method to clear the dependency:
```swift
func reset() {
    _getUsersUseCase = nil
    _newUseCase = nil
}
```






