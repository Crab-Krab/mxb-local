# MXB Local

An independent personal fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app),
based on v0.20.6. The upstream MIT license and attribution are retained.

**Status: local.3 is available as a regular Windows release.**

[Download the Windows installer](https://github.com/Crab-Krab/mxb-local/releases/download/v0.20.6-local.3/MXB.Local_0.20.6-local.3_x64-setup.exe)
from the [local.3 release](https://github.com/Crab-Krab/mxb-local/releases/tag/v0.20.6-local.3).
Close MXB Local, then run `MXB.Local_0.20.6-local.3_x64-setup.exe` to install or
update it. The release includes a checksum file and license. These release assets
are separate from temporary GitHub Actions artifacts and their 30-day retention.

[Successful build and test results](https://github.com/Crab-Krab/mxb-local/actions/runs/40850647284).
The exact Cargo and Bun lockfiles from the successful build are committed here.

Popularity uses the site's top 100 overall, filtered to the selected category.
A category with no entry in that list, such as ReShade at the time of testing,
returns no popular results; Newest and Oldest browse the full category. The source
explains this in the empty-state message.

This fork rebuilds remote descriptions from a safe allowlist, adds a Content
Security Policy, requires patched TLS packages, protects saved credentials with
Windows DPAPI, and turns off automatic usage/game-memory reports. It has its own
name and settings folder and cannot update itself from the original publisher.

Built-in voice chat is excluded, including microphone capture and WebRTC.
[Dependency review and remaining limitations](SECURITY-REVIEW-local.3.md) explain
the advisory findings; a completed scan does not certify zero vulnerabilities.

Native game-DLL downloads are blocked pending independent review. Some private
upstream features are unavailable. See [changes and limitations](LOCAL-FORK.md).

The **Build MXB Local for Windows** workflow under **Actions** produces an installer
only after its build and security checks pass. Builds from this fork are unsigned.
Do not use the original publisher's release downloads as a substitute for this fork.

[Original project documentation](README.upstream.md) describes the original app;
its download links and security claims do not describe this fork.


The local.2 acceptance test completed a track download, installation, in-game play,
uninstallation, and reinstall/upgrade successfully. Local.3 adds the MXB Local title bar, upstream credit and clearer empty popularity
results, removes built-in voice, and passes automated setup, description, credential
and archive-safety checks. The setup browser tests simulate native APIs; the
local.2 acceptance test is not a clean-profile native test of local.3.
