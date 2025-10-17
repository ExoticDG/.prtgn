#define MyAppName "prtgn"
#define MyAppVersion "0.2.0"
#define MyAppPublisher "ExoticDarknessGaming (Dr. Nova Shadowtail)"
#define MyAppURL "https://github.com/ExoticDG/.prtgn"
#define MyAppExeName "prtgn.exe"
#define MyAppAssocName "Protogen File Format"
#define MyAppAssocExt ".prtgn"
#define MyAppAssocKey StringChange(MyAppAssocName, " ", "") + MyAppAssocExt

#define MyLicenseUrl "https://github.com/ExoticDG/.prtgn/blob/main/LICENSE"
#define MyLicenseFile "LICENSE"
#define MyIconUrl "https://github.com/ExoticDG/.prtgn/blob/main/prtgn_logo.ico"
#define MyIconFile "prtgn_logo.ico"

#define MyInfoUrl "https://github.com/ExoticDG/.prtgn/blob/main/README.md"
#define MyInfoFile "README.md"
#define MySourceUrl "https://github.com/ExoticDG/.prtgn/releases/download/v{#MyAppVersion}/prtgn.exe"
#define MySourceFile SourcePath + "\prtgn.exe"

#expr Exec('powershell -Command "Invoke-WebRequest -Uri ''" + MyLicenseUrl + "'' -OutFile ''" + MyLicenseFile + "''"')
#expr Exec('powershell -Command "Invoke-WebRequest -Uri ''" + MyIconUrl + "'' -OutFile ''" + MyIconFile + "''"')
#expr Exec('powershell -Command "Invoke-WebRequest -Uri ''" + MyInfoUrl + "'' -OutFile ''" + MyInfoFile + "''"')
; #expr Exec('powershell -Command "Invoke-WebRequest -Uri ''" + MySourceUrl + "'' -OutFile ''" + MySourceFile + "''"')
#expr Exec('powershell -Command "[System.Net.ServicePointManager]::SecurityProtocol = ''Tls12''; Invoke-WebRequest -Uri ''" + MySourceUrl + "'' -OutFile ''" + MySourceFile + "''"')

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{95A5ABC3-BFE7-4BE2-B3F1-82E15897A241}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={localappdata}\{#MyAppName}
UninstallDisplayIcon={app}\{#MyAppExeName}
; "ArchitecturesAllowed=x64compatible" specifies that Setup cannot run
; on anything but x64 and Windows 11 on Arm.
ArchitecturesAllowed=x64compatible
; "ArchitecturesInstallIn64BitMode=x64compatible" requests that the
; install be done in "64-bit mode" on x64 or Windows 11 on Arm,
; meaning it should use the native 64-bit Program Files directory and
; the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64compatible
ChangesAssociations=yes
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile={#MyLicenseFile}
InfoBeforeFile={#MyInfoFile}
; Remove the following line to run in administrative install mode (install for all users).
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputBaseFilename=mysetup
SetupIconFile={#MyIconFile}
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "{#MySourceFile}"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Registry]
Root: HKA; Subkey: "Software\Classes\{#MyAppAssocExt}\OpenWithProgids"; ValueType: string; ValueName: "{#MyAppAssocKey}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\{#MyAppAssocKey}"; ValueType: string; ValueName: ""; ValueData: "{#MyAppAssocName}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\{#MyAppAssocKey}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"
Root: HKA; Subkey: "Software\Classes\{#MyAppAssocKey}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"

