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


def benchmark_rust(passwords: List[str], rounds: int) -> tuple[float, int]:
    """Benchmark Rust validation via FFI"""
    validator = locksmith.PasswordValidator()
    
    start = time.perf_counter_ns()
    total_valid = validator.validate_passwords_score_repeated(passwords, rounds)
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
    
    # Warmup runs
    print("Warming up...")
    benchmark_python(test_passwords, 100)
    benchmark_rust(test_passwords, 100)
    print()
    
    # Benchmark Python
    print("Running Python benchmark...")
    python_time_ns, python_valid = benchmark_python(test_passwords, args.rounds)
    print(f"  Total time: {format_time(python_time_ns)}")
    print(f"  Time per operation: {format_time(python_time_ns // (args.rounds * len(test_passwords)))}")
    print(f"  Valid passwords found: {python_valid:,}")
    print()
    
    # Benchmark Rust
    print("Running Rust benchmark...")
    rust_time_ns, rust_valid = benchmark_rust(test_passwords, args.rounds)
    print(f"  Total time: {format_time(rust_time_ns)}")
    print(f"  Time per operation: {format_time(rust_time_ns // (args.rounds * len(test_passwords)))}")
    print(f"  Valid passwords found: {rust_valid:,}")
    print()
    
    # Compare results
    print("=" * 70)
    print("Comparison")
    print("=" * 70)
    speedup = python_time_ns / rust_time_ns if rust_time_ns > 0 else float('inf')
    
    if speedup > 1:
        print(f"✓ Rust is {speedup:.2f}x faster than Python")
    elif speedup < 1:
        print(f"⚠ Python is {1/speedup:.2f}x faster than Rust (unexpected)")
    else:
        print("= Performance is similar")
    
    if python_valid != rust_valid:
        print(f"⚠ WARNING: Result mismatch! Python: {python_valid}, Rust: {rust_valid}")
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



