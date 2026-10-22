# One-time migration using Microsoft's documented CachePath setting.
# Run elevated. Never moves SDKs or Windows Installer's separate Package Cache.
[CmdletBinding()]
param([switch] $Resume)
$ErrorActionPreference = 'Stop'
$source = 'C:\ProgramData\Microsoft\VisualStudio\Packages'
$destination = 'E:\DevCache\VisualStudio\Packages'
$registryPath = 'HKLM:\SOFTWARE\Microsoft\VisualStudio\Setup'
$policyPath = 'HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\Setup'
$reportDir = Join-Path (Split-Path -Parent $PSScriptRoot) '.local-toolchain\cache-migration'

function Assert-NoInstaller {
    $busy = Get-Process -Name setup,vs_installer,vs_installershell,msiexec -ErrorAction SilentlyContinue
    if ($busy) { throw 'An installer is running. Close it before migrating the cache.' }
}

function Read-Manifest([string] $root) {
    $items = @(Get-ChildItem -LiteralPath $root -Recurse -Force)
    if ($items | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }) {
        throw "Unexpected reparse point under $root"
    }
    @($items | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
        [pscustomobject]@{
            RelativePath = $_.FullName.Substring($root.Length + 1)
            Length = $_.Length
            SHA256 = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash
        }
    } | Sort-Object RelativePath)
}

