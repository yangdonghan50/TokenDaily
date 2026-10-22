# Prepare the exact installed SDK payloads before any uninstall.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$report = Join-Path $root '.local-toolchain\sdk-migration'
$layout = Join-Path $report 'layout'
$cache = 'E:\DevCache\VisualStudio\Packages\Win11SDK_10.0.26100,version=10.0.26100.15,productarch=neutral'
$manifestPath = Join-Path $cache '_package.json'
$package = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
New-Item -ItemType Directory -Force -Path $layout | Out-Null
Copy-Item -LiteralPath $manifestPath -Destination (Join-Path $report 'package.json') -Force
$candidates = @{}
foreach ($file in (Get-ChildItem -LiteralPath 'C:\ProgramData\Package Cache', $cache -Recurse -File -Force)) {
    $key = $file.Name + '|' + $file.Length
    if (-not $candidates.ContainsKey($key)) { $candidates[$key] = @() }
    $candidates[$key] += $file.FullName
}
$copied = 0
$downloaded = 0
$verified = 0
foreach ($payload in $package.payloads) {
    $target = [IO.Path]::GetFullPath((Join-Path $layout $payload.fileName))
    if (-not $target.StartsWith($layout + '\', [StringComparison]::OrdinalIgnoreCase)) { throw 'Unsafe payload path.' }
    if ($payload.url -notlike 'https://download.visualstudio.microsoft.com/*') { throw 'Unexpected payload host.' }
    if ((Test-Path -LiteralPath $target) -and (Get-Item -LiteralPath $target).Length -eq $payload.size -and (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -eq $payload.sha256) {
        $verified++
        continue
    }
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target) | Out-Null
    $found = $false
    $key = [IO.Path]::GetFileName($payload.fileName) + '|' + $payload.size
    foreach ($candidate in $candidates[$key]) {
        if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -eq $payload.sha256) {
            Copy-Item -LiteralPath $candidate -Destination $target -Force
            $copied++
            $found = $true
            break
        }
    }
    if (-not $found) {
        Write-Output ('Downloading: ' + $payload.fileName)
        Invoke-WebRequest -Uri $payload.url -OutFile $target -TimeoutSec 120 -UseBasicParsing
        $downloaded++
    }
    if ((Get-Item -LiteralPath $target).Length -ne $payload.size -or (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $payload.sha256) {
        throw ('Payload verification failed: ' + $payload.fileName)
    }
    $verified++
}
$signature = Get-AuthenticodeSignature -LiteralPath (Join-Path $layout 'winsdksetup.exe')
if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch 'O=Microsoft Corporation') {
    throw 'SDK installer signature is not valid Microsoft signature.'
}
[pscustomobject]@{ Version='10.0.26100.7705'; Layout=$layout; PayloadCount=$verified; Copied=$copied; Downloaded=$downloaded; Bytes=($package.payloads | Measure-Object size -Sum).Sum; PreparedAt=(Get-Date -Format o) } |
    ConvertTo-Json | Tee-Object -FilePath (Join-Path $report 'prepared.json')
