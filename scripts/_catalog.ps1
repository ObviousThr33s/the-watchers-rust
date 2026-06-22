# Shared reader: turns the Rust integration tests into a readable catalog.
#
# Pure read. It never edits a test -- the tests stay the single source of truth;
# this only *reads* the sentence each test already carries (its fn name) and the
# `///` rationale above it. Dot-source this file, then call Get-TestCatalog.
#
# This is not a group entrypoint (hence the leading underscore). The files you
# actually run are tests-content.ps1, tests-contributors.ps1, tests-reviewers.ps1.

function ConvertTo-Sentence {
    # A snake_case test name is already a sentence; just make it read like one.
    param([string]$FnName)
    $words = $FnName -replace '_', ' '
    if ($words.Length -gt 0) {
        $words = $words.Substring(0, 1).ToUpper() + $words.Substring(1)
    }
    return $words + '.'
}

function Get-TestCatalog {
    # Reads each file and returns one object per file: its `//!` header and the
    # list of #[test] functions, each with its sentence and its `///` rationale.
    param([string[]]$Path)

    foreach ($file in $Path) {
        $lines = Get-Content -LiteralPath $file -Encoding UTF8

        # The module header is the leading `//!` block.
        $header = @()
        foreach ($line in $lines) {
            if ($line -match '^\s*//!\s?(.*)$') { $header += $matches[1] }
            elseif ($line.Trim() -ne '') { break }
        }

        # Walk the body. `///` lines accumulate; `#[test]` arms the next `fn`.
        $tests = @()
        $doc = @()
        $armed = $false
        foreach ($line in $lines) {
            if ($line -match '^\s*///\s?(.*)$') {
                $doc += $matches[1]
            }
            elseif ($line -match '^\s*#\[test\]') {
                $armed = $true
            }
            elseif ($line -match '^\s*fn\s+([a-z0-9_]+)\s*\(') {
                if ($armed) {
                    $tests += [pscustomobject]@{
                        Name      = $matches[1]
                        Sentence  = ConvertTo-Sentence $matches[1]
                        Rationale = ($doc -join ' ').Trim()
                    }
                    $armed = $false
                }
                $doc = @()
            }
            elseif ($line.Trim() -ne '') {
                # Any other code line: a `///` above it wasn't a test rationale.
                $doc = @()
            }
        }

        [pscustomobject]@{
            File   = [System.IO.Path]::GetFileName($file)
            Header = ($header -join "`n").Trim()
            Tests  = $tests
        }
    }
}

function Get-TestFiles {
    # The integration tests, absolute paths. Repo root is one level up.
    param([string[]]$Names)
    $repo = Split-Path -Parent $PSScriptRoot
    foreach ($name in $Names) { Join-Path $repo "tests\$name" }
}

function Save-Catalog {
    # Write the generated Markdown and say where it went. UTF-8 so the glyphs
    # in the tests (e.g. the multibyte-glyph case) survive.
    param([string]$RelPath, [string[]]$Lines)
    $repo = Split-Path -Parent $PSScriptRoot
    $out = Join-Path $repo $RelPath
    Set-Content -LiteralPath $out -Value $Lines -Encoding UTF8
    Write-Host "wrote $RelPath"
}
