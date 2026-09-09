#define MyAppName "VisionOTG"
#define MyAppPublisher "BrushQuadX"
#define MyAppExeName "votg.exe"
#ifndef MyAppVersion
	#define MyAppVersion "0.1.0"
#endif
#ifndef SourceDir
	#define SourceDir AddBackslash(SourcePath) + "..\..\target\windows-bundle"
#endif

[Setup]
AppId={{B7BF3A6F-8A4D-4A4C-9A31-5BD819E4A57D}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
OutputDir={#SourcePath}\..\..\target\windows-installer
OutputBaseFilename=VisionOTGSetup
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=admin
WizardStyle=modern
UninstallDisplayIcon={app}\{#MyAppExeName}

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent
