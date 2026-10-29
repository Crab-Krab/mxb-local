# MXB Local — personal security fork

This is an independent Windows build of Frostn1/mxb-app v0.20.6. It keeps the
upstream MIT license and attribution. It is not an official Frost/mxbsecure release.

**Preparation status: source changes prepared; a successful Windows build and
runtime smoke test are required before this should be called a usable installer.**

## What changes

- Remote descriptions are rebuilt from an allowlist, both in Rust and before the
  UI inserts them. Executable tags, event handlers, frames, unsafe links, and
  attacker-selected attributes are removed. A restrictive Content Security Policy
  provides an additional boundary.
- TLS dependencies have minimum patched versions. The build refreshes the selected
  packages and refuses to proceed if the resulting lockfile misses those minimums.
  The included upstream lockfile has not yet been resolved for these changes.
- Login cookies, the app account token, and saved server tokens use Windows DPAPI,
  bound to your Windows user. Encryption errors do not fall back to plaintext.
- The general settings response no longer exposes saved bearer tokens to the UI.
- Automatic game-memory/module reporting, automatic crash-report uploads, anonymous
  usage reporting, the machine-ID header, forced startup sign-in, and debugger
  refusal are disabled. Features you explicitly use can still contact their services.
- Updates from the original publisher are removed. Native game-DLL downloads and
  plugin installation commands are blocked until independently reviewed builds are
  available. Closing the app exits it by default.
- Name: **MXB Local**. Settings: `%LOCALAPPDATA%\local.mxb.manager`.
  It does not import or overwrite the original app's settings or cookies.

## Features and limits

Basic browsing, downloading, archive installation, library management, and existing
game-folder operations remain in the public source. This is an Internet-enabled
desktop mod manager, not an offline application. Stores, hosting services, and
Cloudflare can still change or refuse third-party clients.

Private secure-content code, some geometry/server-browser features, and the Shop
catalog API credential are not available in the public repository. The public Shop
purchase-login path and Hub functionality remain subject to their own services.
This fork does not bypass a store's purchase or content-entitlement checks.

The fork does not uninstall game plugins already present in your game folder.
Your game may still load those existing DLLs. They have not been audited here.
Optional cloud features still depend on the original service and may not work with
this fork. The fork cannot promise compatibility with every upstream feature.

## Easiest build: GitHub

After the fork is created under your account, open **Actions → Build MXB Local for
Windows → Run workflow**. GitHub provides the Windows compiler, so no coding tools
are needed on your PC. A successful run provides an **MXB-Local-Windows** artifact
containing the installer and its SHA-256 hash. The installer is unsigned; do not
confuse an unsigned local build with the publisher-signed upstream installer.

The workflow has read-only repository permissions, uses no account secrets, does
not publish a release, and does not upload game files or personal settings. It
uploads build artifacts and lockfiles to your repository's Actions run.

## Local build for maintainers

Install Rust stable (MSVC), the Visual Studio C++ build tools and Windows SDK,
Python 3.11+, Bun, and WebView2 as described by Tauri's official prerequisites.
Then run `scripts/build-local.ps1` from a Visual Studio developer PowerShell.

The build script refreshes only the selected TLS packages, builds the frontend,
runs the Windows credential-storage and backend security regressions, then creates
an NSIS installer. The GitHub workflow additionally runs browser rendering tests.
Review and commit the resolved Cargo.lock and bun.lock after a successful build.

No audit or patch can guarantee that a large application has no remaining security
issues. A successful build and a smoke test with a disposable game profile are
required before using this fork for your regular mod collection.
