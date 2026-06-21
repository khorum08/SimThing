# WORKSPACE-CLEANUP-INVENTORY-0 — safe local reclaim + size report (repo root)
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..\..")

function Get-PathSizeGB([string]$Path) {
    if (-not (Test-Path $Path)) { return 0.0 }
    $item = Get-Item -Force $Path
    if ($item.PSIsContainer) {
        $sum = (Get-ChildItem $Path -Recurse -Force -File -ErrorAction SilentlyContinue |
            Measure-Object Length -Sum).Sum
        if ($null -eq $sum) { return 0.0 }
        return [math]::Round($sum / 1GB, 3)
    }
    return [math]::Round($item.Length / 1GB, 3)
}

$beforeTarget = Get-PathSizeGB "target"
$beforeDiag = Get-PathSizeGB "diagnostics"

Write-Host "Before: target=$beforeTarget GB diagnostics=$beforeDiag GB"
cargo clean
Remove-Item -Recurse -Force diagnostics -ErrorAction SilentlyContinue
Remove-Item -Force screenshot_*.png -ErrorAction SilentlyContinue

$afterTarget = Get-PathSizeGB "target"
Write-Host "After: target=$afterTarget GB"
Write-Host "Reclaimed target+diagnostics GB:" ([math]::Round($beforeTarget + $beforeDiag - $afterTarget, 3))