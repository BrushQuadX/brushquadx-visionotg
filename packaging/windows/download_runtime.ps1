param(
	[string]$OutputDirectory = (Join-Path $PSScriptRoot "..\..\target\windows-runtime"),
	[string]$GStreamerVersion = "1.26.5",
	[string]$OnnxRuntimeVersion = "1.20.1"
)

$ErrorActionPreference = "Stop"

$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
$downloadDirectory = Join-Path $OutputDirectory "downloads"
$gstreamerArchive = Join-Path $downloadDirectory "gstreamer-runtime.msi"
$gstreamerDevArchive = Join-Path $downloadDirectory "gstreamer-devel.msi"
$onnxArchive = Join-Path $downloadDirectory "onnxruntime.zip"
$gstreamerDirectory = Join-Path $OutputDirectory "gstreamer"
$gstreamerDevDirectory = Join-Path $OutputDirectory "gstreamer-devel"
$gstreamerRuntimeInstallRoot = Join-Path $OutputDirectory "gstreamer-runtime-install"
$gstreamerDevInstallRoot = Join-Path $OutputDirectory "gstreamer-devel-install"

New-Item -ItemType Directory -Force -Path $downloadDirectory | Out-Null
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerDirectory
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerDevDirectory
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerRuntimeInstallRoot
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerDevInstallRoot

$gstreamerUrl = "https://gstreamer.freedesktop.org/data/pkg/windows/$GStreamerVersion/msvc/gstreamer-1.0-msvc-x86_64-$GStreamerVersion.msi"
$gstreamerDevUrl = "https://gstreamer.freedesktop.org/data/pkg/windows/$GStreamerVersion/msvc/gstreamer-1.0-devel-msvc-x86_64-$GStreamerVersion.msi"
$onnxUrl = "https://github.com/microsoft/onnxruntime/releases/download/v$OnnxRuntimeVersion/onnxruntime-win-x64-$OnnxRuntimeVersion.zip"

Write-Host "Downloading GStreamer $GStreamerVersion"
Invoke-WebRequest -Uri $gstreamerUrl -OutFile $gstreamerArchive -UseBasicParsing
Write-Host "Downloading GStreamer development files $GStreamerVersion"
Invoke-WebRequest -Uri $gstreamerDevUrl -OutFile $gstreamerDevArchive -UseBasicParsing
Write-Host "Downloading ONNX Runtime $OnnxRuntimeVersion"
Invoke-WebRequest -Uri $onnxUrl -OutFile $onnxArchive -UseBasicParsing

function Resolve-GStreamerRoot($installRoot) {
    $candidates = @(
        (Join-Path $installRoot "gstreamer"),
        (Join-Path $installRoot "GStreamer"),
        (Join-Path $installRoot "PFiles64"),
        $installRoot
    )

    foreach ($candidate in $candidates) {
        if (-not (Test-Path $candidate)) { continue }

        $msvcRoot = Get-ChildItem -Path $candidate -Recurse -Directory -Filter "msvc_x86_64" -ErrorAction SilentlyContinue |
            Select-Object -First 1

        if ($msvcRoot) { return $msvcRoot.FullName }
    }

    throw "Could not find the GStreamer MSVC install root under $installRoot."
}

Write-Host "Installing GStreamer runtime into $gstreamerRuntimeInstallRoot"
& msiexec.exe /i $gstreamerArchive /qn INSTALLDIR=$gstreamerRuntimeInstallRoot | Out-Null
Write-Host "Installing GStreamer development files into $gstreamerDevInstallRoot"
& msiexec.exe /i $gstreamerDevArchive /qn INSTALLDIR=$gstreamerDevInstallRoot | Out-Null

$gstreamerRoot = Resolve-GStreamerRoot $gstreamerRuntimeInstallRoot
$gstreamerDevRoot = Resolve-GStreamerRoot $gstreamerDevInstallRoot

$gstreamerPkgConfigRoot = Get-ChildItem -Path $gstreamerDevRoot -Recurse -Directory -Filter "pkgconfig" -ErrorAction SilentlyContinue |
    Select-Object -First 1 -ExpandProperty FullName

if (-not $gstreamerPkgConfigRoot) {
    throw "GStreamer development files did not include a pkg-config directory under $gstreamerDevRoot."
}

New-Item -ItemType Directory -Force -Path $gstreamerDirectory | Out-Null
New-Item -ItemType Directory -Force -Path $gstreamerDevDirectory | Out-Null
Copy-Item -Path (Join-Path $gstreamerRoot "*") -Destination $gstreamerDirectory -Recurse -Force
Copy-Item -Path (Join-Path $gstreamerDevRoot "*") -Destination $gstreamerDevDirectory -Recurse -Force

$onnxExtract = Join-Path $downloadDirectory "onnx-extracted"
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $onnxExtract
New-Item -ItemType Directory -Force -Path $onnxExtract | Out-Null
Expand-Archive -Path $onnxArchive -DestinationPath $onnxExtract
$onnxDll = Get-ChildItem -Path $onnxExtract -Filter "onnxruntime.dll" -Recurse | Select-Object -First 1
if ($null -ne $onnxDll) {
	Copy-Item -Path $onnxDll.FullName -Destination $OutputDirectory -Force
}

Write-Host "Runtime files prepared in $OutputDirectory"
Write-Host "GStreamer pkg-config directory: $gstreamerPkgConfigRoot"
