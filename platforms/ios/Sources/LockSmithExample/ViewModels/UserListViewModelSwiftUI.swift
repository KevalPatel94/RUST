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
    @Published var emptyStateTitle: String?
    @Published var emptyStateSubtitle: String?
    @Published var emptyStateButtonTitle: String?
    @Published var showEmptyState: Bool = false
    
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
        showEmptyState = false
        emptyStateTitle = nil
        emptyStateSubtitle = nil
        emptyStateButtonTitle = nil
        
        Task { @MainActor in
            let result = await useCase.execute()
            
            switch result {
            case .loaded(let domainUsers):
                let presentationUsers = domainUsers.map { domainUser in
                    UserPresentationModel(
                        id: domainUser.id,
                        displayName: domainUser.fullName,
                        email: domainUser.emailDisplay,
                        phone: domainUser.phone,
                        ageDisplay: domainUser.ageDisplay,
                        imageUrl: domainUser.imageUrl
                    )
                }
                self.users = presentationUsers
                self.isLoading = false
                self.showError = false
                self.showEmptyState = false
                
            case .empty(let emptyData):
                // Empty case has EpmtyDataModel with title/subtitle/buttonTitle
                self.users = []
                self.isLoading = false
                self.showError = false
                self.emptyStateTitle = emptyData.title
                self.emptyStateSubtitle = emptyData.subtitle
                self.emptyStateButtonTitle = emptyData.buttonTitle
                self.showEmptyState = true
                
            case .error(let errorDisplay):
                self.isLoading = false
                self.errorMessage = formatError(errorDisplay)
                self.showError = true
                self.showEmptyState = false
            }
        }
    }
    
    func refresh() {
        loadUsers()
    }
    
    // MARK: - Private Methods
    private func formatError(_ errorDisplay: ErrorDisplay) -> String {
        // Platforms directly receive ErrorDisplay - no conversion needed
        return "\(errorDisplay.title): \(errorDisplay.subtitle)"
    }
}

