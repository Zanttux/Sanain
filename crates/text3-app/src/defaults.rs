//! Default apps (#164): make Text³ the application that opens chosen
//! groups of file types, as far as each platform allows.
//!
//! - macOS: Launch Services, per content type (UTI), for the user; the
//!   system may ask for confirmation. Only from Text³.app.
//! - Linux and other freedesktop systems: the desktop entry is written
//!   (as `--register` does), then `xdg-mime default` per MIME type.
//! - Windows: apps may not set defaults (the user's choice is protected).
//!   `--register` declares Text³ as a capable handler per extension
//!   ([`windows_reg_commands`]) and Windows Settings › Default apps is
//!   opened on Text³'s page for the user to confirm each type.
//!
//! Nothing here runs without the user asking: the first-start offer and
//! the Settings button call [`apply`].

use std::path::{Path, PathBuf};

use crate::shellreg;

/// `CFBundleIdentifier` in packaging/macos/Info.plist (kept from Sanain).
pub const MAC_BUNDLE_ID: &str = "io.github.zanttux.sanain";

/// Windows ProgID for documents Text³ opens.
pub const WINDOWS_PROGID: &str = "Text3.Document";
/// Windows capabilities key (HKCU) listed in `RegisteredApplications`.
pub const WINDOWS_CAPABILITIES: &str = r"Software\Text3\Capabilities";
/// Value name in `RegisteredApplications`; also the Settings deep link.
pub const WINDOWS_REGISTERED_NAME: &str = "Text3";

/// A group of related file types the user turns on or off together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    PlainText,
    Markdown,
    Code,
    Data,
    Web,
}

impl Group {
    pub const ALL: [Group; 5] = [
        Group::PlainText,
        Group::Markdown,
        Group::Code,
        Group::Data,
        Group::Web,
    ];

    /// Stable ASCII name (settings, command line).
    pub fn id(self) -> &'static str {
        match self {
            Group::PlainText => "plain",
            Group::Markdown => "markdown",
            Group::Code => "code",
            Group::Data => "data",
            Group::Web => "web",
        }
    }

    pub fn from_id(id: &str) -> Option<Group> {
        Group::ALL.into_iter().find(|g| g.id() == id)
    }

    /// Message id of the group's label.
    pub fn label_id(self) -> &'static str {
        match self {
            Group::PlainText => "default-apps-plain",
            Group::Markdown => "default-apps-markdown",
            Group::Code => "default-apps-code",
            Group::Data => "default-apps-data",
            Group::Web => "default-apps-web",
        }
    }

    /// Ticked in the first-start offer. Web pages are not: most people
    /// want them to open in the browser.
    pub fn preselected(self) -> bool {
        self != Group::Web
    }

    /// File name extensions, without the dot (Windows; shown as examples).
    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Group::PlainText => &["txt", "text", "log"],
            Group::Markdown => &["md", "markdown"],
            Group::Code => &[
                "c", "h", "cpp", "hpp", "cc", "cs", "go", "java", "py", "rb", "rs", "sh", "bash",
                "zsh", "ps1", "lua", "sql", "diff", "patch",
            ],
            Group::Data => &[
                "json", "xml", "yaml", "yml", "toml", "csv", "tsv", "ini", "cfg", "conf",
            ],
            Group::Web => &["html", "htm", "css", "js", "mjs", "ts"],
        }
    }

    /// MIME types (freedesktop). The first one is checked for the status.
    pub fn mime_types(self) -> &'static [&'static str] {
        match self {
            Group::PlainText => &["text/plain", "text/x-log"],
            Group::Markdown => &["text/markdown", "text/x-markdown"],
            Group::Code => &[
                "text/x-csrc",
                "text/x-chdr",
                "text/x-c++src",
                "text/x-c++hdr",
                "text/x-csharp",
                "text/x-go",
                "text/x-java",
                "text/x-python",
                "text/x-python3",
                "text/x-ruby",
                "text/rust",
                "text/x-shellscript",
                "application/x-shellscript",
                "text/x-lua",
                "application/sql",
                "text/x-patch",
                "text/x-diff",
            ],
            Group::Data => &[
                "application/json",
                "application/xml",
                "text/xml",
                "application/x-yaml",
                "application/yaml",
                "application/toml",
                "text/csv",
                "text/tab-separated-values",
            ],
            Group::Web => &[
                "text/html",
                "text/css",
                "text/javascript",
                "application/javascript",
                "text/x-typescript",
            ],
        }
    }

    /// Content types declared by macOS itself (`.ts` is an MPEG transport
    /// stream there, so TypeScript is left out). The first one is checked
    /// for the status.
    pub fn utis(self) -> &'static [&'static str] {
        match self {
            Group::PlainText => &["public.plain-text", "com.apple.log"],
            Group::Markdown => &["net.daringfireball.markdown"],
            Group::Code => &[
                "public.source-code",
                "public.c-source",
                "public.c-header",
                "public.c-plus-plus-source",
                "public.c-plus-plus-header",
                "com.microsoft.c-sharp",
                "com.sun.java-source",
                "public.python-script",
                "public.ruby-script",
                "public.shell-script",
                "public.bash-script",
                "public.zsh-script",
                "public.patch-file",
            ],
            Group::Data => &[
                "public.json",
                "public.xml",
                "public.yaml",
                "public.toml",
                "public.comma-separated-values-text",
                "public.tab-separated-values-text",
                "com.microsoft.ini",
            ],
            Group::Web => &[
                "public.html",
                "public.css",
                "com.netscape.javascript-source",
            ],
        }
    }
}

