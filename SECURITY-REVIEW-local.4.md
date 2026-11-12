# MXB Local local.4 release security review

Review date: November 12, 2026. Scope: Windows x64 public build. This is a source,
dependency-reachability and regression review, not a guarantee of no vulnerabilities.
The installer is unsigned but carries a GitHub build-provenance attestation. The review
does not certify existing game plugins or third-party mods, stores, downloads or upstream
private code.

local.4 follows an independent technical audit of local.3 (commit `fe10110`), which found
no critical or high-severity vulnerability. This release addresses its findings.

## Changes and acceptance evidence

Carried over from local.3, unchanged: allowlist sanitizing of HTML descriptions on both
sides of the native boundary, a restrictive Content Security Policy, DPAPI for saved
secrets, tokens omitted from settings IPC responses, the updater and native game-DLL
installation disabled, and built-in voice excluded.

New in local.4:

- **Release gate.** The build runs `cargo test --locked --workspace`, `bun test`,
  `bun run typecheck` and `bun run lint`. local.3's gate ran 27 selected native tests.
  The two inherited tests that contradicted the fork (the IPC guard and the data-folder
  sharing check) and two stale frontend tests were updated. The sharing check change is a
  behaviour fix: "delete my data" was refused when an upstream app was installed.
- **Credential failure.** A secret DPAPI cannot open is cleared and logged instead of failing
  the whole config load, which made the app rebuild default settings over the file.
  Sealing is unchanged: encryption failure is still an error and nothing is written in
  plaintext. An unreadable config is copied to `config.json.bak` before any save.
- **Native runtime.** The Windows target is built with `crt-static`. The build fails if
  the executable imports `MSVCP*`, `VCRUNTIME*` or `CONCRT*`. The remaining
  `api-ms-win-crt-*` imports are the Universal C Runtime, part of Windows 10 and later.
  `frostmod_install_runtime`, which downloaded `vc_redist.x64.exe` and ran it elevated
  after only a size and `MZ` check, is gated behind `ALLOW_NATIVE_DOWNLOADS` (false).
- **Upstream contact.** The startup roster request is removed. Roster uploads
  (`POST /v1/roster`, `/v1/roster/snapshot`), which the audit did not list, are gated
  behind `ALLOW_ROSTER_CONTRIBUTIONS` (false). The supporters fetch from
  raw.githubusercontent.com is removed, along with that host in the CSP `connect-src`.
  `friends_presence` defaults to false. User-Agent is `mxb-local`.
  `docs/privacy-local.md` lists the remaining contacts, including the mirror lookup on
  api.mxbsecure.com when installing an MX Bikes mod from mxb-mods.com.
- **Description images.** Images on hosts outside the image-cache allowlist are removed
  from shop and hub descriptions instead of loading directly. The CSP `img-src` was not
  narrowed: other parts of the app load images from more hosts, and that change could not
  be tested here.
- **Archive size.** Extraction is capped at 20 times the archive size, with a 4 GiB
  floor. zip and 7z check declared sizes before writing and cap each copy, since declared
  sizes can lie. rar checks declared sizes and counts each written file.
- **Dependency surface.** `tauri-plugin-updater` (never registered), the unlinked
  `vendor/mega` workspace member and nine undeclared voice sources were removed: 23
  packages left `Cargo.lock`, including `zip 4.6.1`, `tar` and `minisign-verify`.
  rollup 4.20.0 → 4.66.0 (clears GHSA-mw96-cpmx-2vgc), and five unused frontend packages
  were removed.
- **Release process.** The publish workflow takes the build run, tag, installer SHA-256
  and review file as inputs. It derives the source commit and artifact digests from the
  run and checks them. Inputs reach scripts only through environment variables, after
  pattern checks. The build attests the installer with `actions/attest-build-provenance`,
  pinned by commit.

