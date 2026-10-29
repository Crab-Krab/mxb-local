# MXB Local

An independent personal fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app),
based on v0.20.6. The upstream MIT license and attribution are retained.

**Status: local.1 built and passed 17 targeted security tests. Local testing
confirmed the OEM bike library and track descriptions after a setup repair.
local.2 adds permanent setup and popularity fixes; its Windows build is pending.**

[Download the earlier local.1 Windows build](https://github.com/Crab-Krab/mxb-local/actions/runs/40806541996/artifacts/12869723534)
and extract the ZIP. Run `MXB Local_0.20.6-local.1_x64-setup.exe` to install it.
This build needs the first-run setup repair; prefer local.2 once its checks pass.
This Actions download is retained through November 28, 2026; keep a local copy.

[Successful build and test results](https://github.com/Crab-Krab/mxb-local/actions/runs/40806541996).
The exact Cargo and Bun lockfiles from that build are committed in this fork.

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