$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = [Security.Principal.WindowsPrincipal]::new($identity)
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'Administrator privileges are required. No migration was performed.'
}
Assert-NoInstaller
if (Get-ItemProperty -LiteralPath $policyPath -Name CachePath -ErrorAction SilentlyContinue) {
    throw 'An enterprise CachePath policy exists; refusing to override it.'
}
$previous = Get-ItemProperty -LiteralPath $registryPath
if ($Resume) {
    $saved = Get-Content -LiteralPath (Join-Path $reportDir 'before.json') -Raw | ConvertFrom-Json
    if ($previous.CachePath -ne $destination -or $saved.Source -ne $source -or $saved.Destination -ne $destination -or $saved.PreviousCachePathPresent) {
        throw 'Resume state does not match this migration.'
    }
} elseif ($previous.PSObject.Properties['CachePath']) {
    throw 'CachePath is already configured; inspect it before proceeding.'
}
if ((Resolve-Path -LiteralPath $source).Path -ne $source) { throw 'Unexpected source path.' }
if ((Get-Item -LiteralPath $source -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) {
    throw 'Source is a reparse point.'
}
if (-not $Resume -and (Test-Path -LiteralPath $destination)) { throw 'Destination already exists; refusing to overwrite it.' }
if ($Resume -and ((Get-Item -LiteralPath $destination -Force).Attributes -band [IO.FileAttributes]::ReparsePoint)) {
    throw 'Destination is a reparse point.'
}
foreach ($parent in @('E:\', 'E:\DevCache', 'E:\DevCache\VisualStudio')) {
    if ((Test-Path -LiteralPath $parent) -and ((Get-Item -LiteralPath $parent -Force).Attributes -band [IO.FileAttributes]::ReparsePoint)) {
        throw "Destination ancestor is a reparse point: $parent"
    }
}
New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
Start-Transcript -Path (Join-Path $reportDir 'migration.log') -Append | Out-Null
try {
    $beforeFree = [IO.DriveInfo]::new('C:\').AvailableFreeSpace
    Write-Output 'Hashing source cache...'
    $manifest = @(Read-Manifest $source)
    if ($manifest.Count -eq 0) { throw 'Cache is empty.' }
    $bytes = ($manifest | Measure-Object Length -Sum).Sum
    if ([IO.DriveInfo]::new('E:\').AvailableFreeSpace -lt ($bytes + 1GB)) { throw 'Insufficient destination space.' }
    if ($Resume) {
        # PS 5.1 emits the entire JSON array as one pipeline item.
        # Direct assignment preserves the array correctly in both PS 5.1 and 7.
        $savedManifest = Get-Content -LiteralPath (Join-Path $reportDir 'manifest.json') -Raw | ConvertFrom-Json
        if (Compare-Object $savedManifest $manifest -Property RelativePath,Length,SHA256) {
            throw 'Source changed since the interrupted migration; refusing cleanup.'
        }
    } else {
    $manifest | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $reportDir 'manifest.json') -Encoding UTF8
    @{ Source=$source; Destination=$destination; PreviousCachePathPresent=$false; Bytes=$bytes } |
        ConvertTo-Json | Set-Content -LiteralPath (Join-Path $reportDir 'before.json') -Encoding UTF8

    New-Item -ItemType Directory -Path $destination -Force | Out-Null
    $acl = [Security.AccessControl.DirectorySecurity]::new()
    $acl.SetAccessRuleProtection($true, $false)
    $acl.SetOwner([Security.Principal.SecurityIdentifier]::new('S-1-5-32-544'))
    foreach ($sid in @('S-1-5-18', 'S-1-5-32-544', 'S-1-1-0')) {
        $rights = if ($sid -eq 'S-1-1-0') { 'ReadAndExecute' } else { 'FullControl' }
        $rule = [Security.AccessControl.FileSystemAccessRule]::new(
            [Security.Principal.SecurityIdentifier]::new($sid), $rights,
            'ContainerInherit, ObjectInherit', 'None', 'Allow')
        $acl.AddAccessRule($rule)
    }
    Set-Acl -LiteralPath $destination -AclObject $acl
    Write-Output 'Copying cache to E:...'
    Get-ChildItem -LiteralPath $source -Force | Copy-Item -Destination $destination -Recurse -Force
    }
    Write-Output 'Verifying destination and checking source has not changed...'
    $copied = @(Read-Manifest $destination)
    $current = @(Read-Manifest $source)
    foreach ($comparison in @(@{ Actual=$copied; Label='destination' }, @{ Actual=$current; Label='source' })) {
        if (Compare-Object -ReferenceObject $manifest -DifferenceObject $comparison.Actual -Property RelativePath,Length,SHA256) {
            throw "Manifest mismatch: $($comparison.Label). Original cache retained."
        }
    }
    Assert-NoInstaller
    New-ItemProperty -LiteralPath $registryPath -Name CachePath -Value $destination -PropertyType String -Force | Out-Null
    if ((Get-ItemPropertyValue -LiteralPath $registryPath -Name CachePath) -ne $destination) {
        throw 'Registry verification failed. Original cache retained.'
    }
    $vswhere = 'C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe'
    try {
        # Windows PowerShell's default native decoding corrupts UTF-8 Chinese
        # strings. Decode vswhere's redirected stdout explicitly.
        $startInfo = [Diagnostics.ProcessStartInfo]::new()
        $startInfo.FileName = $vswhere
        $startInfo.Arguments = '-all -products * -format json -utf8'
        $startInfo.UseShellExecute = $false
        $startInfo.CreateNoWindow = $true
        $startInfo.RedirectStandardOutput = $true
        $startInfo.StandardOutputEncoding = [Text.Encoding]::UTF8
        $discovery = [Diagnostics.Process]::Start($startInfo)
        $json = $discovery.StandardOutput.ReadToEnd()
        $discovery.WaitForExit()
        $instances = $json | ConvertFrom-Json
        if ($discovery.ExitCode -ne 0 -or -not ($instances | Where-Object { $_.isComplete -and $_.isLaunchable })) {
            throw 'VS discovery did not return a complete and launchable instance.'
        }
    } catch {
        Remove-ItemProperty -LiteralPath $registryPath -Name CachePath
        throw "VS discovery failed; CachePath rolled back, original retained. $_"
    }
    # Delete only the exact original cache after both copies matched SHA256.
    Assert-NoInstaller
    if ((Resolve-Path -LiteralPath $source).Path -ne 'C:\ProgramData\Microsoft\VisualStudio\Packages') {
        throw 'Refusing cleanup: source path changed.'
    }
    Remove-Item -LiteralPath $source -Recurse -Force
    [pscustomobject]@{
        Destination=$destination; Files=$manifest.Count; VerifiedBytes=$bytes
        SourceRemoved=(-not (Test-Path -LiteralPath $source))
        CFreeBytesBefore=$beforeFree; CFreeBytesAfter=[IO.DriveInfo]::new('C:\').AvailableFreeSpace
        CompletedAt=(Get-Date -Format o)
    } | ConvertTo-Json | Tee-Object -FilePath (Join-Path $reportDir 'result.json')
} catch {
    $_ | Out-String | Add-Content -LiteralPath (Join-Path $reportDir 'error.log') -Encoding UTF8
    throw
} finally {
    Stop-Transcript | Out-Null
}
