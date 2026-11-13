# MXB Local — personal security fork

This is an independent Windows build of Frostn1/mxb-app v0.20.6. It keeps the
upstream MIT license and attribution. It is not an official Frost/mxbsecure release.

**Build status: local.4 passed the full Windows build: every Rust and frontend test,
typecheck, lint, both browser test suites, the Visual C++ runtime import check,
installer packaging and build attestation. It has not been installed and launched on
a clean Windows PC, and DPAPI recovery has not been exercised on a real profile.**

## What changes

- Remote descriptions are rebuilt from an allowlist, both in Rust and before the
  UI inserts them. Executable tags, event handlers, frames, unsafe links, and
  attacker-selected attributes are removed. A restrictive Content Security Policy
  provides an additional boundary.
- TLS dependencies have minimum patched versions. The build refreshes the selected
  packages only when necessary and refuses to proceed if the lockfile misses those
  minimums. The lockfiles from the successful Windows build are included.
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

## Unreleased

- Game Integration (FrostMod) is hidden throughout the app: the consent dialog that could
  open after setup, the top-bar status, its Settings section, the tour step and the runtime
  banner. Features that only work with FrostMod running are hidden too: paint sync, instant
  preset refresh, the protected-paints prompt and the server-browser repair. The code stays
  in place behind `GAME_INTEGRATION_SUPPORTED` in `apps/manager/src/lib/localFork.ts`.
- The app no longer asks GitHub for the latest FrostMod release at launch and every 30
  minutes. local.4 still did; `docs/privacy-local.md` now says so.
- Folder names that contain "FrostMod" (`FrostMod Models`, `FrostMod Sounds`,
  `FrostMod ReShade`) are unchanged, because existing installs and upstream use them.

## Changes in local.4

Work from the November 2026 technical audit.

- All checks run on every build: the full Rust workspace tests (about 1,900), the
  frontend unit tests, typecheck and lint. Two inherited tests that contradicted the
  fork's stricter rules were updated, and two frontend tests were brought up to date.
- A saved credential DPAPI can no longer open (password reset by an administrator,
  recreated profile, config copied from another PC) is treated as signed out. Before,
  it failed the whole config load and the app silently rebuilt default settings. A
  config that cannot be read is now copied to `config.json.bak` before anything
  replaces it.
- The C and C++ runtimes are linked statically. The app no longer imports
  `MSVCP140.dll` or `VCRUNTIME140.dll`, so it starts without the Visual C++
  redistributable. The build fails if those imports come back. The in-app
  redistributable installer, which ran a downloaded program elevated, is disabled.
- No request to the upstream service at startup, no upload of the server list while
  the Online screen is open, and no supporters-list fetch from GitHub. Friends
  presence is off by default. Description images on hosts outside the image-cache
  allowlist are removed. Requests identify as `mxb-local`. See
  [docs/privacy-local.md](docs/privacy-local.md).
- Archives may unpack to at most 20 times their own size, with a 4 GiB floor.
- The unused updater plugin, vendored MEGA client and orphaned voice sources are gone
  (23 fewer packages in `Cargo.lock`). rollup is updated and five unused frontend
  packages are removed.
- Settings no longer shows update and survey switches that did nothing.
- Releases are published from workflow inputs instead of hard-coded values, and each
  build carries a GitHub build-provenance attestation.
- Upstream-only documents are marked as upstream reference.

## Changes in local.3

- MXB Local branding with an upstream source credit.
- Clearer empty results when a category has no entries in the overall top 100.
- Built-in voice chat is excluded: no microphone capture, voice hotkeys, audio
  test commands, voice supervisor, codec or WebRTC dependencies are built. Shared
  account/server helpers remain for other optional features.
- Browser tests exercise the actual setup screen with simulated native APIs;
  Windows archive tests verify rejection of unsafe paths and links.

## Changes in local.2

- Complete app-only setup before opening the dashboard, for detected or manually
  selected folders. Six setup regressions cover ordering, persistence and failures.
- MXB popularity uses the site's current public top-100 overall ranking, narrowed
  by category and kept in the site's ranked order. An on-screen notice explains
  the limit. This replaces a removed REST endpoint that now returns 404. The
  rankings are parsed as data; their HTML is never inserted into the app UI.
- Ranked links are restricted to the site's HTTPS origin; malformed/executable
  links and oversized pages are rejected. Category metadata and pagination are
  checked, and cached results avoid repeated ranking requests while paging.

See [the local.4 dependency review](SECURITY-REVIEW-local.4.md) for the raw-scan
interpretation, affected-API analysis and remaining maintenance concerns.

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

## Download a published build

Use the [latest release](https://github.com/Crab-Krab/mxb-local/releases/latest)
for the Windows installer, SHA-256 checksum and license. Release assets are separate
from temporary Actions artifacts. This is the verified installer from the successful
Windows build, not a newly rebuilt application.

## Easiest build: GitHub

For another build, open **Actions → Build MXB Local for Windows → Run workflow**.
GitHub provides the Windows compiler, so no coding tools are needed on your PC.
A successful run provides an **MXB-Local-Windows** artifact
containing the installer and its SHA-256 hash. The installer is unsigned; do not
confuse an unsigned local build with the publisher-signed upstream installer.

The workflow has read-only repository permissions, uses no account secrets, does
not publish a release, and does not upload game files or personal settings. It
uploads build artifacts and lockfiles to your repository's Actions run. The build
job alone may also request a short-lived signing identity (`id-token`) to attach a
build-provenance attestation to the installer; pull request builds skip that step.

A separate, explicitly scoped publishing workflow uses repository write permission
to attach the verified build to a GitHub Release. It is run by hand with four inputs:
the build run ID, the release tag, the installer SHA-256 (shown in the build run's
summary) and the security review file. It checks the successful source build, archive
digest, installer digest and published asset digests, and takes release notes from
`release-notes/<tag>.md`. It does not build or execute the desktop app, or upload game
files or personal settings.

## Local build for maintainers

Install Rust stable (MSVC), the Visual Studio C++ build tools and Windows SDK,
Python 3.11+, Bun, and WebView2 as described by Tauri's official prerequisites.
Then run `scripts/build-local.ps1` from a Visual Studio developer PowerShell.

The build script verifies the checked-in TLS versions, refreshes only the selected
TLS packages if those checks fail, runs the frontend tests, typecheck and lint, builds
the frontend, runs the full Rust workspace test suite, then creates an NSIS installer. The GitHub workflow first runs browser
description and setup tests, and records dependency-review evidence.
The checked-in Cargo.lock and bun.lock are the versions from the verified build.

No audit or patch can guarantee that a large application has no remaining security
issues. A successful build and a smoke test with a disposable game profile are
required before using this fork for your regular mod collection.
