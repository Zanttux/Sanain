; Per-user installer for Sanain (no administrator rights).
; Built by scripts/package.py: iscc /DAppVersion=.. /DSourceDir=.. /DOutputDir=.. /DOutputName=.. /DRepoRoot=..

#define AppName "Sanain"

[Setup]
AppId={{6E7B1D2C-58A4-4B5E-9C1F-5A1A9E0D3B21}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=Sanain project
AppPublisherURL=https://github.com/Zanttux/Sanain
AppSupportURL=https://github.com/Zanttux/Sanain/issues
DefaultDirName={localappdata}\Programs\Sanain
DefaultGroupName=Sanain
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
OutputDir={#OutputDir}
OutputBaseFilename={#OutputName}
SetupIconFile={#RepoRoot}\packaging\icons\sanain.ico
UninstallDisplayIcon={app}\sanain.exe
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
Name: "openwith"; Description: "Add Sanain to ""Open with"" and the Explorer context menu"; GroupDescription: "Integration:"

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{autoprograms}\Sanain"; Filename: "{app}\sanain.exe"
Name: "{autodesktop}\Sanain"; Filename: "{app}\sanain.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\sanain.exe"; Parameters: "--register"; Tasks: openwith; Flags: runhidden
Filename: "{app}\sanain.exe"; Description: "{cm:LaunchProgram,Sanain}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "{app}\sanain.exe"; Parameters: "--unregister"; Flags: runhidden; RunOnceId: "Unregister"
