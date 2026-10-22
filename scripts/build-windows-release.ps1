param([Parameter(Mandatory)][string]$SigningDirectory, [string]$ProjectDirectory = (Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'use-local-dev-env.ps1')
$protectedPath = Join-Path ([IO.Path]::GetFullPath($SigningDirectory)) 'updater.key.protected'
# PowerShell 7 ships ProtectedData separately; Windows PowerShell 5.1 has it in System.Security.
try { Add-Type -AssemblyName System.Security.Cryptography.ProtectedData } catch { Add-Type -AssemblyName System.Security }
$privateBytes = [Security.Cryptography.ProtectedData]::Unprotect([IO.File]::ReadAllBytes($protectedPath),$null,[Security.Cryptography.DataProtectionScope]::CurrentUser)
$oldPrivateKey = $env:TAURI_SIGNING_PRIVATE_KEY
$oldPassword = $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD
Push-Location ([IO.Path]::GetFullPath($ProjectDirectory))
try {
    $env:TAURI_SIGNING_PRIVATE_KEY = [Text.Encoding]::UTF8.GetString($privateBytes)
    $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = ''
    & npm.cmd run tauri -- build --bundles nsis
    if ($LASTEXITCODE -ne 0) { throw 'Signed Windows release build failed.' }
} finally {
    $env:TAURI_SIGNING_PRIVATE_KEY = $oldPrivateKey
    $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = $oldPassword
    [Array]::Clear($privateBytes,0,$privateBytes.Length)
    Pop-Location
}