/// The groups ticked by default.
pub fn preselected() -> Vec<Group> {
    Group::ALL.into_iter().filter(|g| g.preselected()).collect()
}

/// Groups from a comma-separated list of ids; unknown ids are an error.
pub fn parse_groups(list: &str) -> Result<Vec<Group>, String> {
    let mut out = Vec::new();
    for id in list.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let g = Group::from_id(id).ok_or_else(|| id.to_owned())?;
        if !out.contains(&g) {
            out.push(g);
        }
    }
    Ok(out)
}

/// What [`apply`] did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Applied {
    /// Text³ is now the default for these groups.
    Set,
    /// Windows Settings was opened for the user to confirm.
    OpenedSettings,
}

/// What the platform lets Text³ do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Support {
    /// Defaults are set directly (macOS from the app bundle, Linux).
    Direct,
    /// The user confirms in the system settings (Windows).
    SystemSettings,
    /// Not possible from this copy (macOS outside Text³.app, portable).
    Unavailable,
}

/// What Text³ can do for the executable at `exe`.
pub fn support(exe: &Path) -> Support {
    if crate::STORE || crate::settings::portable_root_for(exe).is_some() {
        // A portable copy leaves the system alone.
        Support::Unavailable
    } else if cfg!(windows) {
        Support::SystemSettings
    } else if cfg!(target_os = "macos") {
        if crate::settings::app_bundle_of(exe).is_some() {
            Support::Direct
        } else {
            Support::Unavailable
        }
    } else {
        Support::Direct
    }
}

/// The running executable, canonical and without Windows' `\\?\`.
pub fn current_exe() -> std::io::Result<PathBuf> {
    std::env::current_exe()
        .and_then(|p| p.canonicalize())
        .map(|p| shellreg::plain_path(&p))
}

/// Make Text³ the default for `groups` (or, on Windows, register and
/// open Settings). `exe` is the running executable.
pub fn apply(exe: &Path, groups: &[Group]) -> std::io::Result<Applied> {
    match support(exe) {
        Support::Unavailable => Err(std::io::Error::other(
            "this copy of Text³ cannot change the default apps",
        )),
        Support::SystemSettings => {
            shellreg::windows_apply(exe, true)?;
            open_windows_settings()?;
            Ok(Applied::OpenedSettings)
        }
        Support::Direct if cfg!(target_os = "macos") => {
            let mut failed = Vec::new();
            for uti in groups.iter().flat_map(|g| g.utis()) {
                if let Err(e) = text3_platform::set_default_editor(uti, MAC_BUNDLE_ID) {
                    failed.push(format!("{uti}: {e}"));
                }
            }
            // Types the system does not know fail; the group's main type
            // must succeed.
            let main_failed = groups.iter().any(|g| {
                failed
                    .iter()
                    .any(|f| f.starts_with(&format!("{}:", g.utis()[0])))
            });
            if main_failed {
                return Err(std::io::Error::other(failed.join("; ")));
            }
            Ok(Applied::Set)
        }
        Support::Direct => {
            shellreg::register_freedesktop(exe, None)?;
            let status = std::process::Command::new("xdg-mime")
                .args(xdg_mime_args(groups))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()?;
            if status.success() {
                Ok(Applied::Set)
            } else {
                Err(std::io::Error::other("xdg-mime default failed"))
            }
        }
    }
}

/// `xdg-mime default <entry> <mime…>` arguments for `groups`.
pub fn xdg_mime_args(groups: &[Group]) -> Vec<String> {
    let mut args = vec!["default".to_owned(), shellreg::DESKTOP_FILE.to_owned()];
    args.extend(
        groups
            .iter()
            .flat_map(|g| g.mime_types())
            .map(|m| (*m).to_owned()),
    );
    args
}

