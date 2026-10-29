# MXB Local

An independent personal fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app),
based on v0.20.6. The upstream MIT license and attribution are retained.

**Status: local.2 built successfully and passed its Windows build and security
checks. Local testing confirmed the OEM bike library, track descriptions and
popularity browsing.**

[Download the local.2 Windows build](https://github.com/Crab-Krab/mxb-local/actions/runs/40813594922/artifacts/12873995398)
and extract the ZIP. Close MXB Local, then run
`MXB Local_0.20.6-local.2_x64-setup.exe` to install or update it.
This Actions download is retained through November 28, 2026; keep a local copy.

[Successful build and test results](https://github.com/Crab-Krab/mxb-local/actions/runs/40813594922).
The exact Cargo and Bun lockfiles from the successful build are committed here.

Popularity uses the site's top 100 overall, filtered to the selected category.
A category with no entry in that list, such as ReShade at the time of testing,
returns no popular results; Newest and Oldest browse the full category. The source
now explains this in the empty-state message; that wording update is not included
in the local.2 installer linked above.

This fork rebuilds remote descriptions from a safe allowlist, adds a Content
Security Policy, requires patched TLS packages, protects saved credentials with
Windows DPAPI, and turns off automatic usage/game-memory reports. It has its own
name and settings folder and cannot update itself from the original publisher.

Native game-DLL downloads are blocked pending independent review. Some private
upstream features are unavailable. See [changes and limitations](LOCAL-FORK.md).

The **Build MXB Local for Windows** workflow under **Actions** produces an installer
only after its build and security checks pass. Builds from this fork are unsigned.
Do not use the original publisher's release downloads as a substitute for this fork.

[Original project documentation](README.upstream.md) describes the original app;
its download links and security claims do not describe this fork.
