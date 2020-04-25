![logo](/docs/images/logo.png)

### the missing macOS app notarization helper, built with Rust

# About

[Notarizing a macOS app](https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution) involves a series of manual steps, including zipping a bundle, uploading it to to Apple, and polling the notarization service.

`xcnotary` automates these steps for you. It:

- Attempts to fail fast if necessary, performing several checks on your target before uploading it to Apple.
- Zips the input if it is an .app bundle.
- Submits the input to the notarization service, and polls until completion. This step typically takes a few minutes.
- In case of success, attaches the notarization ticket to the target, enabling the app to pass Gatekeeper on first run even without an Internet connection.
- In case of failure, fetches the error log from Apple and outputs it to `stderr`.
- Return a zero/non-zero code for easy CI integration.

![Notarization](/docs/images/notarize.png)

*Screencap sped up for brevity. The service takes several minutes to notarize your upload.*

# Installation

### Homebrew

```sh
# Install
brew install akeru-inc/tap/xcnotary

# Upgrade
brew update
brew upgrade akeru-inc/tap/xcnotary
```

# Usage

To perform various code signing checks on the input without submitting:

```sh
xcnotary precheck <input path>
```

To perform code signing checks, submit to the notarization service, and block waiting for response:

```sh
xcnotary notarize <input path> \
  --developer-account <Apple Developer account> \
  --developer-password-keychain-item <name of keychain item, see below> \
  [--provider <provider short name>]
```

Supported inputs:

- ✅ .app bundles
- ✅ .dmg disk images
- ✅ .pkg installer packages

### Specifying the password keychain item

This tool does not handle your Apple Developer password. Instead, Xcode's helper `altool` reads an app-specific Apple Developer ID password directly from the keychain. See [the documentation](https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow#3087734) for `xcrun altool --store-password-in-keychain-item` to set up a suitable keychain item.

### Specifying the developer team

The optional `--provider` argument should be specified if the developer account is associated with more than one team. This value can be obtained by running the following command and noting the "ProviderShortname" displayed.

```sh
xcrun altool --list-providers  -u "$DEVELOPER_ACCOUNT_USERNAME" -p "@keychain:$PASSWORD_KEYCHAIN_ITEM"
```

### Required network access

- Xcode's `altool` will connect to several Apple hosts as outlined in [the documentation](https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow).

- When notarization fails, `xcnotary` will connect to `https://osxapps-ssl.itunes.apple.com/` on port 443 to retrieve the failure log.

# Bundle pre-checks

`xcnotary` attempts to check the input for some [common notarization issues](https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/resolving_common_notarization_issues) before uploading it to Apple. While not foolproof, these checks may potentially save you minutes waiting for a response only to fail due to an incorrect code signing flag.

![Bundle pre-check](/docs/images/precheck.png)

When the input is an app bundle, the following checks will be performed:

- ✅ Bundle being signed with a Developer ID certificate and not containing unsigned items.
- ✅ Bundle being signed with a secure timestamp.
- ✅ Bundle *not* having the get-task-allow entitlement.
- ✅ Bundle having hardened runtime enabled.

When the input is a *.dmg* or a *.pkg*, only the Developer ID signing check is performed, i.e. the only check that can be performed at the moment without extracting the contents. In your workflow, you may want to run `xcnotary precheck` on your bundle target before packaging it.

# Building for notarization

The following examples set various necessary build flags, such as code signing with a "secure timestamp."

### Bundles

```sh
xcodebuild \
   -target <target> \
   -scheme <scheme> \
   -configuration Release \
   -derivedDataPath .xcodebuild \
   "CODE_SIGN_IDENTITY=Developer ID Application: <team name>" \
   "OTHER_CODE_SIGN_FLAGS=--timestamp --options=runtime" \
   CODE_SIGN_INJECT_BASE_ENTITLEMENTS=NO \
   CODE_SIGN_STYLE=Manual
```

`CODE_SIGN_IDENTITY` should match the corresponding Keychain certificate.

Note that `--options=runtime` will have the effect of opting in your binary to the hardened runtime environment. You most likely want to first manually enable the "Hardened Runtime" capability in Xcode's target settings > "Signing and Capabilities" and make sure your application functions as expected. There, you may also add any entitlements to relax the runtime restrictions.

### Packages

```sh
pkgbuild \
   --component <path to bundle built according to above specs>
   --sign "Developer ID Installer: <team name>" \
   --timestamp \
   <output_pkg_name.pkg>
```

### Disk images

Codesign after creating the DMG:

```sh
codesign -s "Developer ID Application: <team>" <dmg>
```

# Additional Information

- [Change Log](CHANGELOG.md)

- Feature requests/comments/questions? Write: david@akeru.com
