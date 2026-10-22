# One-time migration of the installed Windows SDK using its signed installer.
# Payloads must first be prepared by prepare-sdk-migration.ps1.
[CmdletBinding()]
param([switch] $ResumeUninstalled, [switch] $AllowDependentPackageReinstall)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'sdk-installer-state.ps1')
$root = Split-Path -Parent $PSScriptRoot
$reportBase = Join-Path $root '.local-toolchain\sdk-migration'
$previousRun = Get-ChildItem -LiteralPath $reportBase -Directory -Filter 'attempt-*' | Sort-Object Name -Descending | Select-Object -First 1
$layout = Join-Path $reportBase 'layout'
$report = Join-Path $reportBase ('attempt-' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
$installer = Join-Path $layout 'winsdksetup.exe'
$oldRoot = 'C:\Program Files (x86)\Windows Kits\10'
$newRoot = 'E:\DevTools\WindowsKits\10'
$kitRegistry = 'HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots'
$bundleKey = 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\{c0b93b80-d0a4-47fc-a4d1-21a4008a63ca}'
$addonCode = '{13D2A7A2-1B42-4C03-8F0F-697991259AEE}'
$crtCode = '{0460C87B-7F4C-3170-FAC9-B7A6AE5CE4E9}'
$crtCache = 'E:\DevCache\VisualStudio\Packages\Microsoft.Windows.UniversalCRT.Redistributable.Msi,version=10.0.26624'
$crtMsi = Join-Path $crtCache 'Universal CRT Redistributable-x86_en-us.msi'
$crtDependency = 'HKLM:\SOFTWARE\Classes\Installer\Dependencies\Microsoft.Windows.UniversalCRTRedistributable.x86.10'
$features = 'OptionId.AvrfExternal OptionId.UWPManaged OptionId.WindowsSoftwareLogoToolkit OptionId.SigningTools OptionId.UWPLocalized OptionId.UWPCPP OptionId.DesktopCPPx64 OptionId.DesktopCPPx86 OptionId.DesktopCPPARM64 OptionId.MSIInstallTools'
$requiredFiles = @('Include\10.0.26100.0\um\Windows.h', 'Lib\10.0.26100.0\um\x64\kernel32.lib', 'Lib\10.0.26100.0\ucrt\x64\ucrt.lib', 'bin\10.0.26100.0\x64\rc.exe')

function Run-Sdk([string] $arguments, [string] $logName) {
    $log = Join-Path $report ($logName + '.log')
    Write-Output ('SDK stage: ' + $logName)
    $p = Start-Process -FilePath $installer -ArgumentList ($arguments + ' /quiet /norestart /ceip off /log "' + $log + '"') -WindowStyle Hidden -Wait -PassThru
    $code = $p.ExitCode
    [pscustomobject]@{Stage=$logName;ExitCode=$code;Time=(Get-Date -Format o)} | ConvertTo-Json -Compress | Add-Content -LiteralPath (Join-Path $report 'stages.jsonl')
    if ($code -notin @(0,3010)) { throw "SDK $logName failed with exit code $code. See $log" }
    if ($code -eq 3010) { $script:restartRequired = $true }
}

function Assert-SdkRoot([string] $expected) {
    $actual = (Get-ItemPropertyValue -LiteralPath $kitRegistry -Name KitsRoot10).TrimEnd('\')
    if ($actual -ne $expected) { throw "SDK root is $actual, expected $expected" }
    foreach ($relative in $requiredFiles) {
        if (-not (Test-Path -LiteralPath (Join-Path $expected $relative))) { throw "SDK file missing: $relative" }
    }
}

function Run-Msi([string] $arguments, [string] $stage) {
    Write-Output ('MSI stage: ' + $stage)
    $log = Join-Path $report ($stage + '.log')
    $p = Start-Process -FilePath msiexec.exe -ArgumentList ($arguments + ' /qn /norestart /L*v "' + $log + '"') -WindowStyle Hidden -Wait -PassThru
    [pscustomobject]@{Stage=$stage;ExitCode=$p.ExitCode;Time=(Get-Date -Format o)} | ConvertTo-Json -Compress | Add-Content -LiteralPath (Join-Path $report 'stages.jsonl')
    if ($p.ExitCode -notin @(0,3010)) { throw "MSI stage $stage failed: $($p.ExitCode)" }
    if ($p.ExitCode -eq 3010) { $script:restartRequired=$true }
}

function Remove-PathOwners {
    if (Test-Path -LiteralPath ('HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\' + $addonCode)) {
        $dependents = @(Get-ChildItem 'HKLM:\SOFTWARE\Classes\Installer\Dependencies\Microsoft.UAPSDKAddOn.SDK,10\Dependents' -ErrorAction SilentlyContinue)
        if ($dependents.Count -gt 0) { throw 'SDK AddOn still has dependents; refusing to remove it.' }
        Run-Msi ('/x ' + $addonCode) 'remove-sdk-addon'
    }
    if (Test-Path -LiteralPath ('HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\' + $crtCode)) {
        $vsDependent = 'VS.{AEF703B8-D2CC-4343-915C-F54A30B90937}'
        $dependents = @(Get-ChildItem -LiteralPath ($crtDependency + '\Dependents') -ErrorAction SilentlyContinue)
        if ($dependents | Where-Object { $_.PSChildName -ne $vsDependent }) { throw 'CRT has unexpected dependents.' }
        if ($dependents.Count -gt 0 -and -not $AllowDependentPackageReinstall) { throw 'Explicit approval for temporary dependent-package replacement is required.' }
        # Temporarily replace this exact VS-owned package and restore it below.
        # WiX otherwise silently cancels removal while returning exit code 0.
        Run-Msi ('/x ' + $crtCode + ' IGNOREDEPENDENCIES="' + $vsDependent + '"') 'remove-ucrt-development-redist'
        if (Test-Path -LiteralPath ('HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\' + $crtCode)) { throw 'CRT was not actually removed.' }
    }
}

function Restore-Crt([string] $targetRoot) {
    Run-Msi ('/i "' + $crtMsi + '" KITSROOT="' + $targetRoot + '"') 'restore-ucrt-development-redist'
    # Restore only the saved provider relationship for the same reinstalled MSI.
    # This export contains no SDK path and does not change KitsRoot10.
    & reg.exe import (Join-Path $report 'crt-dependency.reg') | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Could not restore CRT provider relationship.' }
}

function Clear-UnownedSdkRoot {
    # All SDK MSIs share this registry-key component (verified from MSI tables).
    # Only remove stale path values after Windows Installer reports no clients.
    $wi = New-Object -ComObject WindowsInstaller.Installer
    $clients = [TokenDailyMsiState]::Clients('{65DCECBE-68E8-5091-9544-DEC285CB1D59}')
    $count = $clients.Length
    $owners = @()
    for ($i=0; $i -lt $count; $i++) {
        $code = $clients[$i]
        $owners += [pscustomobject]@{ProductCode=$code;Name=$wi.ProductInfo($code,'ProductName')}
    }
    ConvertTo-Json -InputObject $owners | Set-Content -LiteralPath (Join-Path $report 'root-component-owners.json') -Encoding UTF8
    if ($count -ne 0) { throw 'SDK root still has MSI component clients; refusing to change its registration.' }
    if (Test-Path -LiteralPath $bundleKey) { throw 'SDK bundle is still registered.' }
    foreach ($key in @($kitRegistry, 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows Kits\Installed Roots')) {
        $value = Get-ItemProperty -LiteralPath $key -Name KitsRoot10 -ErrorAction SilentlyContinue
        if ($value) {
            if ($value.KitsRoot10.TrimEnd('\') -notin @($oldRoot, $newRoot)) { throw 'Unexpected SDK root registration.' }
            Remove-ItemProperty -LiteralPath $key -Name KitsRoot10
        }
    }
    Write-Output 'Cleared unowned SDK path registration; installer will register the new path.'
}

$principal = [Security.Principal.WindowsPrincipal]::new([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { throw 'Administrator privileges required.' }
if (Get-Process -Name winsdksetup,WinSdkInstaller,setup,vs_installer,vs_installershell,cargo,rustc -ErrorAction SilentlyContinue) { throw 'An installer or compiler is running.' }
# msiexec.exe can remain as an idle Windows service after the last operation.
# Check the active transaction marker instead of treating that service as busy.
if (Test-Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\InProgress') { throw 'Windows Installer has an active transaction.' }
if (-not $AllowDependentPackageReinstall -and (Get-ChildItem -LiteralPath ($crtDependency + '\Dependents') -ErrorAction SilentlyContinue)) {
    throw 'Migration is blocked before uninstall: VS depends on the CRT development package. A separate explicit decision is required.'
}
if ($ResumeUninstalled) {
    $lastResult = Get-Content -LiteralPath (Join-Path $reportBase 'result.json') -Raw | ConvertFrom-Json
    if ($lastResult.Status -ne 'RecoveryRequired' -or (Test-Path -LiteralPath $bundleKey) -or -not $previousRun) { throw 'Unexpected resume state.' }
    $resumeSnapshot = Get-Content -LiteralPath (Join-Path $previousRun.FullName 'before.json') -Raw | ConvertFrom-Json
    if ($resumeSnapshot.OldRoot -ne $oldRoot -or $resumeSnapshot.Version -ne '10.0.26100.7705') { throw 'Unexpected saved SDK state.' }
} else {
    Assert-SdkRoot $oldRoot
    if ((Get-ItemProperty -LiteralPath $bundleKey).DisplayVersion -ne '10.1.26100.7705') { throw 'Unexpected installed SDK version.' }
}
if (Test-Path -LiteralPath $newRoot) { throw 'Destination already exists; refusing to overwrite it.' }
foreach ($path in @('E:\', 'E:\DevTools', 'E:\DevTools\WindowsKits')) {
    if ((Test-Path -LiteralPath $path) -and ((Get-Item -LiteralPath $path -Force).Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Destination ancestor is a reparse point.' }
}
if ([IO.DriveInfo]::new('E:\').AvailableFreeSpace -lt 4GB) { throw 'Insufficient destination space.' }
$package = Get-Content -LiteralPath (Join-Path $reportBase 'package.json') -Raw | ConvertFrom-Json
foreach ($payload in $package.payloads) {
    $file = [IO.Path]::GetFullPath((Join-Path $layout $payload.fileName))
    if (-not $file.StartsWith($layout + '\', [StringComparison]::OrdinalIgnoreCase)) { throw 'Unsafe payload path.' }
    if ((Get-Item -LiteralPath $file).Length -ne $payload.size -or (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash -ne $payload.sha256) { throw 'Offline SDK payload verification failed.' }
}
$sig = Get-AuthenticodeSignature -LiteralPath $installer
if ($sig.Status -ne 'Valid' -or $sig.SignerCertificate.Subject -notmatch 'O=Microsoft Corporation') { throw 'Invalid installer signature.' }
$crtPackage = Get-Content -LiteralPath (Join-Path $crtCache '_package.json') -Raw | ConvertFrom-Json
foreach ($payload in $crtPackage.payloads) {
    $file = Join-Path $crtCache $payload.fileName
    if ((Get-Item -LiteralPath $file).Length -ne $payload.size -or (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash -ne $payload.sha256) { throw 'CRT payload verification failed.' }
}
$sig = Get-AuthenticodeSignature -LiteralPath $crtMsi
if ($sig.Status -ne 'Valid' -or $sig.SignerCertificate.Subject -notmatch 'O=Microsoft Corporation') { throw 'Invalid CRT MSI signature.' }
New-Item -ItemType Directory -Path $report | Out-Null
& reg.exe export 'HKLM\SOFTWARE\Classes\Installer\Dependencies\Microsoft.Windows.UniversalCRTRedistributable.x86.10' (Join-Path $report 'crt-dependency.reg') /y | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'CRT dependency export failed.' }
$env:TEMP = Join-Path $root '.bundle-tmp'
$env:TMP = $env:TEMP
New-Item -ItemType Directory -Force -Path $env:TEMP | Out-Null
$beforeFree = [IO.DriveInfo]::new('C:\').AvailableFreeSpace
$oldBytes = if ($ResumeUninstalled) { $resumeSnapshot.OldSdkBytes } else { (Get-ChildItem -LiteralPath $oldRoot -Recurse -File -Force | Measure-Object Length -Sum).Sum }
$optionKeys = @('HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots\10.0.26100.0\Installed Options','HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows Kits\Installed Roots\10.0.26100.0\Installed Options')
$optionSnapshot = @{}
foreach ($key in $optionKeys) {
    $data = Get-ItemProperty -LiteralPath $key -ErrorAction SilentlyContinue
    foreach ($property in $data.PSObject.Properties) {
        if ($property.Name -like 'OptionId.*' -and $property.Value -eq 1) { $optionSnapshot[$property.Name] = 1 }
    }
}
if ($ResumeUninstalled) {
    foreach ($p in $resumeSnapshot.InstalledOptions.PSObject.Properties) { $optionSnapshot[$p.Name] = $p.Value }
}
@{ OldRoot=$oldRoot; NewRoot=$newRoot; Features=$features; InstalledOptions=$optionSnapshot; OldSdkBytes=$oldBytes; CFreeBytesBefore=$beforeFree; Version='10.0.26100.7705' } |
    ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $report 'before.json') -Encoding UTF8
& reg.exe export 'HKLM\SOFTWARE\Microsoft\Windows Kits' (Join-Path $report 'windows-kits-64.reg') /y | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'Registry export failed.' }
& reg.exe export 'HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows Kits' (Join-Path $report 'windows-kits-32.reg') /y | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'Registry export failed.' }
Start-Transcript -Path (Join-Path $report 'migration.log') -Append | Out-Null
$restartRequired = $false
$started = $false
try {
    $started = $true
    if (-not $ResumeUninstalled) { Run-Sdk '/uninstall' 'uninstall-old' }
    Remove-PathOwners
    Clear-UnownedSdkRoot
    Run-Sdk ('/installpath "' + $newRoot + '" /features ' + $features) 'install-new'
    Assert-SdkRoot $newRoot
    Restore-Crt $newRoot
    Assert-SdkRoot $newRoot
    $remaining = if(Test-Path -LiteralPath $oldRoot) { (Get-ChildItem -LiteralPath $oldRoot -Recurse -File -Force | Measure-Object Length -Sum).Sum } else { 0 }
    [pscustomobject]@{Status='Migrated';Version='10.0.26100.7705';SdkRoot=$newRoot;OldSdkBytes=$oldBytes;OldRootRemainingBytes=$remaining;CFreeBytesBefore=$beforeFree;CFreeBytesAfter=[IO.DriveInfo]::new('C:\').AvailableFreeSpace;RestartRequired=$restartRequired;CompletedAt=(Get-Date -Format o)} |
        ConvertTo-Json | Tee-Object -FilePath (Join-Path $report 'result.json')
} catch {
    $failure = $_ | Out-String
    $failure | Add-Content -LiteralPath (Join-Path $report 'error.log') -Encoding UTF8
    if ($started) {
        try {
            Run-Sdk '/uninstall' 'rollback-uninstall'
            # If the old path is still registered, restore there immediately;
            # do not let the migration-only ownership guard block recovery.
            $registeredRoot = (Get-ItemProperty -LiteralPath $kitRegistry -Name KitsRoot10 -ErrorAction SilentlyContinue).KitsRoot10
            if ($registeredRoot -and $registeredRoot.TrimEnd('\') -ne $oldRoot) {
                Remove-PathOwners
                Clear-UnownedSdkRoot
            }
            Run-Sdk ('/installpath "' + $oldRoot + '" /features ' + $features) 'rollback-install'
            Restore-Crt $oldRoot
            Assert-SdkRoot $oldRoot
            @{Status='RolledBack';Failure=$failure;SdkRoot=$oldRoot;RestartRequired=$restartRequired} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $report 'result.json') -Encoding UTF8
        } catch {
            @{Status='RecoveryRequired';Failure=$failure;RecoveryFailure=($_ | Out-String)} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $report 'result.json') -Encoding UTF8
        }
    }
    throw $failure
} finally {
    Stop-Transcript | Out-Null
    if (Test-Path -LiteralPath (Join-Path $report 'result.json')) {
        Copy-Item -LiteralPath (Join-Path $report 'result.json') -Destination (Join-Path $reportBase 'result.json') -Force
    }
}
