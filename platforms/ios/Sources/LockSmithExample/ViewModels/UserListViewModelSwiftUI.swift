import Foundation
import SwiftUI
import LockSmith
import Combine

// MARK: - Presentation Model
//struct UserPresentationModel: Identifiable {
//    let id: UInt64
//    let displayName: String
//    let email: String
//    let phone: String
//    let age: UInt32
//    let ageDisplay: String
//}

/// SwiftUI-based ViewModel for User List
/// This ViewModel uses @Published properties for SwiftUI binding
@MainActor
final class UserListViewModelSwiftUI: ObservableObject {
    // MARK: - Published Properties
    @Published var users: [UserPresentationModel] = []
    @Published var isLoading: Bool = false
    @Published var errorMessage: String?
    @Published var showError: Bool = false
    
    // MARK: - Private
    private let useCase: GetUsersUseCaseImpl
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Initialization
    init(useCase: GetUsersUseCaseImpl? = nil) {
        self.useCase = useCase ?? (try! GetUsersUseCaseImpl())
    }
    
    // MARK: - Public Methods
    func loadUsers() {
        isLoading = true
        errorMessage = nil
        showError = false
        
        Task { @MainActor in
            do {
                let domainUsers = try await useCase.execute()
                let presentationUsers = domainUsers.map { domainUser in
                    UserPresentationModel(
                        id: domainUser.id,
                        displayName: domainUser.fullName,
                        email: domainUser.email,
                        phone: domainUser.phone,
                        age: domainUser.age,
                        ageDisplay: "\(domainUser.age) years old"
                    )
                }
                
                self.users = presentationUsers
                self.isLoading = false
            } catch let error as DomainError {
                self.isLoading = false
                self.errorMessage = formatError(error)
                self.showError = true
            } catch {
                self.isLoading = false
                self.errorMessage = formatError(error)
                self.showError = true
            }
        }
    }
    
    func refresh() {
        loadUsers()
    }
    
    // MARK: - Private Methods
    private func formatError(_ error: Error) -> String {
        // Convert DomainError to ErrorDisplay - platforms just need title/subtitle
        if let domainError = error as? DomainError {
            let errorDisplay = useCase.toErrorDisplay(error: domainError)
            return "\(errorDisplay.title): \(errorDisplay.subtitle)"
        }
        return error.localizedDescription
    }
}

