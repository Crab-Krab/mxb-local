# MXB Local local.3 release security review

Review date: October 29, 2026. Scope: Windows x64 public build. This is a source,
dependency-reachability and regression review, not a guarantee of no vulnerabilities.
The installer is unsigned. The review does not certify existing game plugins or
third-party mods, stores, downloads or upstream private code.

## Changes and acceptance evidence

- HTML descriptions are parsed through allowlists on both sides of the native boundary.
- Saved Windows secrets use user-bound DPAPI, and settings IPC responses omit tokens.
- The original binary updater, native game-DLL installation commands and automatic
  telemetry paths remain disabled in this fork.
- Built-in voice is excluded from compilation and native IPC. The microphone,
  audio tests, push-to-talk, codec, WebRTC transport and voice supervisor are absent.
  Shared account and server-name helpers remain for unrelated optional features.
- The lockfile no longer contains cpal, opus, str0m or crypto-bigint. The Windows
  normal-dependency graph also checks that voice dependencies are absent.
- The user verified track download/install, in-game play, uninstall and installer
  replacement with local.2. These results are not a clean-profile native test of local.3.
- [Final Windows build](https://github.com/Crab-Krab/mxb-local/actions/runs/40850647284)
  succeeded for commit `8a59578a47841be2c7d51ee889417f310c9b50ab`.
  All 27 selected native tests passed: DPAPI 2, backend security 11, setup readiness
  2 and guarded ZIP/7z/RAR extraction 12. Six setup helper tests, the description
  renderer checks and three setup browser scenarios also passed.
  Setup browser tests exercise the real React component with simulated native APIs;
  they do not launch the packaged executable or modify real game files.

## Dependency findings and applicability

The raw OSV report intentionally retains findings for other platforms and build
packages. A successful report-collection job means the report was collected, not
that there were zero advisories. The Windows normal-dependency graph contains 459
package/version pairs. Version matches alone are not evidence of a reachable exploit.

| Finding | Review of this build |
| --- | --- |
| crypto-bigint / voice DTLS | The optional voice implementation and its dependency chain were removed. crypto-bigint 0.5.5 is absent from the resolved lockfile. |
| quinn-proto peer-handling advisories | QUIC/HTTP3 dependencies remain in the all-platform/all-feature lock inventory but are absent from the Windows normal-dependency graph. |
| block-buffer GHSA-qwgh-2vcv-g2f7 | The maintainer identifies the affected inline-buffer implementation as versions 0.11.0 through before 0.12.1. This graph uses 0.10.4, whose position is a separate field. In addition, production uses panic=abort. The raw OSV match is retained for transparency. |
| rsa RUSTSEC-2023-0071 | The app constructs unauthenticated MEGA clients only for public shared links. No MEGA account login/private-key operation is exposed by the app. The dependency uses RSA BigUint types; the reported private-key timing oracle was not found reachable through the public-link path. This does not certify the library's unused account-login functionality. |
| sevenz-rust RUSTSEC-2026-0245 | Mod extraction uses SevenZReader with app-controlled output creation, not the vulnerable decompress convenience helpers. Names are checked before any extraction and again in the callback; links/reparse points and unsafe paths are refused. Archive regression tests cover hostile names and links plus benign extraction. The package is still unmaintained. |
| zip GHSA-m665-v7pm-7grj | The guarded mod-install path validates central-directory names and uses the same ZipArchive and saved destinations when decoding. It does not validate with ZipArchive then extract local headers with ZipStreamReader. No affected parser-differential extraction path was found. |
| zip RUSTSEC-2026-0367/0368/0369 | Application extraction creates checked files itself, does not call ZipArchive::extract / ZipStreamReader::extract, and does not restore archive Unix permission bits. ZIP links are rejected. The second zip version belongs to the disabled updater dependency; updater registration is absent. |
| zstd RUSTSEC-2026-0391 | zip 2.4.2 uses the read Decoder with_buffer constructor, which creates a fresh owned context. No reuse through the affected borrowed with_context constructors was found. |
| zstd-safe RUSTSEC-2026-0403 | The maintainer explicitly excludes stream::read::Decoder. ZIP uses that reader; the reported uninitialized Vec output affects the write decoder. |
| zstd-safe advanced API advisories 0395-0402 and 0404-0406 | No app calls to seekable, experimental dictionary constructors, training, context cloning, stable input buffers or mixed streaming/one-shot context reuse were found. ZIP's reader owns a fresh context and supplies initialized output slices. These findings remain in the raw report. |
| zstd-sys RUSTSEC-2026-0410 | The reported allocator is specific to WebAssembly; this release targets Windows x64. |
| tokio RUSTSEC-2026-0386 | The reported AsyncFd API is Unix-specific and is not the Windows runtime path. |
| Unmaintained dependencies | fxhash and the unic family remain transitive Windows dependencies. This is a maintenance concern; no exploit is implied by the unmaintained label alone. Linux-only glib/zbus/zvariant entries and build-only proc-macro-error are retained in the full inventory. |
| Frontend/development dependencies | Rollup recorded 572 production chunk modules (570 with rendered code). No fflate, zod, nanoid, source-map-js, Vite, Rollup, esbuild, js-yaml, brace-expansion, baseline-browser-mapping or browserslist module was present. The associated reports concern omitted runtime code or build tools. Output uses ES modules, not the affected Rollup IIFE/UMD URL gadget. This does not approve exposing a development server to untrusted networks. |

The archive guards mitigate the listed high-level extraction problems; they are not
a claim that every parser is memory-safe or that arbitrary downloaded mods are safe.
Future changes to extraction, MEGA login, QUIC, voice, updater or frontend loading
must reopen the corresponding review instead of reusing these conclusions.

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

Registry source copies reviewed locally were verified against the exact Cargo.lock
SHA-256 checksums. Full raw advisory reports and the Windows dependency graph are retained
in the build's dependency-review artifact. Frontend export evidence is generated
by Rollup from the production build rather than inferred from package.json alone.

## Installer verification

The installer is `MXB Local_0.20.6-local.3_x64-setup.exe` (7,601,272 bytes). Its SHA-256 is
`956cd87a909388b749bd842de8b53e4117fffd74c0dba40559d91426b29909a4`. The published asset uses a dot in place of the space.

The artifact digest was verified before opening its archive. The installer was
extracted without executing it; its embedded application identifies itself as
MXB Local 0.20.6-local.3 for Windows x64, is unsigned, and contains no private
game DLL or injector. Cargo.lock and bun.lock match the reviewed source. The
final packaged frontend inventory has the same module IDs and export metadata
as the passed frontend-check job; array ordering differs between those two builds.

The original Actions ZIP also includes a cached local.2 installer. The publisher
selects the exact local.3 filename and independently pins its digest. A subsequent
workflow-only fix selects the configured version when creating future artifacts.
Release attachments persist the finding inventory, affected-version data and source
references; full advisory prose and proof-of-concept code are omitted there.
