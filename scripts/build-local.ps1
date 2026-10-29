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
bun test apps/manager/test/finishSetup.test.mjs
if ($LASTEXITCODE -ne 0) { throw 'First-run setup regression tests failed.' }
bun run --cwd apps/manager build
if ($LASTEXITCODE -ne 0) { throw 'Frontend build failed.' }
cargo test --locked -p mxb-core secret_store::tests --lib
if ($LASTEXITCODE -ne 0) { throw 'Windows credential-storage tests failed.' }
cargo test --locked -p mxb-app local_security_tests --bin mxb-app
if ($LASTEXITCODE -ne 0) { throw 'Backend security regression tests failed.' }
$env:VITE_APP_NAME = 'MXB Local'
Push-Location apps/manager
try {
    bun run tauri build --bundles nsis
    if ($LASTEXITCODE -ne 0) { throw 'Windows packaging failed.' }
} finally { Pop-Location }
Write-Host 'Installer created under target/release/bundle/nsis.'
