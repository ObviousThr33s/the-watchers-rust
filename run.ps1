# run.ps1 — build (release) and launch the game from the project root.
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if ($args.Count -gt 0) {
    Start-Process -FilePath ".\target\release\obelisk.exe" -ArgumentList $args
} else {
    Start-Process -FilePath ".\target\release\obelisk.exe"
}
