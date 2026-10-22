param([Parameter(Mandatory)][string]$SigningDirectory)
$ErrorActionPreference = 'Stop'
$signingRoot = [IO.Path]::GetFullPath($SigningDirectory)
if ($signingRoot.TrimEnd('\') -eq [IO.Path]::GetPathRoot($signingRoot).TrimEnd('\')) { throw 'A drive root cannot be used for signing keys.' }
if (Test-Path -LiteralPath $signingRoot) {
    if (((Get-Item -LiteralPath $signingRoot).Attributes -band [IO.FileAttributes]::ReparsePoint) -or (Get-ChildItem -LiteralPath $signingRoot -Force | Select-Object -First 1)) { throw 'Use a new, empty signing directory; existing files and permissions are preserved.' }
}
New-Item -ItemType Directory -Path $signingRoot -Force | Out-Null
$signingRoot = (Resolve-Path -LiteralPath $signingRoot).Path
$identity = [Security.Principal.WindowsIdentity]::GetCurrent().User
$system = [Security.Principal.SecurityIdentifier]::new('S-1-5-18')
$acl = [Security.AccessControl.DirectorySecurity]::new()
$acl.SetOwner($identity)
$acl.SetAccessRuleProtection($true, $false)
foreach ($sid in @($identity, $system)) {
    $acl.AddAccessRule([Security.AccessControl.FileSystemAccessRule]::new($sid,'FullControl','ContainerInherit,ObjectInherit','None','Allow'))
}
Set-Acl -LiteralPath $signingRoot -AclObject $acl
$keyPath = Join-Path $signingRoot 'updater.key'
$protectedPath = Join-Path $signingRoot 'updater.key.protected'
if ((Test-Path -LiteralPath $keyPath) -or (Test-Path -LiteralPath $protectedPath)) { throw 'Signing material already exists; it must not be overwritten.' }
Push-Location (Split-Path -Parent $PSScriptRoot)
try {
    # Capture CLI output; only the public key is returned to the caller.
    $signerOutput = & npm.cmd run tauri -- signer generate --ci --write-keys $keyPath 2>&1
    if ($LASTEXITCODE -ne 0) { throw 'Updater key generation failed.' }
    Add-Type -AssemblyName System.Security.Cryptography.ProtectedData
    $privateBytes = [IO.File]::ReadAllBytes($keyPath)
    try {
        $protected = [Security.Cryptography.ProtectedData]::Protect($privateBytes,$null,[Security.Cryptography.DataProtectionScope]::CurrentUser)
        [IO.File]::WriteAllBytes($protectedPath,$protected)
        Remove-Item -LiteralPath $keyPath
    } finally { [Array]::Clear($privateBytes,0,$privateBytes.Length) }
    Get-Content -LiteralPath ($keyPath + '.pub')
} finally { $signerOutput=$null; Pop-Location }
