# Dot-source this file before running development/build commands:
# . .\scripts\use-local-dev-env.ps1
# Only changes the current PowerShell process and its future child processes.
$tokenDailyRoot = Split-Path -Parent $PSScriptRoot
$env:CARGO_HOME = Join-Path $tokenDailyRoot '.local-toolchain\cargo'
$env:RUSTUP_HOME = Join-Path $tokenDailyRoot '.local-toolchain\rustup'
$env:CARGO_TARGET_DIR = Join-Path $tokenDailyRoot 'src-tauri\target'
$env:NPM_CONFIG_CACHE = Join-Path $tokenDailyRoot '.npm-cache'
$env:TEMP = Join-Path $tokenDailyRoot '.bundle-tmp'
$env:TMP = $env:TEMP
$env:TMPDIR = $env:TEMP
New-Item -ItemType Directory -Force -Path $env:TEMP, $env:NPM_CONFIG_CACHE | Out-Null
$tokenDailyCargoBin = Join-Path $env:CARGO_HOME 'bin'
if (($env:Path -split ';') -notcontains $tokenDailyCargoBin) {
    $env:Path = $tokenDailyCargoBin + ';' + $env:Path
}
