param(
	[string]$Configuration = "release",
	[string]$OutputDirectory = (Join-Path $PSScriptRoot "..\..\target\windows-bundle"),
	[switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$repositoryRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\.."))
$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
$runtimeDirectory = Join-Path $repositoryRoot "target\windows-runtime"
$executable = Join-Path $repositoryRoot "target\$Configuration\votg.exe"
$installerCompiler = Join-Path ${env:ProgramFiles(x86)} "Inno Setup 6\ISCC.exe"

& (Join-Path $PSScriptRoot "download_runtime.ps1") -OutputDirectory $runtimeDirectory
$gstreamerDevDirectory = Join-Path $runtimeDirectory "gstreamer-devel"
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = $gstreamerDevDirectory
$env:PKG_CONFIG_PATH = Join-Path $gstreamerDevDirectory "lib\pkgconfig"

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
Copy-Item (Join-Path $repositoryRoot "assets\models") (Join-Path $OutputDirectory "models") -Recurse
if (Test-Path (Join-Path $repositoryRoot "assets\config")) {
	Copy-Item (Join-Path $repositoryRoot "assets\config") (Join-Path $OutputDirectory "config") -Recurse
}

if (-not (Test-Path $installerCompiler)) {
	throw "Inno Setup 6 was not found at $installerCompiler. Install it from https://jrsoftware.org/isdl.php."
}

& $installerCompiler "/DMyAppVersion=0.1.0" "/DSourceDir=$OutputDirectory" (Join-Path $PSScriptRoot "installer.iss")
if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed with exit code $LASTEXITCODE." }
