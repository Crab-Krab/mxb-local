# MXB Local

A Windows desktop app for browsing, installing and managing MX Bikes mods, with improvements to security and privacy.

**[Download for Windows](https://github.com/Crab-Krab/mxb-local/releases/download/v0.20.6-local.3/MXB.Local_0.20.6-local.3_x64-setup.exe)** · [Latest release](https://github.com/Crab-Krab/mxb-local/releases/latest) · [Security review](SECURITY-REVIEW-local.3.md)

Windows x64 · Current version: **0.20.6-local.3**

## Features

- Browse tracks, bikes, rider gear and ReShade presets.
- Download, install and uninstall mods from the app.
- Manage installed content in your library.
- Launch MX Bikes directly from the app.

MXB Local runs on your PC and uses the internet to access mod catalogs, downloads and optional online services.

## Getting started

### First-time installation

1. Make sure MX Bikes is installed on your PC.
2. Download the **Windows installer** using the link above.
3. Open `MXB.Local_0.20.6-local.3_x64-setup.exe` and follow the installation wizard.
4. Launch **MXB Local** and choose **MX Bikes** on the setup screen.
5. Let the app detect your folders automatically. If prompted, select the requested folders, then start browsing mods.

Use the **.exe installer** to install the app. GitHub's **Source code** archives are for working with the source.

### Updating an existing installation

1. Close MXB Local.
2. Download the installer from the [latest release](https://github.com/Crab-Krab/mxb-local/releases/latest).
3. Run it to replace your current installation.

You do not need to uninstall MXB Local first. Updates reuse your settings and existing mod folders. Updates are installed manually.

## Security and privacy

This fork includes:

- Sanitized mod descriptions and a Content Security Policy.
- Saved credentials encrypted for your Windows user account.
- Updated TLS dependencies.
- Automatic usage and game-memory reporting disabled.
- Built-in voice chat removed, including microphone capture and WebRTC.
- Updates from the original publisher and native game-DLL downloads blocked.

MXB Local uses its own settings folder, separate from the original app. Some private upstream integrations are unavailable, and online features depend on their external services.

The Windows installer is **unsigned**. See the [security review](SECURITY-REVIEW-local.3.md) for the scope of the checks, dependency findings and remaining limitations.

## Catalog sorting

For **mxb-mods**, **Most popular** uses the site's top 100 overall, filtered to your selected category. A category can have no entries in that list.

Choose **Newest** or **Oldest** to browse the full category.

## Documentation

- [Fork details and build instructions](LOCAL-FORK.md)
- [Security review](SECURITY-REVIEW-local.3.md)
- [Build and test results for local.3](https://github.com/Crab-Krab/mxb-local/actions/runs/40850647284)
- [Original project documentation](README.upstream.md)

## Credits and license

An independent fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app), based on v0.20.6.

Distributed under the [MIT License](LICENSE), with upstream copyright and attribution retained.
