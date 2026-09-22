param(
    [int]$Skip = 0,
    [int]$Limit = 10,
    [string]$ReportPath
)

$ErrorActionPreference = 'Stop'
$repo = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$cohort = Get-Content -LiteralPath (Join-Path $repo 'registry/cohorts/product-a-cohort.json') -Raw | ConvertFrom-Json
$health = Get-Content -LiteralPath (Join-Path $repo 'registry/catalog/infinity-mod-forge-health.json') -Raw | ConvertFrom-Json
$inspector = Join-Path $repo 'target/debug/iepm.exe'
if (-not (Test-Path -LiteralPath $inspector -PathType Leaf)) {
    throw "Build the IEPM CLI first: cargo build -p iepm"
}

$destination = Join-Path $repo 'target/pa4-artifacts/cohort'
New-Item -ItemType Directory -Force -Path $destination | Out-Null
$healthById = @{}
foreach ($entry in $health.entries) { $healthById[[int]$entry.source_id] = $entry }
$candidates = @(
    $cohort.packages | Where-Object {
        -not $_.curated_package -and
        $healthById.ContainsKey([int]$_.source_id) -and
        $healthById[[int]$_.source_id].acquisition.kind -eq 'github-tag-source-archive' -and
        $healthById[[int]$_.source_id].artifact_probe.status -eq 'archive-signature-detected'
    } | Select-Object -Skip $Skip -First $Limit
)
$results = @()
foreach ($package in $candidates) {
    $id = [int]$package.source_id
    $record = $healthById[$id]
    $archive = Join-Path $destination ("{0}.zip" -f $id)
    $partial = Join-Path $destination ("{0}.partial" -f $id)
    Write-Host ("[{0}] {1}" -f $id, $package.name)
    try {
        if (-not (Test-Path -LiteralPath $archive -PathType Leaf)) {
            Invoke-WebRequest -Uri $record.acquisition.candidate_url -Headers @{ 'User-Agent' = 'IEPM-PA4-audit' } -OutFile $partial
            Move-Item -LiteralPath $partial -Destination $archive
        }
        $sha = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
        $inspectionText = & $inspector inspect-package --path $archive 2>&1
        if ($LASTEXITCODE -ne 0) { throw ($inspectionText | Out-String) }
        $inspection = ($inspectionText | Out-String) | ConvertFrom-Json
        $results += [pscustomobject]@{
            source_id = $id
            package = $package.candidate_package
            name = $package.name
            candidate_url = $record.acquisition.candidate_url
            sha256 = $sha
            bytes = (Get-Item -LiteralPath $archive).Length
            weidu_package = [bool]$inspection.weidu_package
            tp2_files = @($inspection.tp2_files | ForEach-Object { $_.path })
            components_observed = ($inspection.tp2_files | ForEach-Object { @($_.components).Count } | Measure-Object -Sum).Sum
            tp2_observations = @($inspection.tp2_files)
            error = $null
        }
    } catch {
        $results += [pscustomobject]@{
            source_id = $id
            package = $package.candidate_package
            name = $package.name
            candidate_url = $record.acquisition.candidate_url
            sha256 = $null
            bytes = $null
            weidu_package = $null
            tp2_files = @()
            components_observed = $null
            tp2_observations = @()
            error = $_.Exception.Message
        }
    }
}
$report = [pscustomobject]@{
    schema = 1
    catalog_source_revision = $cohort.catalog_source_revision
    skip = $Skip
    limit = $Limit
    results = $results
}
if (-not $ReportPath) {
    $ReportPath = Join-Path $destination ("acquisition-{0}-{1}.json" -f $Skip, $Limit)
}
$reportPath = $ReportPath
$report | ConvertTo-Json -Depth 12 | Set-Content -LiteralPath $reportPath -Encoding utf8
Write-Host ("Report: {0}" -f $reportPath)
$results | Select-Object source_id,package,weidu_package,components_observed,error | Format-Table -AutoSize
