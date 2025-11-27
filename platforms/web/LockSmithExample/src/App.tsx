import { useState, useEffect } from 'react'
import { 
  WasmPasswordValidator, 
  initialize_localization, 
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

function App() {
  console.log('App component rendering...')
  const [wasmLoaded, setWasmLoaded] = useState(false)
  const [validator, setValidator] = useState<WasmPasswordValidator | null>(null)
  const [password, setPassword] = useState('')
  const [isPasswordVisible, setIsPasswordVisible] = useState(false)
  const [selectedLocale, setSelectedLocale] = useState<LocaleOption>(LOCALE_OPTIONS[0])
  const [validationMessage, setValidationMessage] = useState('')
  const [localizedTexts, setLocalizedTexts] = useState<Record<string, string>>({})

  // Initialize WASM
  useEffect(() => {
    const initWasm = async () => {
      try {
        console.log('Initializing WASM...')
        
        // With bundler target, WASM loads automatically when module is imported
        // Small delay to ensure WASM module is fully loaded
        await new Promise(resolve => setTimeout(resolve, 50))
        
        console.log('Calling initialize_localization...')
        initialize_localization()
        
        console.log('Setting app locale...')
        set_app_locale(selectedLocale.code)
        
        console.log('Creating validator...')
        setValidator(new WasmPasswordValidator())
        
        console.log('WASM initialized successfully')
        setWasmLoaded(true)
      } catch (error) {
        console.error('Failed to initialize WASM:', error)
        // Show UI even on error so user can see what went wrong
        setWasmLoaded(true)
      }
    }
    
    initWasm()
  }, [])

  // Update locale when selectedLocale changes
  useEffect(() => {
    if (wasmLoaded) {
      set_app_locale(selectedLocale.code)
      updateLocalizedTexts()
    }
  }, [selectedLocale, wasmLoaded])

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
    if (!wasmLoaded) return
    
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
    if (wasmLoaded) {
      updateLocalizedTexts()
    }
  }, [wasmLoaded, selectedLocale])

  const handleSamplePasswordClick = (sample: SamplePassword) => {
    setPassword(sample.value)
  }

  const isValid = validationMessage === localizedTexts['password-valid']
  const validationColor = password ? (isValid ? '#4CAF50' : '#FF0000') : '#FF0000'

  if (!wasmLoaded) {
    return (
      <div className="app">
        <h1>Loading LockSmith...</h1>
        <p>Initializing WebAssembly module...</p>
      </div>
    )
  }

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

export default App

