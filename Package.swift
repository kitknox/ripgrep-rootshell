// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "ripgrep_ios",
    platforms: [
        .iOS(.v14),
        .macCatalyst(.v14),
        .visionOS(.v1),
    ],
    products: [
        .library(name: "ripgrep_ios", targets: ["ripgrep_ios"]),
    ],
    targets: [
        .binaryTarget(
            name: "ripgrep_ios",
            url: "https://github.com/kitknox/ripgrep-rootshell/releases/download/v0.1.0/ripgrep_ios.xcframework.zip",
            checksum: "df554a63cc9bb9aa2b852766146ad882e83e001bffce5301eadad33619e16691"
        ),
    ]
)
