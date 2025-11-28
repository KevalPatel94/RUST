#!/usr/bin/env python3
"""
LockSmith Password Validator Benchmark

Compares pure Python password validation against Rust implementation via FFI.
"""

import sys
import time
import argparse
from typing import List

# Import the locksmith bindings
try:
    from . import locksmith
except ImportError:
    # Allow running as a script
    import locksmith


def validate_password_python(password: str) -> bool:
    """
    Pure Python implementation of password validation.
    This is used for comparison against the Rust implementation.
    """
    if len(password) < 8:
        return False
    if len(password) > 20:
        return False
    if not any(c.isupper() for c in password):
        return False
    if not any(c.islower() for c in password):
        return False
    if not any(c.isdigit() for c in password):
        return False
    if not any(c in "~!@#$%^&*()^&+=" for c in password):
        return False
    return True


def benchmark_python(passwords: List[str], rounds: int) -> tuple[float, int]:
    """Benchmark pure Python validation"""
    start = time.perf_counter_ns()
    total_valid = 0
    for _ in range(rounds):
        for password in passwords:
            if validate_password_python(password):
                total_valid += 1
    end = time.perf_counter_ns()
    elapsed_ns = end - start
    return elapsed_ns, total_valid


def benchmark_rust_batch(passwords: List[str], rounds: int) -> tuple[float, int]:
    """Benchmark Rust batch validation (Rust does 10,000 rounds internally)"""
    validator = locksmith.PasswordValidator()
    
    start = time.perf_counter_ns()
    total_valid = validator.validate_passwords_count_valid(passwords, rounds)
    end = time.perf_counter_ns()
    elapsed_ns = end - start
    return elapsed_ns, total_valid


def benchmark_rust_individual(passwords: List[str], rounds: int) -> tuple[float, int]:
    """Benchmark Rust validation with FFI overhead (10,000 individual calls)"""
    validator = locksmith.PasswordValidator()
    
    start = time.perf_counter_ns()
    total_valid = 0
    for _ in range(rounds):
        for password in passwords:
            result = validator.validate_with_message(password)
            if "valid" in result.lower():
                total_valid += 1
    end = time.perf_counter_ns()
    elapsed_ns = end - start
    return elapsed_ns, total_valid


def format_time(ns: int) -> str:
    """Format nanoseconds into readable time"""
    if ns < 1000:
        return f"{ns} ns"
    elif ns < 1_000_000:
        return f"{ns / 1000:.2f} μs"
    elif ns < 1_000_000_000:
        return f"{ns / 1_000_000:.2f} ms"
    else:
        return f"{ns / 1_000_000_000:.2f} s"


def main():
    parser = argparse.ArgumentParser(
        description="Benchmark password validation: Python vs Rust"
    )
    parser.add_argument(
        "--rounds",
        type=int,
        default=10000,
        help="Number of rounds to run (default: 10000)"
    )
    parser.add_argument(
        "--locale",
        default="en-US",
        choices=["en-US", "en", "es", "fr"],
        help="Locale for validation messages (default: en-US)"
    )
    args = parser.parse_args()
    
    # Initialize localization
    locksmith.initialize_localization()
    locksmith.set_app_locale(args.locale.split("-")[0])  # Use base locale code
    
    # Test passwords - mix of valid and invalid
    test_passwords = [
        "Ab1!abc",        # Valid
        "Ab1!",           # Too short
        "Abcdefghijklmnopqrstu1!",  # Too long
        "abc1!abc",       # No uppercase
        "ABC1!ABC",       # No lowercase
        "Abc!Abcd",       # No number
        "Abc1Abcd",       # No symbol
    ]
    
    print("=" * 70)
    print("LockSmith Password Validator Benchmark")
    print("=" * 70)
    print(f"Locale: {args.locale}")
    print(f"Rounds: {args.rounds:,}")
    print(f"Passwords per round: {len(test_passwords)}")
    print(f"Total validations: {args.rounds * len(test_passwords):,}")
    print("=" * 70)
    print()
    
    # Benchmark 1: Native Python (10,000 calls in Python)
    print("1. Native Python Implementation (10,000 calls in Python)")
    print("-" * 70)
    python_time_ns, python_valid = benchmark_python(test_passwords, args.rounds)
    print(f"  Total time: {format_time(python_time_ns)}")
    print(f"  Time per operation: {format_time(python_time_ns // (args.rounds * len(test_passwords)))}")
    print(f"  Valid passwords found: {python_valid:,}")
    print()
    
    # Benchmark 2: Rust Batch (Rust does 10,000 rounds internally)
    print("2. Rust Batch Function (10,000 rounds in Rust)")
    print("-" * 70)
    rust_batch_time_ns, rust_batch_valid = benchmark_rust_batch(test_passwords, args.rounds)
    print(f"  Total time: {format_time(rust_batch_time_ns)}")
    print(f"  Time per operation: {format_time(rust_batch_time_ns // (args.rounds * len(test_passwords)))}")
    print(f"  Valid passwords found: {rust_batch_valid:,}")
    print()
    
    # Benchmark 3: Rust Individual (10,000 FFI calls from Python)
    print("3. Rust Individual Calls (10,000 FFI calls from Python)")
    print("-" * 70)
    rust_individual_time_ns, rust_individual_valid = benchmark_rust_individual(test_passwords, args.rounds)
    print(f"  Total time: {format_time(rust_individual_time_ns)}")
    print(f"  Time per operation: {format_time(rust_individual_time_ns // (args.rounds * len(test_passwords)))}")
    print(f"  Valid passwords found: {rust_individual_valid:,}")
    print()
    
    # Compare results
    print("=" * 70)
    print("Performance Comparison")
    print("=" * 70)
    
    speedup_batch = python_time_ns / rust_batch_time_ns if rust_batch_time_ns > 0 else float('inf')
    speedup_individual = python_time_ns / rust_individual_time_ns if rust_individual_time_ns > 0 else float('inf')
    ffi_overhead = rust_individual_time_ns / rust_batch_time_ns if rust_batch_time_ns > 0 else float('inf')
    
    print(f"Rust Batch vs Python:     {speedup_batch:.2f}x {'faster' if speedup_batch > 1 else 'slower'}")
    print(f"Rust Individual vs Python: {speedup_individual:.2f}x {'faster' if speedup_individual > 1 else 'slower'}")
    print(f"FFI Overhead:             {ffi_overhead:.2f}x (Individual vs Batch)")
    print()
    
    if python_valid != rust_batch_valid or python_valid != rust_individual_valid:
        print(f"⚠ WARNING: Result mismatch!")
        print(f"  Python: {python_valid}, Rust Batch: {rust_batch_valid}, Rust Individual: {rust_individual_valid}")
    else:
        print(f"✓ Results match: {python_valid:,} valid passwords")
    
    print("=" * 70)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\nBenchmark interrupted.")
        sys.exit(1)
    except Exception as e:
        print(f"\nError: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)