/// Whether Text³ is already the default for the group's main type.
/// `None` when the platform does not say (Windows) or for a copy that
/// cannot change defaults.
pub fn is_default(exe: &Path, group: Group) -> Option<bool> {
    if support(exe) != Support::Direct {
        return None;
    }
    if cfg!(target_os = "macos") {
        let id = text3_platform::default_editor(group.utis()[0]);
        Some(id.is_some_and(|id| id.eq_ignore_ascii_case(MAC_BUNDLE_ID)))
    } else {
        let out = std::process::Command::new("xdg-mime")
            .args(["query", "default", group.mime_types()[0]])
            .stderr(std::process::Stdio::null())
            .output()
            .ok()?;
        Some(String::from_utf8_lossy(&out.stdout).trim() == shellreg::DESKTOP_FILE)
    }
}

/// Windows Settings › Default apps, on Text³'s page (Windows 11; Windows
/// 10 opens the general page).
pub fn windows_settings_uri() -> String {
    format!("ms-settings:defaultapps?registeredAppUser={WINDOWS_REGISTERED_NAME}")
}

fn open_windows_settings() -> std::io::Result<()> {
    // By full path: a bare name is looked up next to text3.exe first.
    let explorer =
        text3_platform::system_program("explorer.exe", text3_platform::SystemDir::Windows)
            .ok_or_else(|| std::io::Error::other("the Windows folder was not found"))?;
    std::process::Command::new(explorer)
        .arg(windows_settings_uri())
        .spawn()
        .map(|_| ())
}

/// Every extension of every group (Windows registers them all).
pub fn all_extensions() -> impl Iterator<Item = &'static str> {
    Group::ALL
        .into_iter()
        .flat_map(|g| g.extensions().iter().copied())
}

