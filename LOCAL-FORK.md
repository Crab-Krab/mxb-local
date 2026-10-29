# MXB Local — personal security fork

This is an independent Windows build of Frostn1/mxb-app v0.20.6. It keeps the
upstream MIT license and attribution. It is not an official Frost/mxbsecure release.

**Build status: local.2 built successfully and passed its Windows build and
security checks. Local testing confirmed OEM bikes in Library, track descriptions
and popularity browsing. ReShade has no entries in the tested top-100 overall
ranking; Newest and Oldest still show that full category. A later source-only
wording correction explains this case without claiming the category is empty.**

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

Use the [local.2 release](https://github.com/Crab-Krab/mxb-local/releases/tag/v0.20.6-local.2)
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
uploads build artifacts and lockfiles to your repository's Actions run.

A separate, explicitly scoped publishing workflow uses repository write permission
to attach the verified build to a GitHub Release. It checks the successful source
build, archive digest, installer digest and published asset digests. It does not
build or execute the desktop app, or upload game files or personal settings.

## Local build for maintainers

Install Rust stable (MSVC), the Visual Studio C++ build tools and Windows SDK,
Python 3.11+, Bun, and WebView2 as described by Tauri's official prerequisites.
Then run `scripts/build-local.ps1` from a Visual Studio developer PowerShell.

The build script verifies the checked-in TLS versions, refreshes only the selected
TLS packages if those checks fail, builds the frontend,
runs the Windows credential-storage and backend security regressions, then creates
an NSIS installer. The GitHub workflow additionally runs browser rendering tests.
The checked-in Cargo.lock and bun.lock are the versions from the verified build.

No audit or patch can guarantee that a large application has no remaining security
issues. A successful build and a smoke test with a disposable game profile are
required before using this fork for your regular mod collection.

