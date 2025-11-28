import Foundation
import LockSmith

public struct Calculator {
    public init() {}

    /// Adds two unsigned 64-bit integers using the shared Rust implementation.
    public func add(_ lhs: UInt64, _ rhs: UInt64) -> UInt64 {
        UInt64(LockSmith.add(a: lhs, b: rhs))
    }

    /// Returns the absolute difference between two unsigned 64-bit integers.
    public func difference(_ lhs: UInt64, _ rhs: UInt64) -> UInt64 {
        UInt64(LockSmith.difference(a: lhs, b: rhs))
    }
}







