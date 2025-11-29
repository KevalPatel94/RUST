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
    private let useCase: GetUsersUseCaseImpl
    
    // MARK: - Initialization
    init(useCase: GetUsersUseCaseImpl? = nil) {
        self.useCase = useCase ?? (try! GetUsersUseCaseImpl())
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
                    do {
                        let domainUsers = try await self.useCase.execute()
                        let presentationUsers = domainUsers.map { domainUser in
                            UserPresentationModel(
                                id: domainUser.id,
                                displayName: domainUser.fullName,
                                email: domainUser.email,
                                phone: domainUser.phone,
                                age: domainUser.age,
                                ageDisplay: "\(domainUser.age) years old",
                                imageUrl: domainUser.imageUrl
                            )
                        }
                        await MainActor.run {
                            observer.onNext(presentationUsers)
                            observer.onCompleted()
                        }
                    } catch let error as DomainError {
                        await MainActor.run {
                            observer.onError(error)
                        }
                    } catch {
                        await MainActor.run {
                            observer.onError(error)
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
            },
            onError: { [weak self] error in
                self?.isLoading.onNext(false)
                let errorMessage = self?.formatError(error) ?? "Unknown error"
                self?.error.onNext(errorMessage)
            }
        )
        .disposed(by: disposeBag)
    }
    
    private func formatError(_ error: Error) -> String {
        // Convert DomainError to ErrorDisplay - platforms just need title/subtitle
        if let domainError = error as? DomainError {
            let errorDisplay = useCase.toErrorDisplay(error: domainError)
            return "\(errorDisplay.title): \(errorDisplay.subtitle)"
        }
        return error.localizedDescription
    }
}

// MARK: - Presentation Model
struct UserPresentationModel: Identifiable {
    let id: UInt64
    let displayName: String
    let email: String
    let phone: String
    let age: UInt32
    let ageDisplay: String
    let imageUrl: String
}

