package com.locksmith.example

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Test
import org.junit.runner.RunWith
import uniffi.locksmith.*
import org.junit.Assert.assertTrue

@RunWith(AndroidJUnit4::class)
class PasswordValidatorBenchmarkTest {
    
    companion object {
        init {
            // Ensure the native library is loaded before any tests run
            try {
                uniffiEnsureInitialized()
                initializeLocalization()
            } catch (e: Exception) {
                throw RuntimeException("Failed to initialize native library: ${e.message}", e)
            }
        }
    }
    
    private val testPasswords = listOf(
        "Abc1!abc",        // Valid
        "Ab1!",           // Too short
        "Abcdefghijklmnopqrstu1!",  // Too long
        "abc1!abc",       // No uppercase
        "ABC1!ABC",       // No lowercase
        "Abc!Abcd",       // No number
        "Abc1Abcd",       // No symbol
    )
    
    private val rounds = 10_000
    
    // MARK: - Native Kotlin Implementation
    
    private fun validatePasswordKotlin(password: String): Boolean {
        if (password.length < 8) return false
        if (password.length > 20) return false
        if (!password.any { it.isUpperCase() }) return false
        if (!password.any { it.isLowerCase() }) return false
        if (!password.any { it.isDigit() }) return false
        if (!password.any { "~!@#$%^&*()^&+=".contains(it) }) return false
        return true
    }
    
    private fun benchmarkKotlin(): Pair<Long, Int> {
        val startTime = System.nanoTime()
        var validCount = 0
        
        repeat(rounds) {
            testPasswords.forEach { password ->
                if (validatePasswordKotlin(password)) {
                    validCount++
                }
            }
        }
        
        val elapsed = System.nanoTime() - startTime
        return Pair(elapsed, validCount)
    }
    
    // MARK: - Rust Batch (Rust does 10,000 rounds internally)
    
    private fun benchmarkRustBatch(): Pair<Long, Int> {
        val validator = PasswordValidator()
        val startTime = System.nanoTime()
        
        val validCount = validator.validatePasswordsCountValid(testPasswords, rounds.toUInt())
        
        val elapsed = System.nanoTime() - startTime
        return Pair(elapsed, validCount.toInt())
    }
    
    // MARK: - Rust Individual (10,000 FFI calls from Kotlin)
    
    private fun benchmarkRustIndividual(): Pair<Long, Int> {
        val validator = PasswordValidator()
        val startTime = System.nanoTime()
        var validCount = 0
        
        repeat(rounds) {
            testPasswords.forEach { password ->
                val result = validator.validateWithMessage(password)
                if (result.contains("valid", ignoreCase = true)) {
                    validCount++
                }
            }
        }
        
        val elapsed = System.nanoTime() - startTime
        return Pair(elapsed, validCount)
    }
    
    @Test
    fun benchmarkPasswordValidation() {
        println("=".repeat(70))
        println("Password Validator Benchmark")
        println("=".repeat(70))
        println("Rounds: ${rounds.formatNumber()}")
        println("Passwords per round: ${testPasswords.size}")
        println("Total validations: ${(rounds * testPasswords.size).formatNumber()}")
        println("=".repeat(70))
        println()
        
        // Benchmark 1: Native Kotlin
        println("1. Native Kotlin Implementation (10,000 calls in Kotlin)")
        println("-".repeat(70))
        val (kotlinTime, kotlinValid) = benchmarkKotlin()
        val kotlinPerOp = kotlinTime / (rounds * testPasswords.size)
        println("  Total time: ${kotlinTime / 1_000_000.0} ms")
        println("  Time per operation: $kotlinPerOp ns")
        println("  Valid passwords found: ${kotlinValid.formatNumber()}")
        println()
        
        // Benchmark 2: Rust Batch
        println("2. Rust Batch Function (10,000 rounds in Rust)")
        println("-".repeat(70))
        val (rustBatchTime, rustBatchValid) = benchmarkRustBatch()
        val rustBatchPerOp = rustBatchTime / (rounds * testPasswords.size)
        println("  Total time: ${rustBatchTime / 1_000_000.0} ms")
        println("  Time per operation: $rustBatchPerOp ns")
        println("  Valid passwords found: ${rustBatchValid.formatNumber()}")
        println()
        
        // Benchmark 3: Rust Individual
        println("3. Rust Individual Calls (10,000 FFI calls from Kotlin)")
        println("-".repeat(70))
        val (rustIndividualTime, rustIndividualValid) = benchmarkRustIndividual()
        val rustIndividualPerOp = rustIndividualTime / (rounds * testPasswords.size)
        println("  Total time: ${rustIndividualTime / 1_000_000.0} ms")
        println("  Time per operation: $rustIndividualPerOp ns")
        println("  Valid passwords found: ${rustIndividualValid.formatNumber()}")
        println()
        
        // Comparison
        println("=".repeat(70))
        println("Performance Comparison")
        println("=".repeat(70))
        
        val speedupBatch = kotlinTime.toDouble() / rustBatchTime
        val speedupIndividual = kotlinTime.toDouble() / rustIndividualTime
        val ffiOverhead = rustIndividualTime.toDouble() / rustBatchTime
        
        println("Rust Batch vs Kotlin:     ${String.format("%.2f", speedupBatch)}x ${if (speedupBatch > 1) "faster" else "slower"}")
        println("Rust Individual vs Kotlin: ${String.format("%.2f", speedupIndividual)}x ${if (speedupIndividual > 1) "faster" else "slower"}")
        println("FFI Overhead:             ${String.format("%.2f", ffiOverhead)}x (Individual vs Batch)")
        println()
        
        if (kotlinValid == rustBatchValid && kotlinValid == rustIndividualValid) {
            println("✓ Results match: ${kotlinValid.formatNumber()} valid passwords")
        } else {
            println("⚠ WARNING: Result mismatch!")
            println("  Kotlin: $kotlinValid, Rust Batch: $rustBatchValid, Rust Individual: $rustIndividualValid")
        }
        
        println("=".repeat(70))
        
        // Assert that benchmarks completed successfully
        assertTrue("Kotlin benchmark should complete", kotlinTime > 0)
        assertTrue("Rust batch benchmark should complete", rustBatchTime > 0)
        assertTrue("Rust individual benchmark should complete", rustIndividualTime > 0)
    }
    
    private fun Int.formatNumber(): String = String.format("%,d", this)
    private fun Long.formatNumber(): String = String.format("%,d", this)
}
