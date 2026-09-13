param(
	[string]$Configuration = "release",
	[string]$OutputDirectory = (Join-Path $PSScriptRoot "..\..\target\windows-bundle"),
    [string]$Version = "",
	[switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$repositoryRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\.."))
$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
$runtimeDirectory = Join-Path $repositoryRoot "target\windows-runtime"
$executable = Join-Path $repositoryRoot "target\$Configuration\votg.exe"
$installerCompiler = $null

if (-not $Version) {
    if ($env:GITHUB_REF -like "refs/tags/*") {
        $Version = $env:GITHUB_REF_NAME
    }
}

if (-not $Version) {
    $Version = "1.0.0"
}

$Version = $Version -replace '^v', ''
if ($Version -notmatch '^\d+(\.\d+){1,3}$') {
    throw "Version must be numeric and contain between two and four components, for example 1.0.0 or v1.2.3."
}

$innoSetupRoots = @(
    ${env:ProgramFiles(x86)},
    ${env:ProgramFiles}
)

foreach ($innoSetupRoot in $innoSetupRoots) {
    if (-not $innoSetupRoot) { continue }

    $candidate = Get-ChildItem -Path $innoSetupRoot -Filter "ISCC.exe" -Recurse -ErrorAction SilentlyContinue |
        Sort-Object FullName -Descending |
        Select-Object -ExpandProperty FullName -First 1

    if ($candidate) {
        $installerCompiler = $candidate
        break
    }
}

if (-not $installerCompiler) {
    $installerCompiler = Get-Command "ISCC.exe" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
}

& (Join-Path $PSScriptRoot "download_runtime.ps1") -OutputDirectory $runtimeDirectory
$gstreamerDevDirectory = Join-Path $runtimeDirectory "gstreamer-devel"
$gstreamerPkgConfigRoot = Get-ChildItem -Path $gstreamerDevDirectory -Recurse -Directory -Filter "pkgconfig" -ErrorAction SilentlyContinue |
    Select-Object -First 1 -ExpandProperty FullName

if (-not $gstreamerPkgConfigRoot) {
    $gstreamerPackageRoot = Get-ChildItem -Path $runtimeDirectory -Recurse -Directory -Filter "msvc_x86_64" -ErrorAction SilentlyContinue |
        Select-Object -First 1 -ExpandProperty FullName

    if ($gstreamerPackageRoot) {
        $gstreamerPkgConfigRoot = Join-Path $gstreamerPackageRoot "lib\pkgconfig"
    }
}

if (-not $gstreamerPkgConfigRoot -or -not (Test-Path $gstreamerPkgConfigRoot)) {
    throw "Could not find the GStreamer pkg-config directory under $gstreamerDevDirectory."
}

$gstreamerRoot = Split-Path -Path $gstreamerPkgConfigRoot -Parent | Split-Path -Parent
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = $gstreamerRoot
$env:PKG_CONFIG_PATH = $gstreamerPkgConfigRoot

if (-not $SkipBuild) {
	Push-Location $repositoryRoot
	try { cargo build --locked --release } finally { Pop-Location }
}

if (-not (Test-Path $executable)) { throw "Release executable not found: $executable" }
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $OutputDirectory
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null

Copy-Item $executable (Join-Path $OutputDirectory "votg.exe")
Copy-Item (Join-Path $runtimeDirectory "onnxruntime.dll") $OutputDirectory
Copy-Item (Join-Path $runtimeDirectory "gstreamer") (Join-Path $OutputDirectory "gstreamer") -Recurse
$gstreamerBinDirectory = Join-Path $OutputDirectory "gstreamer\bin"
Copy-Item (Join-Path $gstreamerBinDirectory "*.dll") $OutputDirectory -Force
Copy-Item (Join-Path $repositoryRoot "assets\models") (Join-Path $OutputDirectory "models") -Recurse
Copy-Item (Join-Path $repositoryRoot "assets\images\visionotg.ico") $OutputDirectory

if (-not $installerCompiler -or -not (Test-Path $installerCompiler)) {
	throw "Inno Setup was not found. Install a recent version from https://jrsoftware.org/isdl.php."
}

& $installerCompiler "/DMyAppVersion=$Version" "/DSourceDir=$OutputDirectory" (Join-Path $PSScriptRoot "installer.iss")
if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed with exit code $LASTEXITCODE." }
