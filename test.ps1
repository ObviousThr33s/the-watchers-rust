# test.ps1 — run the test suite from the project root.
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

cargo test @args
exit $LASTEXITCODE
