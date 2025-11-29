import { useState } from 'react'
import { Link } from 'react-router-dom'
import {
  WasmPasswordValidator,
  initialize_localization,
  set_app_locale,
} from 'locksmith'
import './Benchmark.css'

const TEST_PASSWORDS = [
  'Abc1!abc',        // Valid
  'Ab1!',           // Too short
  'Abcdefghijklmnopqrstu1!',  // Too long
  'abc1!abc',       // No uppercase
  'ABC1!ABC',       // No lowercase
  'Abc!Abcd',       // No number
  'Abc1Abcd',       // No symbol
]

const ROUNDS = 10_000

interface BenchmarkResult {
  name: string
  totalTime: number
  perOperation: number
  result: number
}

function Benchmark() {
  const [results, setResults] = useState<BenchmarkResult[]>([])
  const [isRunning, setIsRunning] = useState(false)
  const [output, setOutput] = useState('')

  // Native JavaScript Implementation
  const validatePasswordJS = (password: string): boolean => {
    if (password.length < 8) return false
    if (password.length > 20) return false
    if (!/[A-Z]/.test(password)) return false
    if (!/[a-z]/.test(password)) return false
    if (!/\d/.test(password)) return false
    if (!/[~!@#$%^&*()^&+=]/.test(password)) return false
    return true
  }

  const benchmarkJS = (): BenchmarkResult => {
    const startTime = performance.now()
    let validCount = 0

    for (let i = 0; i < ROUNDS; i++) {
      for (const password of TEST_PASSWORDS) {
        if (validatePasswordJS(password)) {
          validCount++
        }
      }
    }

    const totalTime = performance.now() - startTime
    const perOperation = (totalTime * 1_000_000) / (ROUNDS * TEST_PASSWORDS.length)

    return {
      name: 'Native JavaScript',
      totalTime,
      perOperation,
      result: validCount,
    }
  }

  // Rust Batch (Rust does 10,000 rounds internally)
  const benchmarkRustBatch = (validator: WasmPasswordValidator): BenchmarkResult => {
    const startTime = performance.now()
    
    const validCount = validator.validate_passwords_count_valid(
      TEST_PASSWORDS,
      ROUNDS
    )

    const totalTime = performance.now() - startTime
    const perOperation = (totalTime * 1_000_000) / (ROUNDS * TEST_PASSWORDS.length)

    return {
      name: 'Rust Batch Function',
      totalTime,
      perOperation,
      result: Number(validCount),
    }
  }

  // Rust Individual (10,000 FFI calls from JavaScript)
  const benchmarkRustIndividual = (validator: WasmPasswordValidator): BenchmarkResult => {
    const startTime = performance.now()
    let validCount = 0

    for (let i = 0; i < ROUNDS; i++) {
      for (const password of TEST_PASSWORDS) {
        const result = validator.validate(password)
        if (result.toLowerCase().includes('valid')) {
          validCount++
        }
      }
    }

    const totalTime = performance.now() - startTime
    const perOperation = (totalTime * 1_000_000) / (ROUNDS * TEST_PASSWORDS.length)

    return {
      name: 'Rust Individual Calls',
      totalTime,
      perOperation,
      result: validCount,
    }
  }

  const runBenchmark = async () => {
    setIsRunning(true)
    setOutput('')

    try {
      // Initialize WASM
      initialize_localization()
      set_app_locale('en')
      const validator = new WasmPasswordValidator()

      let outputText = '='.repeat(70) + '\n'
      outputText += 'Password Validator Benchmark\n'
      outputText += '='.repeat(70) + '\n'
      outputText += `Rounds: ${ROUNDS.toLocaleString()}\n`
      outputText += `Passwords per round: ${TEST_PASSWORDS.length}\n`
      outputText += `Total validations: ${(ROUNDS * TEST_PASSWORDS.length).toLocaleString()}\n`
      outputText += '='.repeat(70) + '\n\n'

      const benchmarkResults: BenchmarkResult[] = []

      // Benchmark 1: Native JavaScript
      outputText += '1. Native JavaScript Implementation (10,000 calls in JavaScript)\n'
      outputText += '-'.repeat(70) + '\n'
      const jsResult = benchmarkJS()
      benchmarkResults.push(jsResult)
      outputText += `  Total time: ${jsResult.totalTime.toFixed(3)} ms\n`
      outputText += `  Time per operation: ${jsResult.perOperation.toFixed(2)} ns\n`
      outputText += `  Valid passwords found: ${jsResult.result.toLocaleString()}\n\n`

      // Benchmark 2: Rust Batch
      outputText += '2. Rust Batch Function (10,000 rounds in Rust)\n'
      outputText += '-'.repeat(70) + '\n'
      const rustBatchResult = benchmarkRustBatch(validator)
      benchmarkResults.push(rustBatchResult)
      outputText += `  Total time: ${rustBatchResult.totalTime.toFixed(3)} ms\n`
      outputText += `  Time per operation: ${rustBatchResult.perOperation.toFixed(2)} ns\n`
      outputText += `  Valid passwords found: ${rustBatchResult.result.toLocaleString()}\n\n`

      // Benchmark 3: Rust Individual
      outputText += '3. Rust Individual Calls (10,000 FFI calls from JavaScript)\n'
      outputText += '-'.repeat(70) + '\n'
      const rustIndividualResult = benchmarkRustIndividual(validator)
      benchmarkResults.push(rustIndividualResult)
      outputText += `  Total time: ${rustIndividualResult.totalTime.toFixed(3)} ms\n`
      outputText += `  Time per operation: ${rustIndividualResult.perOperation.toFixed(2)} ns\n`
      outputText += `  Valid passwords found: ${rustIndividualResult.result.toLocaleString()}\n\n`

      // Comparison
      outputText += '='.repeat(70) + '\n'
      outputText += 'Performance Comparison\n'
      outputText += '='.repeat(70) + '\n'

      const speedupBatch = jsResult.totalTime / rustBatchResult.totalTime
      const speedupIndividual = jsResult.totalTime / rustIndividualResult.totalTime
      const ffiOverhead = rustIndividualResult.totalTime / rustBatchResult.totalTime

      outputText += `Rust Batch vs JavaScript:     ${speedupBatch.toFixed(2)}x ${speedupBatch > 1 ? 'faster' : 'slower'}\n`
      outputText += `Rust Individual vs JavaScript: ${speedupIndividual.toFixed(2)}x ${speedupIndividual > 1 ? 'faster' : 'slower'}\n`
      outputText += `FFI Overhead:                  ${ffiOverhead.toFixed(2)}x (Individual vs Batch)\n`
      outputText += '\n'

      if (jsResult.result === rustBatchResult.result && jsResult.result === rustIndividualResult.result) {
        outputText += `✓ Results match: ${jsResult.result.toLocaleString()} valid passwords\n`
      } else {
        outputText += '⚠ WARNING: Result mismatch!\n'
        outputText += `  JavaScript: ${jsResult.result}, Rust Batch: ${rustBatchResult.result}, Rust Individual: ${rustIndividualResult.result}\n`
      }

      outputText += '='.repeat(70) + '\n'

      setResults(benchmarkResults)
      setOutput(outputText)
    } catch (error) {
      setOutput(`Error running benchmark: ${error}`)
      console.error('Benchmark error:', error)
    } finally {
      setIsRunning(false)
    }
  }

  return (
    <div className="benchmark">
      <Link to="/" className="back-link">← Back to Menu</Link>
      <h1>Password Validator Benchmark</h1>
      
      <button
        onClick={runBenchmark}
        disabled={isRunning}
        className="benchmark-button"
      >
        {isRunning ? 'Running...' : 'Run Benchmark'}
      </button>

      {output && (
        <div className="benchmark-results">
          <pre>{output}</pre>
        </div>
      )}
    </div>
  )
}

export default Benchmark

