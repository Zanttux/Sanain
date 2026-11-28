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
; Pick the Windows display language without asking when it is one of ours.
ShowLanguageDialog=auto
OutputDir={#OutputDir}
OutputBaseFilename={#OutputName}
SetupIconFile={#RepoRoot}\packaging\icons\text3.ico
UninstallDisplayIcon={app}\text3.exe
LicenseFile={#SourceDir}\LICENSE
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
; Artwork rendered by scripts/make-icons.py (100 % and 200 % sizes).
WizardImageFile={#RepoRoot}\packaging\windows\wizard-image-100.bmp,{#RepoRoot}\packaging\windows\wizard-image-200.bmp
WizardSmallImageFile={#RepoRoot}\packaging\windows\wizard-small-100.bmp,{#RepoRoot}\packaging\windows\wizard-small-200.bmp
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
; One per interface language (locales/). Messages: packaging/windows/languages
; (Inno Setup 6.7.1 translations, see its README); English is the compiler's.
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "czech"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Czech.isl"
Name: "danish"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Danish.isl"
Name: "german"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\German.isl"
Name: "greek"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Greek.isl"
Name: "spanish"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Spanish.isl"
Name: "finnish"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Finnish.isl"
Name: "french"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\French.isl"
Name: "hungarian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Hungarian.isl"
Name: "indonesian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Indonesian.isl"
Name: "italian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Italian.isl"
Name: "japanese"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Japanese.isl"
Name: "korean"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Korean.isl"
Name: "norwegian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Norwegian.isl"
Name: "dutch"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Dutch.isl"
Name: "polish"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Polish.isl"
Name: "brazilianportuguese"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\BrazilianPortuguese.isl"
Name: "portuguese"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Portuguese.isl"
Name: "romanian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Romanian.isl"
Name: "russian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Russian.isl"
Name: "swedish"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Swedish.isl"
Name: "turkish"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Turkish.isl"
Name: "ukrainian"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Ukrainian.isl"
Name: "vietnamese"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\Vietnamese.isl"
Name: "chinesesimplified"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\ChineseSimplified.isl"
Name: "chinesetraditional"; MessagesFile: "{#RepoRoot}\packaging\windows\languages\ChineseTraditional.isl"

[CustomMessages]
; Machine translations, accepted until reviewed (D24).
english.IntegrationGroup=Integration:
english.OpenWithTask=Add %1 to "Open with" and the Explorer context menu
czech.IntegrationGroup=Integrace:
czech.OpenWithTask=Přidat %1 do nabídky „Otevřít v programu“ a do místní nabídky Průzkumníka
danish.IntegrationGroup=Integration:
danish.OpenWithTask=Tilføj %1 til "Åbn med" og Stifinderens genvejsmenu
german.IntegrationGroup=Integration:
german.OpenWithTask=%1 zu „Öffnen mit“ und zum Explorer-Kontextmenü hinzufügen
greek.IntegrationGroup=Ενσωμάτωση:
greek.OpenWithTask=Προσθήκη του %1 στο «Άνοιγμα με» και στο μενού περιβάλλοντος της Εξερεύνησης αρχείων
spanish.IntegrationGroup=Integración:
spanish.OpenWithTask=Añadir %1 a «Abrir con» y al menú contextual del Explorador
finnish.IntegrationGroup=Integrointi:
finnish.OpenWithTask=Lisää %1 Avaa sovelluksessa -valikkoon ja Resurssienhallinnan pikavalikkoon
french.IntegrationGroup=Intégration :
french.OpenWithTask=Ajouter %1 à « Ouvrir avec » et au menu contextuel de l’Explorateur
hungarian.IntegrationGroup=Integráció:
hungarian.OpenWithTask=%1 hozzáadása a „Megnyitás ezzel” menühöz és a Fájlkezelő helyi menüjéhez
indonesian.IntegrationGroup=Integrasi:
indonesian.OpenWithTask=Tambahkan %1 ke "Buka dengan" dan menu konteks Explorer
italian.IntegrationGroup=Integrazione:
italian.OpenWithTask=Aggiungi %1 ad "Apri con" e al menu contestuale di Esplora file
japanese.IntegrationGroup=統合:
japanese.OpenWithTask=%1 を [プログラムから開く] とエクスプローラーのコンテキスト メニューに追加する
korean.IntegrationGroup=통합:
korean.OpenWithTask=%1을(를) [연결 프로그램] 및 파일 탐색기 바로 가기 메뉴에 추가
norwegian.IntegrationGroup=Integrasjon:
norwegian.OpenWithTask=Legg til %1 i «Åpne med» og hurtigmenyen i Filutforsker
dutch.IntegrationGroup=Integratie:
dutch.OpenWithTask=%1 toevoegen aan "Openen met" en het snelmenu van Verkenner
polish.IntegrationGroup=Integracja:
polish.OpenWithTask=Dodaj %1 do menu „Otwórz za pomocą” i menu kontekstowego Eksploratora
brazilianportuguese.IntegrationGroup=Integração:
brazilianportuguese.OpenWithTask=Adicionar o %1 a "Abrir com" e ao menu de contexto do Explorador de Arquivos
portuguese.IntegrationGroup=Integração:
portuguese.OpenWithTask=Adicionar o %1 a "Abrir com" e ao menu de contexto do Explorador de Ficheiros
romanian.IntegrationGroup=Integrare:
romanian.OpenWithTask=Adaugă %1 la „Deschidere cu” și în meniul contextual din Explorer
russian.IntegrationGroup=Интеграция:
russian.OpenWithTask=Добавить %1 в «Открыть с помощью» и контекстное меню Проводника
swedish.IntegrationGroup=Integrering:
swedish.OpenWithTask=Lägg till %1 i ”Öppna med” och snabbmenyn i Utforskaren
turkish.IntegrationGroup=Tümleştirme:
turkish.OpenWithTask=%1 uygulamasını "Birlikte aç" menüsüne ve Dosya Gezgini bağlam menüsüne ekle
ukrainian.IntegrationGroup=Інтеграція:
ukrainian.OpenWithTask=Додати %1 до «Відкрити за допомогою» та контекстного меню Провідника
vietnamese.IntegrationGroup=Tích hợp:
vietnamese.OpenWithTask=Thêm %1 vào "Mở bằng" và menu ngữ cảnh của File Explorer
chinesesimplified.IntegrationGroup=集成：
chinesesimplified.OpenWithTask=将 %1 添加到“打开方式”和文件资源管理器的右键菜单
chinesetraditional.IntegrationGroup=整合：
chinesetraditional.OpenWithTask=將 %1 加入「開啟檔案」和檔案總管的快顯功能表

; The installer's last page offers the Default apps choice (#164):
; text3.exe --default-apps registers Text³ and opens Windows Settings.
english.DefaultAppsRun=Open files with %1
czech.DefaultAppsRun=Otevírat soubory v %1
danish.DefaultAppsRun=Åbn filer med %1
german.DefaultAppsRun=Dateien mit %1 öffnen
greek.DefaultAppsRun=Άνοιγμα αρχείων με το %1
spanish.DefaultAppsRun=Abrir archivos con %1
finnish.DefaultAppsRun=Avaa tiedostot %1:lla
french.DefaultAppsRun=Ouvrir les fichiers avec %1
hungarian.DefaultAppsRun=Fájlok megnyitása a %1 programmal
indonesian.DefaultAppsRun=Buka file dengan %1
italian.DefaultAppsRun=Apri i file con %1
japanese.DefaultAppsRun=%1 でファイルを開く
korean.DefaultAppsRun=%1 앱으로 파일 열기
norwegian.DefaultAppsRun=Åpne filer med %1
dutch.DefaultAppsRun=Bestanden openen met %1
polish.DefaultAppsRun=Otwieraj pliki w %1
brazilianportuguese.DefaultAppsRun=Abrir arquivos com o %1
portuguese.DefaultAppsRun=Abrir ficheiros com o %1
romanian.DefaultAppsRun=Deschide fișierele cu %1
russian.DefaultAppsRun=Открывать файлы в %1
swedish.DefaultAppsRun=Öppna filer med %1
turkish.DefaultAppsRun=Dosyaları %1 ile aç
ukrainian.DefaultAppsRun=Відкривати файли в %1
vietnamese.DefaultAppsRun=Mở tệp bằng %1
chinesesimplified.DefaultAppsRun=用 %1 打开文件
chinesetraditional.DefaultAppsRun=用 %1 開啟檔案

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "openwith"; Description: "{cm:OpenWithTask,{#AppName}}"; GroupDescription: "{cm:IntegrationGroup}"

[InstallDelete]
; Upgrading a Sanain installation (same AppId, same folder): remove the
; old program and its shortcuts. User data is never touched here; Text³
; copies it to its own folders on first start (docs/REBRAND.md).
Type: files; Name: "{app}\sanain.exe"
Type: files; Name: "{autoprograms}\Sanain.lnk"
Type: files; Name: "{autodesktop}\Sanain.lnk"

[Registry]
; Sanain's "Edit with Sanain" and Open with entries would point at the
; removed sanain.exe. text3.exe --register/--unregister also remove them.
Root: HKCU; Subkey: "Software\Classes\*\shell\Sanain"; ValueType: none; Flags: deletekey dontcreatekey
Root: HKCU; Subkey: "Software\Classes\Applications\sanain.exe"; ValueType: none; Flags: deletekey dontcreatekey

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\text3.exe"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\text3.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\text3.exe"; Parameters: "--register"; Tasks: openwith; Flags: runhidden
Filename: "{app}\text3.exe"; Parameters: "--default-apps"; Description: "{cm:DefaultAppsRun,{#AppName}}"; Flags: nowait postinstall skipifsilent unchecked
Filename: "{app}\text3.exe"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "{app}\text3.exe"; Parameters: "--unregister"; Flags: runhidden; RunOnceId: "Unregister"
