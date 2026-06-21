# STUDIO-WINDOWS-DIAGNOSTIC-DEBUG-EXE-0 — minimal debug Studio build + checksum
# Package: simthing-mapeditor; binary: simthing-studio

$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..\..")

Write-Host "Building debug simthing-studio..."
cargo build -p simthing-mapeditor --bin simthing-studio
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$exe = "target\debug\simthing-studio.exe"
if (-not (Test-Path $exe)) {
    Write-Error "Expected executable not found: $exe"
    exit 1
}

Write-Host ""
Write-Host "SHA256:"
Get-FileHash $exe -Algorithm SHA256