Test evidence: on Linux, mxb-app 1,129 and mxb-core 752 tests passed with no failures,
plus `bun test` (144), typecheck, lint (0 errors), both headless-browser suites and the
secure-boundary and GUID checks. On Windows,
[run 43170251678](https://github.com/Crab-Krab/mxb-local/actions/runs/43170251678) passed
every step for the reviewed branch. The release build is the run on `main` after merge,
linked from the release notes.

Not verified: installing and launching on a clean Windows PC, and DPAPI recovery on a real
profile after a password reset.

## Dependency findings and applicability

The raw OSV report from the build's dependency-review artifact retains findings for other
platforms and build packages. Version matches alone are not evidence of a reachable
exploit. A RustSec-only match against `Cargo.lock` (database commit `fd6f398`,
2026-11-11) gives 39 matches, down from 45 in local.3.

| Finding | Review of this build |
| --- | --- |
| crypto-bigint / voice DTLS | The optional voice implementation and its dependency chain were removed. crypto-bigint 0.5.5 is absent from the resolved lockfile. |
| quinn-proto peer-handling advisories | QUIC/HTTP3 dependencies remain in the all-platform/all-feature lock inventory but are absent from the Windows normal-dependency graph. |
| block-buffer GHSA-qwgh-2vcv-g2f7 | The maintainer identifies the affected inline-buffer implementation as versions 0.11.0 through before 0.12.1. This graph uses 0.10.4, whose position is a separate field. In addition, production uses panic=abort. The raw OSV match is retained for transparency. |
| rsa RUSTSEC-2023-0071 | The app constructs unauthenticated MEGA clients only for public shared links. No MEGA account login/private-key operation is exposed by the app. The dependency uses RSA BigUint types; the reported private-key timing oracle was not found reachable through the public-link path. This does not certify the library's unused account-login functionality. |
| sevenz-rust RUSTSEC-2026-0245 | Mod extraction uses SevenZReader with app-controlled output creation, not the vulnerable decompress convenience helpers. Names are checked before any extraction and again in the callback; links/reparse points and unsafe paths are refused. Archive regression tests cover hostile names and links plus benign extraction. The package is still unmaintained. |
| zip GHSA-m665-v7pm-7grj | The guarded mod-install path validates central-directory names and uses the same ZipArchive and saved destinations when decoding. It does not validate with ZipArchive then extract local headers with ZipStreamReader. No affected parser-differential extraction path was found. |
| zip RUSTSEC-2026-0367/0368/0369 | Application extraction creates checked files itself, does not call ZipArchive::extract / ZipStreamReader::extract, and does not restore archive Unix permission bits. ZIP links are rejected. The second zip version (4.6.1), from the unused updater plugin, is no longer in the lockfile. |
| zstd RUSTSEC-2026-0391 | zip 2.4.2 uses the read Decoder with_buffer constructor, which creates a fresh owned context. No reuse through the affected borrowed with_context constructors was found. |
| zstd-safe RUSTSEC-2026-0403 | The maintainer explicitly excludes stream::read::Decoder. ZIP uses that reader; the reported uninitialized Vec output affects the write decoder. |
| zstd-safe advanced API advisories 0395-0402 and 0404-0406 | No app calls to seekable, experimental dictionary constructors, training, context cloning, stable input buffers or mixed streaming/one-shot context reuse were found. ZIP's reader owns a fresh context and supplies initialized output slices. These findings remain in the raw report. |
| zstd-sys RUSTSEC-2026-0410 | The reported allocator is specific to WebAssembly; this release targets Windows x64. |
| tokio RUSTSEC-2026-0386 | The reported AsyncFd API is Unix-specific and is not the Windows runtime path. |
| Unmaintained dependencies | fxhash and the unic family remain transitive Windows dependencies. This is a maintenance concern; no exploit is implied by the unmaintained label alone. Linux-only glib/zbus/zvariant entries and build-only proc-macro-error are retained in the full inventory. |
| Frontend/development dependencies | In the local.3 build, Rollup recorded 572 production chunk modules (570 with rendered code). No fflate, zod, nanoid, source-map-js, Vite, Rollup, esbuild, js-yaml, brace-expansion, baseline-browser-mapping or browserslist module was present. The associated reports concern omitted runtime code or build tools. Output uses ES modules, not the affected Rollup IIFE/UMD URL gadget. This does not approve exposing a development server to untrusted networks. local.4 changed only build tooling here (rollup, removed unused packages); the build regenerates the module inventory as `frontend-runtime-modules.json`. |
| zip RUSTSEC-2026-0429 | Denial of service in `ZipArchive::extract` through symlinks. No Rust source calls `ZipArchive::extract`, `ZipStreamReader` or `sevenz_rust::decompress`. The `.extract` calls in `bundle.rs` and `fileshare.rs` are the app's own `Fetched::extract`, which goes through the guarded `install::extract_archive` / `extract_zip_from`. ZIP links are rejected before anything is written. Not reachable. |
| rustix RUSTSEC-2026-0486 | Uninitialized memory in `getxattr` and related calls. Extended attributes are a Unix interface; not part of the Windows build. |
| sevenz-rust RUSTSEC-2026-0246 (unmaintained) | Still used for 7z extraction behind the archive guard. Its maintained successor `sevenz-rust2` should be evaluated as a separate, tested change. |

The archive guards mitigate the listed high-level extraction problems; they are not
a claim that every parser is memory-safe or that arbitrary downloaded mods are safe.
Future changes to extraction, MEGA login, QUIC, updater or frontend loading must reopen
the corresponding review instead of reusing these conclusions.

## Primary references

- [Voice cryptography advisory](https://rustsec.org/advisories/RUSTSEC-2026-0387.html)
- [block-buffer maintainer advisory](https://github.com/RustCrypto/utils/security/advisories/GHSA-qwgh-2vcv-g2f7)
- [RSA timing advisory](https://rustsec.org/advisories/RUSTSEC-2023-0071.html)
- [7z extraction advisory](https://rustsec.org/advisories/RUSTSEC-2026-0245.html)
- [ZIP maintainer parser-differential advisory](https://github.com/zip-rs/zip2/security/advisories/GHSA-m665-v7pm-7grj)
- [ZIP 2.4.2 decoder source](https://github.com/zip-rs/zip2/blob/v2.4.2/src/compression.rs)
- [zstd context-reuse advisory](https://rustsec.org/advisories/RUSTSEC-2026-0391.html)
- [zstd write-decoder advisory](https://rustsec.org/advisories/RUSTSEC-2026-0403.html)
- [zstd WebAssembly advisory](https://rustsec.org/advisories/RUSTSEC-2026-0410.html)
- [ZIP extract denial-of-service advisory](https://rustsec.org/advisories/RUSTSEC-2026-0429.html)
- [rustix xattr advisory](https://rustsec.org/advisories/RUSTSEC-2026-0486.html)

## Installer verification

The publish workflow verifies the build run, the archive digest and the installer
SHA-256 against the value entered by the publisher, and checks every uploaded release
asset's digest. The installer hash is printed in the release notes and in
`SHA256SUMS.txt`. Anyone can check an installer's origin with
`gh attestation verify <installer> --repo Crab-Krab/mxb-local`.
