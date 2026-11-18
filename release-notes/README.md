# Release notes

One file per release, named after its tag (for example `v1.0.0.md`). The publish
workflow (`.github/workflows/publish-local-release.yml`) refuses to run without it and appends
the build link, source commit and verified installer SHA-256 itself, so leave those out.

Start the "Install or update" section with the wording below, so a new install isn't told to
close an app it doesn't have yet:

```markdown
### Install or update

Download **MXB.Local_<version>_x64-setup.exe** below.

- **New install:** run the installer and follow the steps.
- **Updating:** close MXB Local first, then run the installer. Your settings are kept.

GitHub's automatic Source code downloads contain the source, not the Windows installer.
```

A release's notes can still be edited on GitHub after it is published. If you do, make the same
edit here too, or the next release copies the old wording.
