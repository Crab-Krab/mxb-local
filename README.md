# MXB Local

An independent fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app), focused on security, privacy and Windows setup fixes.

**[Download for Windows](https://github.com/Crab-Krab/mxb-local/releases/download/v0.20.6-local.4/MXB.Local_0.20.6-local.4_x64-setup.exe)** · [Latest release](https://github.com/Crab-Krab/mxb-local/releases/latest) · [Security review](SECURITY-REVIEW-local.4.md)

Windows x64 (Windows 10 or later) · Current version: **0.20.6-local.4**

## Changes in this fork

- Sanitized mod descriptions and a Content Security Policy.
- Saved credentials encrypted for your Windows user account.
- Updated TLS dependencies.
- Automatic usage and game-memory reporting disabled. Nothing is sent to the original developer's service at startup, and the server list you see is never uploaded. See [what the app contacts](docs/privacy-local.md).
- Images in mod descriptions load only from the mod sites themselves, so a mod author can't see who viewed their page.
- Built-in voice chat removed, including microphone capture and WebRTC.
- Updates from the original publisher, native game-DLL downloads and the runtime installer blocked.
- No separate Microsoft Visual C++ package needed. The app includes the parts it uses.
- If a saved login can't be decrypted (for example after a Windows password reset), you are signed out of that one service instead of losing your settings.
- Archives that would unpack to an unreasonable size are refused instead of filling your disk.
- Fixed first-run setup completion for automatically detected and manually selected folders.
- Restored mxb-mods popularity browsing and clarified empty category results.

MXB Local uses its own settings folder, separate from the original app. Some private upstream integrations are unavailable, and online features depend on their external services.

The Windows installer is **unsigned**. Each release build carries a GitHub attestation, so you can check an installer came from this repository's build with `gh attestation verify <installer> --repo Crab-Krab/mxb-local`. See the [security review](SECURITY-REVIEW-local.4.md) for the scope of the checks, dependency findings and remaining limitations.

## Getting started

### First-time installation

1. Make sure MX Bikes is installed on your PC.
2. Download the **Windows installer** using the link above.
3. Open `MXB.Local_0.20.6-local.4_x64-setup.exe` and follow the installation wizard.
4. Launch **MXB Local** and choose **MX Bikes** on the setup screen.
5. Let the app detect your folders automatically. If prompted, select the requested folders, then start browsing mods.

Use the **.exe installer** to install the app. GitHub's **Source code** archives are for working with the source.

### Updating an existing installation

1. Close MXB Local.
2. Download the installer from the [latest release](https://github.com/Crab-Krab/mxb-local/releases/latest).
3. Run it to replace your current installation.

You do not need to uninstall MXB Local first. Updates reuse your settings and existing mod folders. Updates are installed manually.

## Catalog sorting

For **mxb-mods**, **Most popular** uses the site's top 100 overall, filtered to your selected category. A category can have no entries in that list.

Choose **Newest** or **Oldest** to browse the full category.

## Documentation

- [Fork details and build instructions](LOCAL-FORK.md)
- [Security review](SECURITY-REVIEW-local.4.md)
- [What the app contacts](docs/privacy-local.md)
- [Build and test results](https://github.com/Crab-Krab/mxb-local/actions/workflows/build-local-windows.yml)
- [Original project documentation](README.upstream.md)

## Credits and license

An independent fork of [Frostn1/mxb-app](https://github.com/Frostn1/mxb-app), based on v0.20.6.

Distributed under the [MIT License](LICENSE), with upstream copyright and attribution retained.
