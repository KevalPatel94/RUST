import XCTest
@testable import LockSmith

final class PasswordValidatorBenchmarkTests: XCTestCase {
    
    private let testPasswords = [
        "Abc1!abc",        // Valid
        "Ab1!",           // Too short
        "Abcdefghijklmnopqrstu1!",  // Too long
        "abc1!abc",       // No uppercase
        "ABC1!ABC",       // No lowercase
        "Abc!Abcd",       // No number
        "Abc1Abcd",       // No symbol
    ]
    
    private let rounds = 10_000
    
    // MARK: - Native Swift Implementation
    
    private func validatePasswordSwift(_ password: String) -> Bool {
        if password.count < 8 { return false }
        if password.count > 20 { return false }
        if !password.contains(where: { $0.isUppercase }) { return false }
        if !password.contains(where: { $0.isLowercase }) { return false }
        if !password.contains(where: { $0.isNumber }) { return false }
        if !password.contains(where: { "~!@#$%^&*()^&+=".contains($0) }) { return false }
        return true
    }
    
    private func benchmarkSwift() -> (time: TimeInterval, valid: Int) {
        let startTime = CFAbsoluteTimeGetCurrent()
        var validCount = 0
        
        for _ in 0..<rounds {
            for password in testPasswords {
                if validatePasswordSwift(password) {
                    validCount += 1
                }
            }
        }
        
        let elapsed = CFAbsoluteTimeGetCurrent() - startTime
        return (elapsed, validCount)
    }
    
    // MARK: - Rust Batch (Rust does 10,000 rounds internally)
    
    private func benchmarkRustBatch() -> (time: TimeInterval, valid: Int) {
        let validator = PasswordValidator()
        let startTime = CFAbsoluteTimeGetCurrent()
        
        let validCount = validator.validatePasswordsCountValid(inputs: testPasswords, rounds: UInt32(rounds))
        
        let elapsed = CFAbsoluteTimeGetCurrent() - startTime
        return (elapsed, Int(validCount))
    }
    
    // MARK: - Rust Individual (10,000 FFI calls from Swift)
    
    private func benchmarkRustIndividual() -> (time: TimeInterval, valid: Int) {
        let validator = PasswordValidator()
        let startTime = CFAbsoluteTimeGetCurrent()
        var validCount = 0
        
        for _ in 0..<rounds {
            for password in testPasswords {
                let result = validator.validateWithMessage(password: password)
                if result.lowercased().contains("valid") {
                    validCount += 1
                }
            }
        }
        
        let elapsed = CFAbsoluteTimeGetCurrent() - startTime
        return (elapsed, validCount)
    }
    
    func testPasswordValidatorBenchmark() {
        print(String(repeating: "=", count: 70))
        print("Password Validator Benchmark")
        print(String(repeating: "=", count: 70))
        print("Rounds: \(rounds.formatted())")
        print("Passwords per round: \(testPasswords.count)")
        print("Total validations: \((rounds * testPasswords.count).formatted())")
        print(String(repeating: "=", count: 70))
        print()
        
        // Benchmark 1: Native Swift
        print("1. Native Swift Implementation (10,000 calls in Swift)")
        print(String(repeating: "-", count: 70))
        let (swiftTime, swiftValid) = benchmarkSwift()
        let swiftPerOp = (swiftTime * 1_000_000_000) / Double(rounds * testPasswords.count)
        print(String(format: "  Total time: %.3f ms", swiftTime * 1000))
        print(String(format: "  Time per operation: %.2f ns", swiftPerOp))
        print("  Valid passwords found: \(swiftValid.formatted())")
        print()
        
        // Benchmark 2: Rust Batch
        print("2. Rust Batch Function (10,000 rounds in Rust)")
        print(String(repeating: "-", count: 70))
        let (rustBatchTime, rustBatchValid) = benchmarkRustBatch()
        let rustBatchPerOp = (rustBatchTime * 1_000_000_000) / Double(rounds * testPasswords.count)
        print(String(format: "  Total time: %.3f ms", rustBatchTime * 1000))
        print(String(format: "  Time per operation: %.2f ns", rustBatchPerOp))
        print("  Valid passwords found: \(rustBatchValid.formatted())")
        print()
        
        // Benchmark 3: Rust Individual
        print("3. Rust Individual Calls (10,000 FFI calls from Swift)")
        print(String(repeating: "-", count: 70))
        let (rustIndividualTime, rustIndividualValid) = benchmarkRustIndividual()
        let rustIndividualPerOp = (rustIndividualTime * 1_000_000_000) / Double(rounds * testPasswords.count)
        print(String(format: "  Total time: %.3f ms", rustIndividualTime * 1000))
        print(String(format: "  Time per operation: %.2f ns", rustIndividualPerOp))
        print("  Valid passwords found: \(rustIndividualValid.formatted())")
        print()
        
        // Comparison
        print(String(repeating: "=", count: 70))
        print("Performance Comparison")
        print(String(repeating: "=", count: 70))
        
        let speedupBatch = swiftTime / rustBatchTime
        let speedupIndividual = swiftTime / rustIndividualTime
        let ffiOverhead = rustIndividualTime / rustBatchTime
        
        print(String(format: "Rust Batch vs Swift:     %.2fx %@", speedupBatch, speedupBatch > 1 ? "faster" : "slower"))
        print(String(format: "Rust Individual vs Swift: %.2fx %@", speedupIndividual, speedupIndividual > 1 ? "faster" : "slower"))
        print(String(format: "FFI Overhead:             %.2fx (Individual vs Batch)", ffiOverhead))
        print()
        
        if swiftValid == rustBatchValid && swiftValid == rustIndividualValid {
            print("✓ Results match: \(swiftValid.formatted()) valid passwords")
        } else {
            print("⚠ WARNING: Result mismatch!")
            print("  Swift: \(swiftValid), Rust Batch: \(rustBatchValid), Rust Individual: \(rustIndividualValid)")
        }
        
        print(String(repeating: "=", count: 70))
        
        // Assert that benchmarks completed successfully
        XCTAssertGreaterThan(swiftTime, 0, "Swift benchmark should complete")
        XCTAssertGreaterThan(rustBatchTime, 0, "Rust batch benchmark should complete")
        XCTAssertGreaterThan(rustIndividualTime, 0, "Rust individual benchmark should complete")
    }
}

