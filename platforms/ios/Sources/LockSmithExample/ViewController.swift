import UIKit
import LockSmith

final class ViewController: UIViewController {
    private enum LocaleOption: Int, CaseIterable {
        case english
        case spanish
        case french

        var code: String {
            switch self {
            case .english: return "en"
            case .spanish: return "es"
            case .french: return "fr"
            }
        }

        var title: String {
            switch self {
            case .english: return "English"
            case .spanish: return "Español"
            case .french: return "Français"
            }
        }
    }

    private enum SamplePassword: Int, CaseIterable {
        case tooShort
        case tooLong
        case noUppercase
        case noLowercase
        case noNumber
        case noSymbol
        case valid

        var localizationKey: String {
            switch self {
            case .tooShort: return "password-sample-too-short"
            case .tooLong: return "password-sample-too-long"
            case .noUppercase: return "password-sample-no-uppercase"
            case .noLowercase: return "password-sample-no-lowercase"
            case .noNumber: return "password-sample-no-number"
            case .noSymbol: return "password-sample-no-symbol"
            case .valid: return "password-sample-valid"
            }
        }

        var value: String {
            switch self {
            case .tooShort: return "Ab1!"
            case .tooLong: return "Abcdefghijklmnopqrstu1!"
            case .noUppercase: return "abc1!abc"
            case .noLowercase: return "ABC1!ABC"
            case .noNumber: return "Abc!Abcd"
            case .noSymbol: return "Abc1Abcd"
            case .valid: return "Abc1!abc"
            }
        }
    }

    private let titleLabel: UILabel = {
        let label = UILabel()
        label.textAlignment = .center
        label.numberOfLines = 0
        label.font = UIFont.preferredFont(forTextStyle: .title2)
        return label
    }()

