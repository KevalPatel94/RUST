import Foundation
import RxSwift
import RxCocoa
import LockSmith

/// RxSwift-based ViewModel for User List
/// This ViewModel uses RxSwift for reactive state management
final class UserListViewModelRx {
    // MARK: - Inputs
    let loadUsersTrigger = PublishSubject<Void>()
    let refreshTrigger = PublishSubject<Void>()
    
    // MARK: - Outputs
    let users: BehaviorSubject<[UserPresentationModel]> = BehaviorSubject(value: [])
    let isLoading: BehaviorSubject<Bool> = BehaviorSubject(value: false)
    let error: PublishSubject<String> = PublishSubject()
    
    // MARK: - Private
    private let disposeBag = DisposeBag()
    private let useCase: GetUsersUseCaseProtocol?
    
    // MARK: - Initialization
    /// Initialize with dependency injection
    /// - Parameter useCase: The use case to use. If nil, uses DependencyContainer.shared.getUsersUseCase
    init(useCase: GetUsersUseCaseProtocol? = GetUsersUseCaseImpl()) {
        self.useCase = useCase
        setupBindings()
    }
    
    // MARK: - Setup
    private func setupBindings() {
        // Combine load and refresh triggers
        Observable.merge(
            loadUsersTrigger.asObservable(),
            refreshTrigger.asObservable()
        )
        .do(onNext: { [weak self] _ in
            self?.isLoading.onNext(true)
        })
        .flatMapLatest { [weak self] _ -> Observable<[UserPresentationModel]> in
            guard let self = self else { return .just([]) }
            
            return Observable.create { observer in
                Task {
                    let result = await self.useCase?.execute()
                    guard let result = result else { return }
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
                        await MainActor.run {
                            observer.onNext(presentationUsers)
                            observer.onCompleted()
                        }
                        
                    case .empty(_):
                        // Empty case - no users to display
                        await MainActor.run {
                            observer.onNext([])
                            observer.onCompleted()
                        }
                        
                    case .error(let errorDisplay):
                        // Error case - convert ErrorDisplay to error
                        let errorMessage = "\(errorDisplay.title): \(errorDisplay.subtitle)"
                        await MainActor.run {
                            observer.onError(NSError(domain: "UserDomainError", code: -1, userInfo: [NSLocalizedDescriptionKey: errorMessage]))
                        }
                    }
                }
                return Disposables.create()
            }
        }
        .observe(on: MainScheduler.instance)
        .subscribe(
            onNext: { [weak self] users in
                self?.isLoading.onNext(false)
                self?.users.onNext(users)
            }
        )
        .disposed(by: disposeBag)
    }
}

// MARK: - Presentation Model
struct UserPresentationModel: Identifiable {
    let id: UInt64
    let displayName: String
    let email: String
    let phone: String
    let ageDisplay: String
    let imageUrl: String
}

