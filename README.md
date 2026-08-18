# ripgrep_ios

`ripgrep_ios` packages ripgrep 15.1.0 as a dynamic XCFramework for rootshell. The
Swift package exposes the binary product `ripgrep_ios`; the C entry point used by
the app is `rg_main`.

## Supported platforms

- iOS 14 or later (arm64 device and arm64 simulator)
- Mac Catalyst 14 or later (arm64 and x86_64)
- visionOS 1 or later (arm64 device and arm64 simulator)

The framework links `ios_system.framework` at runtime. Consumers must also link
and embed a compatible ios_system build.

## Installation

Add this repository as a Swift package dependency:

```text
https://github.com/kitknox/ripgrep-rootshell.git
```

Select the `ripgrep_ios` library product. Releases use standard public GitHub
asset URLs, so consumers do not need a GitHub token or `.netrc` configuration.

## Building a release

The build expects sibling checkouts named `ripgrep` and `ios_system` under the
same parent directory. Build ios_system first, then run from this repository:

```sh
swift run --package-path xcfs build
```

The command builds all supported slices and writes these ignored artifacts:

- `.build/ripgrep_ios.xcframework`
- `.build/ripgrep_ios.xcframework.zip`
- `.build/release.md`

For each release, set the versioned GitHub release-download URL in
`Package.swift` and use the checksum reported in `release.md` (or by
`swift package compute-checksum`). Commit the manifest, tag that commit, and
publish a GitHub release with `ripgrep_ios.xcframework.zip` attached.
