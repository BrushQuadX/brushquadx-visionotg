param(
	[string]$OutputDirectory = (Join-Path $PSScriptRoot "..\..\target\windows-runtime"),
	[string]$GStreamerVersion = "1.26.5",
	[string]$OnnxRuntimeVersion = "1.20.1"
)

$ErrorActionPreference = "Stop"

$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
$downloadDirectory = Join-Path $OutputDirectory "downloads"
$gstreamerArchive = Join-Path $downloadDirectory "gstreamer.zip"
$gstreamerDevArchive = Join-Path $downloadDirectory "gstreamer-devel.zip"
$onnxArchive = Join-Path $downloadDirectory "onnxruntime.zip"
$gstreamerDirectory = Join-Path $OutputDirectory "gstreamer"
$gstreamerDevDirectory = Join-Path $OutputDirectory "gstreamer-devel"

New-Item -ItemType Directory -Force -Path $downloadDirectory | Out-Null
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerDirectory
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerDevDirectory
New-Item -ItemType Directory -Force -Path $gstreamerDirectory | Out-Null
New-Item -ItemType Directory -Force -Path $gstreamerDevDirectory | Out-Null

$gstreamerUrl = "https://gstreamer.freedesktop.org/data/pkg/windows/$GStreamerVersion/msvc/gstreamer-1.0-msvc-x86_64-$GStreamerVersion.zip"
$gstreamerDevUrl = "https://gstreamer.freedesktop.org/data/pkg/windows/$GStreamerVersion/msvc/gstreamer-1.0-devel-msvc-x86_64-$GStreamerVersion.zip"
$onnxUrl = "https://github.com/microsoft/onnxruntime/releases/download/v$OnnxRuntimeVersion/onnxruntime-win-x64-$OnnxRuntimeVersion.zip"

Write-Host "Downloading GStreamer $GStreamerVersion"
Invoke-WebRequest -Uri $gstreamerUrl -OutFile $gstreamerArchive -UseBasicParsing
Write-Host "Downloading GStreamer development files $GStreamerVersion"
Invoke-WebRequest -Uri $gstreamerDevUrl -OutFile $gstreamerDevArchive -UseBasicParsing
Write-Host "Downloading ONNX Runtime $OnnxRuntimeVersion"
Invoke-WebRequest -Uri $onnxUrl -OutFile $onnxArchive -UseBasicParsing

$gstreamerExtract = Join-Path $downloadDirectory "gstreamer-extracted"
$gstreamerDevExtract = Join-Path $downloadDirectory "gstreamer-devel-extracted"
$onnxExtract = Join-Path $downloadDirectory "onnx-extracted"
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $gstreamerExtract, $gstreamerDevExtract, $onnxExtract
Expand-Archive -Path $gstreamerArchive -DestinationPath $gstreamerExtract
Expand-Archive -Path $gstreamerDevArchive -DestinationPath $gstreamerDevExtract
Expand-Archive -Path $onnxArchive -DestinationPath $onnxExtract

$gstreamerRoot = Get-ChildItem -Path $gstreamerExtract -Directory | Select-Object -First 1
if ($null -eq $gstreamerRoot) { throw "The GStreamer archive did not contain a root directory." }
Copy-Item -Path (Join-Path $gstreamerRoot.FullName "*") -Destination $gstreamerDirectory -Recurse -Force
$gstreamerDevRoot = Get-ChildItem -Path $gstreamerDevExtract -Directory | Select-Object -First 1
if ($null -eq $gstreamerDevRoot) { throw "The GStreamer development archive did not contain a root directory." }
Copy-Item -Path (Join-Path $gstreamerDevRoot.FullName "*") -Destination $gstreamerDevDirectory -Recurse -Force

$onnxDll = Get-ChildItem -Path $onnxExtract -Filter "onnxruntime.dll" -Recurse | Select-Object -First 1
if ($null -ne $onnxDll) {
	Copy-Item -Path $onnxDll.FullName -Destination $OutputDirectory -Force
}

Write-Host "Runtime files prepared in $OutputDirectory"
