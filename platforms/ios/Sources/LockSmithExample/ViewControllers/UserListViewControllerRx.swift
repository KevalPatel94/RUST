import UIKit
import RxSwift
import RxCocoa
import LockSmith

/// UIKit + RxSwift implementation of User List View Controller
final class UserListViewControllerRx: UIViewController {
    
    // MARK: - UI Components
    private let tableView: UITableView = {
        let tableView = UITableView(frame: .zero, style: .plain)
        tableView.translatesAutoresizingMaskIntoConstraints = false
        tableView.register(UserTableViewCell.self, forCellReuseIdentifier: "UserCell")
        tableView.rowHeight = UITableView.automaticDimension
        tableView.estimatedRowHeight = 80
        return tableView
    }()
    
    private let loadingIndicator: UIActivityIndicatorView = {
        let indicator = UIActivityIndicatorView(style: .large)
        indicator.translatesAutoresizingMaskIntoConstraints = false
        indicator.hidesWhenStopped = true
        return indicator
    }()
    
    private let errorLabel: UILabel = {
        let label = UILabel()
        label.translatesAutoresizingMaskIntoConstraints = false
        label.textAlignment = .center
        label.numberOfLines = 0
        label.textColor = .systemRed
        label.font = UIFont.preferredFont(forTextStyle: .body)
        label.isHidden = true
        return label
    }()
    
    private let retryButton: UIButton = {
        let button = UIButton(type: .system)
        button.translatesAutoresizingMaskIntoConstraints = false
        button.setTitle("Retry", for: .normal)
        button.titleLabel?.font = UIFont.preferredFont(forTextStyle: .body)
        button.isHidden = true
        return button
    }()
    
    // MARK: - RxSwift
    private let disposeBag = DisposeBag()
    private let viewModel: UserListViewModelRx
    
    // MARK: - Initialization
    init(viewModel: UserListViewModelRx = UserListViewModelRx()) {
        self.viewModel = viewModel
        super.init(nibName: nil, bundle: nil)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    // MARK: - Lifecycle
    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
        setupConstraints()
        bindViewModel()
        viewModel.loadUsersTrigger.onNext(())
    }
    
    // MARK: - Setup
    private func setupUI() {
        title = "Users (RxSwift)"
        view.backgroundColor = .systemBackground
        
        view.addSubview(tableView)
        view.addSubview(loadingIndicator)
        view.addSubview(errorLabel)
        view.addSubview(retryButton)
        
        // Add refresh control
        let refreshControl = UIRefreshControl()
        tableView.refreshControl = refreshControl
        
        // Bind refresh control
        refreshControl.rx.controlEvent(.valueChanged)
            .subscribe(onNext: { [weak self] _ in
                self?.viewModel.refreshTrigger.onNext(())
            })
            .disposed(by: disposeBag)
        
        // Bind retry button
        retryButton.rx.tap
            .subscribe(onNext: { [weak self] _ in
                self?.viewModel.loadUsersTrigger.onNext(())
            })
            .disposed(by: disposeBag)
    }
    
    private func setupConstraints() {
        NSLayoutConstraint.activate([
            tableView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            tableView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            tableView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            tableView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            
            loadingIndicator.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            loadingIndicator.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            
            errorLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            errorLabel.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            errorLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 32),
            errorLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -32),
            
            retryButton.topAnchor.constraint(equalTo: errorLabel.bottomAnchor, constant: 16),
            retryButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
        ])
    }
    
    // MARK: - RxSwift Bindings
    private func bindViewModel() {
        // Bind users to table view
        viewModel.users
            .bind(to: tableView.rx.items(cellIdentifier: "UserCell", cellType: UserTableViewCell.self)) { _, user, cell in
                cell.configure(with: user)
            }
            .disposed(by: disposeBag)
        
        // Bind loading state
        viewModel.isLoading
            .subscribe(onNext: { [weak self] isLoading in
                if isLoading {
                    self?.loadingIndicator.startAnimating()
                } else {
                    self?.loadingIndicator.stopAnimating()
                    self?.tableView.refreshControl?.endRefreshing()
                }
            })
            .disposed(by: disposeBag)
        
        // Bind error state
        viewModel.error
            .subscribe(onNext: { [weak self] errorMessage in
                self?.errorLabel.text = errorMessage
                self?.errorLabel.isHidden = false
                self?.retryButton.isHidden = false
                self?.tableView.isHidden = true
            })
            .disposed(by: disposeBag)
        
        // Show/hide table view based on loading and error
        Observable.combineLatest(
            viewModel.isLoading,
            viewModel.error.map { $0 != nil }
        )
        .map { isLoading, hasError in
            !isLoading && !hasError
        }
        .bind(to: tableView.rx.isHidden)
        .disposed(by: disposeBag)
        
        // Hide error UI when loading starts
        viewModel.isLoading
            .filter { $0 }
            .subscribe(onNext: { [weak self] _ in
                self?.errorLabel.isHidden = true
                self?.retryButton.isHidden = true
            })
            .disposed(by: disposeBag)
        
        // Table view item selection
        tableView.rx.itemSelected
            .subscribe(onNext: { [weak self] indexPath in
                self?.tableView.deselectRow(at: indexPath, animated: true)
            })
            .disposed(by: disposeBag)
    }
}

// MARK: - User Table View Cell
private final class UserTableViewCell: UITableViewCell {
    private let nameLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.preferredFont(forTextStyle: .headline)
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }()
    
    private let emailLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.preferredFont(forTextStyle: .subheadline)
        label.textColor = .secondaryLabel
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }()
    
    private let ageLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.preferredFont(forTextStyle: .caption1)
        label.textColor = .tertiaryLabel
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }()
    
    override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        setupUI()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupUI() {
        contentView.addSubview(nameLabel)
        contentView.addSubview(emailLabel)
        contentView.addSubview(ageLabel)
        
        NSLayoutConstraint.activate([
            nameLabel.topAnchor.constraint(equalTo: contentView.topAnchor, constant: 12),
            nameLabel.leadingAnchor.constraint(equalTo: contentView.leadingAnchor, constant: 16),
            nameLabel.trailingAnchor.constraint(equalTo: contentView.trailingAnchor, constant: -16),
            
            emailLabel.topAnchor.constraint(equalTo: nameLabel.bottomAnchor, constant: 4),
            emailLabel.leadingAnchor.constraint(equalTo: nameLabel.leadingAnchor),
            emailLabel.trailingAnchor.constraint(equalTo: nameLabel.trailingAnchor),
            
            ageLabel.topAnchor.constraint(equalTo: emailLabel.bottomAnchor, constant: 4),
            ageLabel.leadingAnchor.constraint(equalTo: nameLabel.leadingAnchor),
            ageLabel.bottomAnchor.constraint(equalTo: contentView.bottomAnchor, constant: -12),
        ])
    }
    
    func configure(with user: UserPresentationModel) {
        nameLabel.text = user.displayName
        emailLabel.text = user.email
        ageLabel.text = user.ageDisplay
    }
}

