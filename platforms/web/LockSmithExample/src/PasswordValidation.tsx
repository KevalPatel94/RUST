import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { 
  WasmPasswordValidator, 
  set_app_locale, 
  get_translated_text, 
  get_rust_demo_title 
} from 'locksmith'
import './App.css'

type LocaleOption = {
  code: string
  title: string
}

type SamplePassword = {
  localizationKey: string
  value: string
}

const LOCALE_OPTIONS: LocaleOption[] = [
  { code: 'en', title: 'English' },
  { code: 'es', title: 'Español' },
  { code: 'fr', title: 'Français' }
]

const SAMPLE_PASSWORDS: SamplePassword[] = [
  { localizationKey: 'password-sample-too-short', value: 'Ab1!' },
  { localizationKey: 'password-sample-too-long', value: 'Abcdefghijklmnopqrstu1!' },
  { localizationKey: 'password-sample-no-uppercase', value: 'abc1!abc' },
  { localizationKey: 'password-sample-no-lowercase', value: 'ABC1!ABC' },
  { localizationKey: 'password-sample-no-number', value: 'Abc!Abcd' },
  { localizationKey: 'password-sample-no-symbol', value: 'Abc1Abcd' },
  { localizationKey: 'password-sample-valid', value: 'Abc1!abc' }
]

function PasswordValidation() {
  const [validator] = useState(() => new WasmPasswordValidator())
  const [password, setPassword] = useState('')
  const [isPasswordVisible, setIsPasswordVisible] = useState(false)
  const [selectedLocale, setSelectedLocale] = useState<LocaleOption>(LOCALE_OPTIONS[0])
  const [validationMessage, setValidationMessage] = useState('')
  const [localizedTexts, setLocalizedTexts] = useState<Record<string, string>>({})

  // Update locale when selectedLocale changes
  useEffect(() => {
    set_app_locale(selectedLocale.code)
    updateLocalizedTexts()
  }, [selectedLocale])

  // Validate password when it changes
  useEffect(() => {
    if (validator && password) {
      const message = validator.validate(password)
      setValidationMessage(message)
    } else {
      setValidationMessage('')
    }
  }, [password, validator])

  const updateLocalizedTexts = () => {
    const texts: Record<string, string> = {}
    const keys = [
      'locale-picker-label',
      'password-input-placeholder',
      'password-instructions',
      'password-sample-header',
      'password-toggle-show',
      'password-toggle-hide',
      'password-valid',
      ...SAMPLE_PASSWORDS.map(s => s.localizationKey)
    ]
    
    keys.forEach(key => {
      texts[key] = get_translated_text(key)
    })
    
    setLocalizedTexts(texts)
  }

  useEffect(() => {
    updateLocalizedTexts()
  }, [selectedLocale])

  const handleSamplePasswordClick = (sample: SamplePassword) => {
    setPassword(sample.value)
  }

  const isValid = validationMessage === localizedTexts['password-valid']
  const validationColor = password ? (isValid ? '#4CAF50' : '#FF0000') : '#FF0000'

  // Get title safely
  let title = 'LockSmith'
  try {
    title = get_rust_demo_title('Web')
  } catch (error) {
    console.error('Error getting demo title:', error)
  }

  return (
    <div className="app">
      <header className="app-header">
        <Link to="/" className="back-link">← Back to Menu</Link>
        <h1>{title}</h1>
      </header>

      <main className="app-main">
        {/* Locale Selector */}
        <div className="locale-section">
          <p className="locale-label">{localizedTexts['locale-picker-label']}</p>
          <div className="locale-buttons">
            {LOCALE_OPTIONS.map(option => (
              <button
                key={option.code}
                className={`locale-button ${selectedLocale.code === option.code ? 'active' : ''}`}
                onClick={() => setSelectedLocale(option)}
              >
                {option.title}
              </button>
            ))}
          </div>
        </div>

        {/* Password Input */}
        <div className="password-section">
          <div className="password-input-group">
            <input
              type={isPasswordVisible ? 'text' : 'password'}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder={localizedTexts['password-input-placeholder']}
              className="password-input"
            />
            <button
              className="toggle-button"
              onClick={() => setIsPasswordVisible(!isPasswordVisible)}
            >
              {localizedTexts[isPasswordVisible ? 'password-toggle-hide' : 'password-toggle-show']}
            </button>
          </div>

          {/* Validation Message */}
          {validationMessage && (
            <p className="validation-message" style={{ color: validationColor }}>
              {validationMessage}
            </p>
          )}

          {/* Instructions */}
          <p className="instructions">{localizedTexts['password-instructions']}</p>
        </div>

        {/* Sample Passwords */}
        <div className="samples-section">
          <h2 className="samples-header">{localizedTexts['password-sample-header']}</h2>
          <div className="samples-list">
            {SAMPLE_PASSWORDS.map((sample) => (
              <button
                key={sample.localizationKey}
                className="sample-button"
                onClick={() => handleSamplePasswordClick(sample)}
              >
                •{localizedTexts[sample.localizationKey]}
              </button>
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}

export default PasswordValidation

