$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)

foreach ($tool in @('cargo', 'bun', 'python')) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        throw "Missing $tool. Use the GitHub Windows build instead, or install the documented build tools."
    }
}
python scripts/prepare-local-dependencies.py
if ($LASTEXITCODE -ne 0) { throw 'Dependency preparation failed.' }
bun install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { throw 'Frontend dependency installation failed.' }
# Every check runs on every build. Filtered runs once let two regressions through unnoticed.
bun test
if ($LASTEXITCODE -ne 0) { throw 'Frontend unit tests failed.' }
bun run typecheck
if ($LASTEXITCODE -ne 0) { throw 'Frontend type check failed.' }
bun run lint
if ($LASTEXITCODE -ne 0) { throw 'Frontend lint failed.' }
bun run --cwd apps/manager build
if ($LASTEXITCODE -ne 0) { throw 'Frontend build failed.' }
cargo test --locked --workspace
if ($LASTEXITCODE -ne 0) { throw 'Backend test suite failed.' }
$env:VITE_APP_NAME = 'MXB Local'
Push-Location apps/manager
try {
    bun run tauri build --bundles nsis
    if ($LASTEXITCODE -ne 0) { throw 'Windows packaging failed.' }
} finally { Pop-Location }
Write-Host 'Installer created under target/release/bundle/nsis.'
