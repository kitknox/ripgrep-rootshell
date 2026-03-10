// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "ripgrep_ios",
    products: [
        .library(name: "ripgrep_ios", targets: ["ripgrep_ios"]),
    ],
    targets: [
        .binaryTarget(
            name: "ripgrep_ios",
            path: ".build/ripgrep_ios.xcframework"
        ),
    ]
)
