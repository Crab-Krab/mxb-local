# MXB Local

An independent personal fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app),
based on v0.20.6. The upstream MIT license and attribution are retained.

**Status: local.3 is undergoing final build and dependency checks before a regular release. The downloadable local.2 build remains available below.**

[Download the Windows installer](https://github.com/Crab-Krab/mxb-local/releases/download/v0.20.6-local.2/MXB.Local_0.20.6-local.2_x64-setup.exe)
from the [local.2 release](https://github.com/Crab-Krab/mxb-local/releases/tag/v0.20.6-local.2).
Close MXB Local, then run `MXB.Local_0.20.6-local.2_x64-setup.exe` to install or
update it. The release includes a checksum file and license. These release assets
are separate from temporary GitHub Actions artifacts and their 30-day retention.

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


The local.2 acceptance test completed a track download, installation, in-game play,
uninstallation, and reinstall/upgrade successfully. The next build includes the
MXB Local title bar, upstream credit and clearer empty popularity results, plus
browser tests of first-run setup and a current dependency-advisory review.
