; Per-user installer for Text³ (no administrator rights).
; Built by scripts/package.py: iscc /DAppVersion=.. /DSourceDir=.. /DOutputDir=.. /DOutputName=.. /DRepoRoot=..

#define AppName "Text³"

[Setup]
; AppId is unchanged from Sanain: installing Text³ upgrades an existing
; Sanain installation in place instead of adding a second program.
AppId={{6E7B1D2C-58A4-4B5E-9C1F-5A1A9E0D3B21}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=Text³ project
AppPublisherURL=https://github.com/Zanttux/text3
AppSupportURL=https://github.com/Zanttux/text3/issues
DefaultDirName={localappdata}\Programs\Text3
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
OutputDir={#OutputDir}
OutputBaseFilename={#OutputName}
SetupIconFile={#RepoRoot}\packaging\icons\text3.ico
UninstallDisplayIcon={app}\text3.exe
LicenseFile={#SourceDir}\LICENSE
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "finnish"; MessagesFile: "compiler:Languages\Finnish.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "openwith"; Description: "Add {#AppName} to ""Open with"" and the Explorer context menu"; GroupDescription: "Integration:"

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\text3.exe"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\text3.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\text3.exe"; Parameters: "--register"; Tasks: openwith; Flags: runhidden
Filename: "{app}\text3.exe"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "{app}\text3.exe"; Parameters: "--unregister"; Flags: runhidden; RunOnceId: "Unregister"
