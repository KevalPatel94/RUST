// swift-tools-version: 5.10

import PackageDescription

let package = Package(
    name: "LockSmith",
    platforms: [
        .iOS(.v15),
        .macOS(.v12),
    ],
    products: [
        .library(name: "Calculator", targets: ["Calculator", "LibLockSmith"]),
        .library(name: "LockSmith", targets: ["LockSmith", "Calculator"]),
    ],
    dependencies: [
    ],
    targets: [
        .binaryTarget(
            name: "LibLockSmith",
            path: "Frameworks/locksmithFFI.xcframework"
        ),
        .target(
            name: "Calculator",
            dependencies: [
                "LockSmith"
            ],
            resources: []
        ),
        .target(
            name: "LockSmith",
            dependencies: [
                "LibLockSmith",
            ],
            path: "Sources/LockSmith"
        ),
        .testTarget(
            name: "MySampleBenchmarks",
            dependencies: [
                "LockSmith",
                "Calculator"
            ]
        ),
    ]
)