    private let localeLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.preferredFont(forTextStyle: .subheadline)
        label.textAlignment = .center
        label.numberOfLines = 0
        return label
    }()

    private lazy var localeControl: UISegmentedControl = {
        let control = UISegmentedControl(items: LocaleOption.allCases.map { $0.title })
        control.selectedSegmentIndex = 0
        control.addTarget(self, action: #selector(localeChanged(_:)), for: .valueChanged)
        return control
    }()

    private let showPasswordButton: UIButton = {
        let button = UIButton(type: .system)
        button.titleLabel?.font = UIFont.preferredFont(forTextStyle: .body)
        return button
    }()

    private let passwordField: UITextField = {
        let field = UITextField()
        field.borderStyle = .roundedRect
        field.isSecureTextEntry = true
        field.textContentType = .password
        field.autocorrectionType = .no
        field.autocapitalizationType = .none
        return field
    }()

    private let instructionsLabel: UILabel = {
        let label = UILabel()
        label.numberOfLines = 0
        label.textAlignment = .center
        label.font = UIFont.preferredFont(forTextStyle: .footnote)
        label.textColor = .secondaryLabel
        return label
    }()

    private let samplesLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.preferredFont(forTextStyle: .subheadline)
        label.textAlignment = .center
        label.numberOfLines = 0
        return label
    }()

    private let validationLabel: UILabel = {
        let label = UILabel()
        label.numberOfLines = 0
        label.textAlignment = .center
        label.font = UIFont.preferredFont(forTextStyle: .body)
        label.textColor = .systemRed
        return label
    }()

    private lazy var passwordFieldStack: UIStackView = {
        let stack = UIStackView(arrangedSubviews: [passwordField, showPasswordButton])
        stack.axis = .horizontal
        stack.spacing = 12
        showPasswordButton.setContentHuggingPriority(.required, for: .horizontal)
        passwordField.setContentHuggingPriority(.defaultLow, for: .horizontal)
        return stack
    }()

    private lazy var sampleButtonsStack: UIStackView = {
        let stack = UIStackView()
        stack.axis = .vertical
        stack.spacing = 8
        stack.distribution = .fillEqually
        SamplePassword.allCases.forEach { sample in
            let button = UIButton(type: .system)
            button.tag = sample.rawValue
            button.contentHorizontalAlignment = .left
            button.titleLabel?.numberOfLines = 0
            button.addTarget(self, action: #selector(sampleButtonTapped(_:)), for: .touchUpInside)
            sampleButtons[sample] = button
            stack.addArrangedSubview(button)
        }
        return stack
    }()

    private lazy var stackView: UIStackView = {
        let stack = UIStackView(arrangedSubviews: [
            titleLabel,
            localeLabel,
            localeControl,
            passwordFieldStack,
            validationLabel,
            instructionsLabel,
            samplesLabel,
            sampleButtonsStack
        ])
        stack.axis = .vertical
        stack.spacing = 16
        return stack
    }()

    private let validator = PasswordValidator()
    private var sampleButtons: [SamplePassword: UIButton] = [:]

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .systemBackground
        initializeLocalization()

        passwordField.addTarget(self, action: #selector(passwordChanged(_:)), for: .editingChanged)
        showPasswordButton.addTarget(self, action: #selector(togglePasswordVisibility), for: .touchUpInside)

        setupLayout()
        applyLocalization()
        updateValidationMessage(with: "")
        Task {
             let str = await validator.sayAfter(ms: 10000, who: "Kev😊")
            print(str)
        }

    }

    private func setupLayout() {
        view.addSubview(stackView)
        stackView.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            stackView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 32),
            stackView.leadingAnchor.constraint(equalTo: view.layoutMarginsGuide.leadingAnchor),
            stackView.trailingAnchor.constraint(equalTo: view.layoutMarginsGuide.trailingAnchor)
        ])

        passwordField.heightAnchor.constraint(equalToConstant: 44).isActive = true
    }

    private func applyLocalization() {
        let greeting = getRustDemoTitle(name: "iOS")
        titleLabel.text = greeting
        localeLabel.text = getLocalizedText(key: "locale-picker-label")
        passwordField.placeholder = getLocalizedText(key: "password-input-placeholder")
        instructionsLabel.text = getLocalizedText(key: "password-instructions")
        samplesLabel.text = getLocalizedText(key: "password-sample-header")
        updatePasswordToggleTitle()
        updateSampleButtons()
        updateValidationMessage(with: passwordField.text ?? "")
    }

    private func updateValidationMessage(with password: String) {
        let message = validator.validateWithMessage(password: password)
        validationLabel.text = message

        if password.isEmpty {
            validationLabel.textColor = .systemRed
            return
        }

        let successText = getLocalizedText(key: "password-valid")
        validationLabel.textColor = (message == successText) ? .systemGreen : .systemRed
    }

    private func updatePasswordToggleTitle() {
        let key = passwordField.isSecureTextEntry ? "password-toggle-show" : "password-toggle-hide"
        let title = getLocalizedText(key: key)
        showPasswordButton.setTitle(title, for: .normal)
    }

    private func updateSampleButtons() {
        SamplePassword.allCases.forEach { sample in
            let title = "• " + getLocalizedText(key: sample.localizationKey)
            sampleButtons[sample]?.setTitle(title, for: .normal)
        }
    }

    @objc
    private func passwordChanged(_ sender: UITextField) {
        updateValidationMessage(with: sender.text ?? "")
    }

    @objc
    private func togglePasswordVisibility() {
        passwordField.isSecureTextEntry.toggle()
        updatePasswordToggleTitle()
    }

    @objc
    private func sampleButtonTapped(_ sender: UIButton) {
        guard let sample = SamplePassword(rawValue: sender.tag) else {
            return
        }
        passwordField.text = sample.value
        passwordField.sendActions(for: .editingChanged)
    }

    @objc
    private func localeChanged(_ sender: UISegmentedControl) {
        guard let option = LocaleOption(rawValue: sender.selectedSegmentIndex) else {
            return
        }
        setAppLocale(locale: option.code)
        applyLocalization()
    }
}
