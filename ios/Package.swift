// swift-tools-version: 6.0

import PackageDescription

let package = Package(
    name: "GcodeKit",
    platforms: [
        .iOS(.v17),
        .macOS(.v14),
    ],
    products: [
        .library(
            name: "GcodeKit",
            targets: ["GcodeKit"]
        ),
    ],
    targets: [
        .target(
            name: "GcodeKit",
            path: "Sources/GcodeKit"
        ),
        .executableTarget(
            name: "GcodeKitTests",
            dependencies: ["GcodeKit"],
            path: "Tests/GcodeKitTests"
        ),
    ]
)