/// `reg.exe` argument lists that declare (`register`) or remove Text³ as
/// a capable handler: the ProgID, an `OpenWithProgids` entry per
/// extension, the capabilities and the `RegisteredApplications` entry.
/// Never touches `UserChoice`, so no default changes without the user.
pub fn windows_reg_commands(exe: &str, register: bool) -> Vec<Vec<String>> {
    let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
    let progid = format!(r"HKCU\Software\Classes\{WINDOWS_PROGID}");
    let caps = format!(r"HKCU\{WINDOWS_CAPABILITIES}");
    let app_root = r"HKCU\Software\Text3";
    let registered = r"HKCU\Software\RegisteredApplications";
    let open_with = |ext: &str| format!(r"HKCU\Software\Classes\.{ext}\OpenWithProgids");
    if !register {
        let mut cmds = vec![
            s(&["delete", &progid, "/f"]),
            s(&["delete", app_root, "/f"]),
            s(&["delete", registered, "/v", WINDOWS_REGISTERED_NAME, "/f"]),
        ];
        cmds.extend(
            all_extensions().map(|e| s(&["delete", &open_with(e), "/v", WINDOWS_PROGID, "/f"])),
        );
        return cmds;
    }
    let command = format!("\"{exe}\" \"%1\"");
    let icon = format!("\"{exe}\",0");
    let mut cmds = vec![
        s(&["add", &progid, "/ve", "/d", "Text document", "/f"]),
        s(&[
            "add",
            &progid,
            "/v",
            "FriendlyTypeName",
            "/d",
            "Text³",
            "/f",
        ]),
        s(&[
            "add",
            &format!(r"{progid}\DefaultIcon"),
            "/ve",
            "/d",
            &icon,
            "/f",
        ]),
        s(&[
            "add",
            &format!(r"{progid}\shell\open\command"),
            "/ve",
            "/d",
            &command,
            "/f",
        ]),
        s(&["add", &caps, "/v", "ApplicationName", "/d", "Text³", "/f"]),
        s(&[
            "add",
            &caps,
            "/v",
            "ApplicationDescription",
            "/d",
            "Native text editor",
            "/f",
        ]),
        s(&[
            "add",
            registered,
            "/v",
            WINDOWS_REGISTERED_NAME,
            "/d",
            WINDOWS_CAPABILITIES,
            "/f",
        ]),
    ];
    for ext in all_extensions() {
        cmds.push(s(&[
            "add",
            &format!(r"{caps}\FileAssociations"),
            "/v",
            &format!(".{ext}"),
            "/d",
            WINDOWS_PROGID,
            "/f",
        ]));
        cmds.push(s(&[
            "add",
            &open_with(ext),
            "/v",
            WINDOWS_PROGID,
            "/t",
            "REG_NONE",
            "/f",
        ]));
    }
    cmds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_round_trip_and_parse() {
        for g in Group::ALL {
            assert_eq!(Group::from_id(g.id()), Some(g));
            assert!(!g.extensions().is_empty() && !g.mime_types().is_empty());
            assert!(!g.utis().is_empty());
        }
        assert_eq!(
            parse_groups("plain, code,plain"),
            Ok(vec![Group::PlainText, Group::Code])
        );
        assert_eq!(parse_groups("plain,pdf"), Err("pdf".to_owned()));
        assert_eq!(parse_groups(""), Ok(vec![]));
        assert!(!preselected().contains(&Group::Web));
        assert_eq!(preselected().len(), 4);
    }

    /// `xdg-mime default` only takes types the desktop entry lists.
    #[test]
    fn desktop_entry_lists_every_group_type() {
        for m in Group::ALL.iter().flat_map(|g| g.mime_types()) {
            assert!(
                shellreg::MIME_TYPES.contains(m),
                "{m} missing from MIME_TYPES"
            );
        }
        let args = xdg_mime_args(&[Group::Markdown]);
        assert_eq!(
            args,
            [
                "default",
                shellreg::DESKTOP_FILE,
                "text/markdown",
                "text/x-markdown"
            ]
        );
    }

    /// The bundle id must match the one in Info.plist.
    #[test]
    fn mac_bundle_id_matches_info_plist() {
        let plist = include_str!("../../../packaging/macos/Info.plist");
        assert!(plist.contains(&format!(
            "<key>CFBundleIdentifier</key><string>{MAC_BUNDLE_ID}</string>"
        )));
    }

    #[test]
    fn extensions_unique_lowercase() {
        let all: Vec<_> = all_extensions().collect();
        let mut dedup = all.clone();
        dedup.sort_unstable();
        dedup.dedup();
        assert_eq!(all.len(), dedup.len(), "an extension is in two groups");
        assert!(all.iter().all(|e| {
            e.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        }));
    }

    #[test]
    fn windows_commands_only_declare() {
        let exe = r"C:\Apps\Text3\text3.exe";
        let add = windows_reg_commands(exe, true);
        assert!(
            add.iter().all(|c| c[1].starts_with(r"HKCU\")),
            "per user only"
        );
        assert!(
            add.iter().all(|c| !c[1].contains("UserChoice")),
            "never the protected choice"
        );
        assert!(add.iter().any(|c| c.contains(&format!("\"{exe}\" \"%1\""))));
        let txt = add
            .iter()
            .find(|c| c[1] == r"HKCU\Software\Classes\.txt\OpenWithProgids")
            .unwrap();
        assert_eq!(txt[3], WINDOWS_PROGID);
        assert!(
            add.iter()
                .any(|c| c[1] == r"HKCU\Software\RegisteredApplications"
                    && c.contains(&WINDOWS_CAPABILITIES.to_owned()))
        );
        let undo = windows_reg_commands(exe, false);
        assert!(undo.iter().all(|c| c[0] == "delete"));
        // Removing only Text³'s value, never the extension keys themselves.
        assert!(
            undo.iter()
                .any(|c| c[1] == r"HKCU\Software\Classes\.txt\OpenWithProgids" && c[2] == "/v")
        );
        assert!(!undo.iter().any(|c| c[1] == r"HKCU\Software\Classes\.txt"));
        assert_eq!(
            windows_settings_uri(),
            "ms-settings:defaultapps?registeredAppUser=Text3"
        );
    }

    #[test]
    fn portable_and_unbundled_copies_leave_the_system_alone() {
        let d = tempfile::tempdir().unwrap();
        let exe = d.path().join("text3");
        std::fs::write(d.path().join(crate::settings::PORTABLE_MARKER), "").unwrap();
        assert_eq!(support(&exe), Support::Unavailable);
        assert_eq!(is_default(&exe, Group::PlainText), None);
        assert!(apply(&exe, &[Group::PlainText]).is_err());
        let loose = Path::new("/opt/text3/text3");
        if crate::STORE {
            // The App Store build never changes system defaults.
            assert_eq!(support(loose), Support::Unavailable);
        } else if cfg!(target_os = "macos") {
            assert_eq!(support(loose), Support::Unavailable);
            assert_eq!(
                support(Path::new("/Applications/Text³.app/Contents/MacOS/text3")),
                Support::Direct
            );
        } else if cfg!(windows) {
            assert_eq!(support(loose), Support::SystemSettings);
        } else {
            assert_eq!(support(loose), Support::Direct);
        }
    }
}
