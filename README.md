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

## Private package access

Until this repository is public, the package source is cloned over SSH and the
release archive is downloaded over HTTPS. Configure a fine-grained GitHub token
with Contents read access for this repository in `~/.netrc`:

```netrc
machine github.com
  login token
  password <fine-grained-personal-access-token>
```

Protect the file with `chmod 600 ~/.netrc`. Do not commit credentials to this
repository. For command-line Xcode builds, select netrc explicitly with
`-packageAuthorizationProvider netrc`.

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

For each release, upload the generated ZIP under the matching semantic-version
tag and set `Package.swift` to the checksum reported in `release.md` (or by
`swift package compute-checksum`). The tag, release URL, archive, and manifest
checksum must all agree.
