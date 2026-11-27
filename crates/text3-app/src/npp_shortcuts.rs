//! Import keyboard shortcuts from a Notepad++ `shortcuts.xml`.
//!
//! Notepad++ (Windows) keeps the file at `%AppData%\Notepad++\shortcuts.xml`.
//! It lists only the shortcuts the user changed:
//!
//! ```xml
//! <NotepadPlus>
//!   <InternalCommands>
//!     <Shortcut id="42010" Ctrl="yes" Alt="no" Shift="no" Key="68" />
//!   </InternalCommands>
//!   <ScintillaKeys>
//!     <ScintKey ScintID="2338" menuCmdID="0" Ctrl="yes" Alt="no" Shift="yes" Key="76" />
//!   </ScintillaKeys>
//! </NotepadPlus>
//! ```
//!
//! `id` is a Notepad++ menu command (`IDM_*` in `menuCmdID.h`), `ScintID` a
//! Scintilla message (`SCI_*`), and `Key` a Windows virtual-key code;
//! `Key="0"` means the shortcut was removed. Only commands Sanain has are
//! mapped; macros, run commands, plugin commands and unknown ids or keys are
//! counted as skipped. The input is size-capped, entities are never
//! expanded (attribute values are read raw) and parsing never panics.

use crate::keymap::{Chord, Mods};

/// Largest file accepted.
pub const MAX_BYTES: usize = 1024 * 1024;
const MAX_DEPTH: usize = 32;

/// Notepad++ menu command ids (`menuCmdID.h`) → Sanain command ids.
pub const INTERNAL_COMMANDS: &[(u32, &str)] = &[
    (41001, "file.new"),            // IDM_FILE_NEW
    (41002, "file.open"),           // IDM_FILE_OPEN
    (41003, "file.close-tab"),      // IDM_FILE_CLOSE
    (41004, "file.close-all"),      // IDM_FILE_CLOSEALL
    (41005, "file.close-others"),   // IDM_FILE_CLOSEALL_BUT_CURRENT
    (41006, "file.save"),           // IDM_FILE_SAVE
    (41007, "file.save-all"),       // IDM_FILE_SAVEALL
    (41008, "file.save-as"),        // IDM_FILE_SAVEAS
    (42001, "edit.cut"),            // IDM_EDIT_CUT
    (42002, "edit.copy"),           // IDM_EDIT_COPY
    (42003, "edit.undo"),           // IDM_EDIT_UNDO
    (42004, "edit.redo"),           // IDM_EDIT_REDO
    (42005, "edit.paste"),          // IDM_EDIT_PASTE
    (42007, "edit.select-all"),     // IDM_EDIT_SELECTALL
    (42010, "edit.duplicate-line"), // IDM_EDIT_DUP_LINE
    (42014, "edit.move-line-up"),   // IDM_EDIT_LINE_UP
    (42015, "edit.move-line-down"), // IDM_EDIT_LINE_DOWN
    (42016, "tools.upper-case"),    // IDM_EDIT_UPPERCASE
    (42017, "tools.lower-case"),    // IDM_EDIT_LOWERCASE
    (42022, "edit.toggle-comment"), // IDM_EDIT_BLOCK_COMMENT (toggle line comment)
    (43001, "edit.find"),           // IDM_SEARCH_FIND
    (43002, "edit.find-next"),      // IDM_SEARCH_FINDNEXT
    (43003, "edit.replace"),        // IDM_SEARCH_REPLACE
    (43004, "edit.go-to-line"),     // IDM_SEARCH_GOTOLINE
    (43010, "edit.find-prev"),      // IDM_SEARCH_FINDPREV
    (43013, "edit.find-in-folder"), // IDM_SEARCH_FINDINFILES
    (44023, "view.zoom-in"),        // IDM_VIEW_ZOOMIN
    (44024, "view.zoom-out"),       // IDM_VIEW_ZOOMOUT
    (44029, "view.unfold-all"),     // IDM_VIEW_TOGGLE_UNFOLDALL
    (44030, "view.fold"),           // IDM_VIEW_FOLD_CURRENT
    (44031, "view.unfold"),         // IDM_VIEW_UNFOLD_CURRENT
];

/// Scintilla messages (`Scintilla.h`) → Sanain command ids, for
/// `<ScintKey>` entries that are not tied to a menu command.
pub const SCINTILLA_COMMANDS: &[(u32, &str)] = &[
    (2011, "edit.redo"),        // SCI_REDO
    (2013, "edit.select-all"),  // SCI_SELECTALL
    (2176, "edit.undo"),        // SCI_UNDO
    (2177, "edit.cut"),         // SCI_CUT
    (2178, "edit.copy"),        // SCI_COPY
    (2179, "edit.paste"),       // SCI_PASTE
    (2333, "view.zoom-in"),     // SCI_ZOOMIN
    (2334, "view.zoom-out"),    // SCI_ZOOMOUT
    (2338, "edit.delete-line"), // SCI_LINEDELETE
];

/// Windows virtual-key code → Sanain key name.
pub fn key_name(vk: u32) -> Option<String> {
    let named = match vk {
        0x08 => "Backspace",
        0x09 => "Tab",
        0x0D => "Enter",
        0x1B => "Escape",
        0x20 => "Space",
        0x21 => "PageUp",
        0x22 => "PageDown",
        0x23 => "End",
        0x24 => "Home",
        0x25 => "ArrowLeft",
        0x26 => "ArrowUp",
        0x27 => "ArrowRight",
        0x28 => "ArrowDown",
        0x2E => "Delete",
        // Digits and letters are their own names; numpad digits too.
        0x30..=0x39 | 0x41..=0x5A => {
            return char::from_u32(vk).map(String::from);
        }
        0x60..=0x69 => return char::from_u32(vk - 0x30).map(String::from),
        0x6B => "Plus",  // VK_ADD (numpad +)
        0x6D => "Minus", // VK_SUBTRACT (numpad -)
        0x6F => "Slash", // VK_DIVIDE (numpad /)
        0x70..=0x7B => return Some(format!("F{}", vk - 0x6F)),
        0xBA => "Semicolon",    // VK_OEM_1 (US ;:)
        0xBB => "Equals",       // VK_OEM_PLUS (US =+)
        0xBC => "Comma",        // VK_OEM_COMMA
        0xBD => "Minus",        // VK_OEM_MINUS
        0xBE => "Period",       // VK_OEM_PERIOD
        0xBF => "Slash",        // VK_OEM_2 (US /?)
        0xDB => "OpenBracket",  // VK_OEM_4 (US [{)
        0xDD => "CloseBracket", // VK_OEM_6 (US ]})
        _ => return None,
    };
    Some(named.to_owned())
}

/// What an import found.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Import {
    /// (Sanain command id, chord or `None` for a removed shortcut), in file
    /// order; a command appears once (its last entry wins).
    pub entries: Vec<(&'static str, Option<Chord>)>,
    /// Entries that could not be used (unknown command or key, macros, run
    /// and plugin commands, plain keys without a modifier).
    pub skipped: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportError {
    TooLarge,
    /// Not well-formed XML, not UTF-8, or not a Notepad++ shortcuts file.
    Invalid,
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooLarge => "file too large",
            Self::Invalid => "not a Notepad++ shortcuts.xml",
        })
    }
}

impl std::error::Error for ImportError {}

#[derive(Default)]
struct Attrs {
    id: Option<u32>,
    scint_id: Option<u32>,
    menu_cmd_id: Option<u32>,
    ctrl: bool,
    alt: bool,
    shift: bool,
    key: Option<u32>,
}

fn read_attrs(e: &quick_xml::events::BytesStart<'_>) -> Result<Attrs, ImportError> {
    let mut a = Attrs::default();
    for attr in e.attributes() {
        let attr = attr.map_err(|_| ImportError::Invalid)?;
        // Raw value: the attributes used here are numbers and yes/no, so
        // no unescaping (and no entity handling) is needed.
        let value = attr.value.trim();
        let num = || value.parse::<u32>().ok();
        let yes = value.eq_ignore_ascii_case("yes");
        match attr.key.as_ref() {
            "id" => a.id = num(),
            "ScintID" => a.scint_id = num(),
            "menuCmdID" => a.menu_cmd_id = num(),
            "Ctrl" => a.ctrl = yes,
            "Alt" => a.alt = yes,
            "Shift" => a.shift = yes,
            "Key" => a.key = num(),
            _ => {}
        }
    }
    Ok(a)
}

fn lookup(table: &[(u32, &'static str)], id: u32) -> Option<&'static str> {
    table.iter().find(|(n, _)| *n == id).map(|(_, c)| *c)
}

/// The chord for an entry: `Ok(None)` for a removed shortcut, `Err(())`
/// for a key Sanain cannot bind.
fn chord(a: &Attrs) -> Result<Option<Chord>, ()> {
    let vk = a.key.ok_or(())?;
    if vk == 0 {
        return Ok(None);
    }
    let key = key_name(vk).ok_or(())?;
    let function_key = key.len() > 1 && key.starts_with('F');
    // A plain or Shift-only key would capture typing.
    if !(a.ctrl || a.alt || function_key) {
        return Err(());
    }
    let mods = Mods {
        command: a.ctrl,
        ctrl: false,
        alt: a.alt,
        shift: a.shift,
    };
    // Round-trip through the parser so only keys the keymap accepts pass.
    Chord::parse(&Chord { mods, key }.format())
        .map(Some)
        .ok_or(())
}

/// Parse a Notepad++ `shortcuts.xml`.
pub fn parse(bytes: &[u8]) -> Result<Import, ImportError> {
    use quick_xml::events::Event;
    if bytes.len() > MAX_BYTES {
        return Err(ImportError::TooLarge);
    }
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    let text = std::str::from_utf8(bytes).map_err(|_| ImportError::Invalid)?;
    let mut reader = quick_xml::Reader::from_str(text);
    reader.config_mut().check_end_names = true;
    let mut stack: Vec<String> = Vec::new();
    let mut seen_root = false;
    let mut out = Import::default();
    loop {
        let event = reader.read_event().map_err(|_| ImportError::Invalid)?;
        let (e, empty) = match event {
            Event::Start(e) => (e, false),
            Event::Empty(e) => (e, true),
            Event::End(_) => {
                stack.pop().ok_or(ImportError::Invalid)?;
                continue;
            }
            Event::Eof => break,
            _ => continue,
        };
        let name = e.name().as_ref().to_owned();
        if stack.is_empty() {
            if seen_root || name != "NotepadPlus" {
                return Err(ImportError::Invalid);
            }
            seen_root = true;
        } else {
            let section = stack.get(1).map(String::as_str);
            let in_section = stack.len() == 2;
            match (section, name.as_str()) {
                (Some("InternalCommands"), "Shortcut") if in_section => {
                    let a = read_attrs(&e)?;
                    let cmd = a.id.and_then(|id| lookup(INTERNAL_COMMANDS, id));
                    record(&mut out, cmd, &a);
                }
                (Some("ScintillaKeys"), "ScintKey") if in_section => {
                    let a = read_attrs(&e)?;
                    let cmd = match a.menu_cmd_id {
                        Some(m) if m != 0 => lookup(INTERNAL_COMMANDS, m),
                        _ => a.scint_id.and_then(|s| lookup(SCINTILLA_COMMANDS, s)),
                    };
                    record(&mut out, cmd, &a);
                }
                (Some("Macros"), "Macro")
                | (Some("UserDefinedCommands"), "Command")
                | (Some("PluginCommands"), "PluginCommand")
                    if in_section =>
                {
                    out.skipped += 1;
                }
                _ => {}
            }
        }
        if !empty {
            if stack.len() >= MAX_DEPTH {
                return Err(ImportError::Invalid);
            }
            stack.push(name);
        }
    }
    if !seen_root || !stack.is_empty() {
        return Err(ImportError::Invalid);
    }
    Ok(out)
}

fn record(out: &mut Import, cmd: Option<&'static str>, a: &Attrs) {
    match (cmd, chord(a)) {
        (Some(cmd), Ok(c)) => {
            out.entries.retain(|(id, _)| *id != cmd);
            out.entries.push((cmd, c));
        }
        _ => out.skipped += 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chord(s: &str) -> Option<Chord> {
        Some(Chord::parse(s).unwrap())
    }

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<NotepadPlus>
    <InternalCommands>
        <Shortcut id="42010" Ctrl="yes" Alt="no" Shift="no" Key="68" />
        <Shortcut id="43002" Ctrl="no" Alt="no" Shift="no" Key="114" />
        <Shortcut id="43010" Ctrl="no" Alt="no" Shift="yes" Key="114" />
        <Shortcut id="44030" Ctrl="yes" Alt="yes" Shift="no" Key="70" />
        <Shortcut id="44023" Ctrl="yes" Alt="no" Shift="no" Key="107" />
        <Shortcut id="41006" Ctrl="no" Alt="no" Shift="no" Key="0" />
        <Shortcut id="99999" Ctrl="yes" Alt="no" Shift="no" Key="65" />
        <Shortcut id="42014" Ctrl="yes" Alt="no" Shift="no" Key="220" />
        <Shortcut id="42016" Ctrl="no" Alt="no" Shift="yes" Key="85" />
    </InternalCommands>
    <Macros>
        <Macro name="Trim" Ctrl="no" Alt="yes" Shift="yes" Key="84">
            <Action type="2" message="0" wParam="42024" lParam="0" sParam="" />
        </Macro>
    </Macros>
    <UserDefinedCommands>
        <Command name="Launch" Ctrl="yes" Alt="yes" Shift="yes" Key="82">x</Command>
    </UserDefinedCommands>
    <PluginCommands />
    <ScintillaKeys>
        <ScintKey ScintID="2338" menuCmdID="0" Ctrl="yes" Alt="no" Shift="yes" Key="76" />
        <ScintKey ScintID="2177" menuCmdID="42001" Ctrl="yes" Alt="yes" Shift="no" Key="88">
            <NextKey Ctrl="no" Alt="no" Shift="yes" Key="46" />
        </ScintKey>
        <ScintKey ScintID="2404" menuCmdID="0" Ctrl="yes" Alt="no" Shift="no" Key="68" />
    </ScintillaKeys>
</NotepadPlus>
"#;

    #[test]
    fn parses_sample() {
        let got = parse(SAMPLE.as_bytes()).unwrap();
        assert_eq!(
            got.entries,
            vec![
                ("edit.duplicate-line", chord("Mod+D")),
                ("edit.find-next", chord("F3")),
                ("edit.find-prev", chord("Shift+F3")),
                ("view.fold", chord("Mod+Alt+F")),
                ("view.zoom-in", chord("Mod+Plus")),
                ("file.save", None),
                ("edit.delete-line", chord("Mod+Shift+L")),
                ("edit.cut", chord("Mod+Alt+X")),
            ]
        );
        // Unknown id 99999, Backslash key 220, Shift+U without Ctrl/Alt,
        // one macro, one run command, SCI_LINEDUPLICATE (not mapped).
        assert_eq!(got.skipped, 6);
    }

    #[test]
    fn last_entry_for_a_command_wins() {
        let xml = r#"<NotepadPlus><InternalCommands>
            <Shortcut id="42001" Ctrl="yes" Alt="no" Shift="no" Key="88"/>
            </InternalCommands><ScintillaKeys>
            <ScintKey ScintID="2177" menuCmdID="0" Ctrl="yes" Alt="no" Shift="yes" Key="88"/>
            </ScintillaKeys></NotepadPlus>"#;
        let got = parse(xml.as_bytes()).unwrap();
        assert_eq!(got.entries, vec![("edit.cut", chord("Mod+Shift+X"))]);
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            "",
            "<NotepadPlus><InternalCommands></NotepadPlus>",
            "<NotepadPlus>",
            "<Other><InternalCommands/></Other>",
            "<NotepadPlus/><NotepadPlus/>",
            "<NotepadPlus><InternalCommands><Shortcut id=\"1\" id=\"2\"/></InternalCommands></NotepadPlus>",
            "<NotepadPlus><InternalCommands><Shortcut id=42010/></InternalCommands></NotepadPlus>",
            "not xml at all",
        ] {
            assert_eq!(parse(bad.as_bytes()), Err(ImportError::Invalid), "{bad}");
        }
        assert_eq!(
            parse(b"<NotepadPlus>\xff</NotepadPlus>"),
            Err(ImportError::Invalid)
        );
        let deep = format!("<NotepadPlus>{}", "<a>".repeat(100));
        assert_eq!(parse(deep.as_bytes()), Err(ImportError::Invalid));
    }

    #[test]
    fn size_cap() {
        let mut big = String::from("<NotepadPlus>");
        big.push_str(&" ".repeat(MAX_BYTES));
        big.push_str("</NotepadPlus>");
        assert_eq!(parse(big.as_bytes()), Err(ImportError::TooLarge));
        let ok = format!("\u{feff}<NotepadPlus>{}</NotepadPlus>", " ".repeat(1000));
        assert_eq!(parse(ok.as_bytes()), Ok(Import::default()));
    }

    #[test]
    fn entities_are_not_expanded() {
        let xml = r#"<?xml version="1.0"?>
<!DOCTYPE NotepadPlus [<!ENTITY k "68"><!ENTITY a "&k;&k;&k;&k;">]>
<NotepadPlus><InternalCommands>
<Shortcut id="42010" Ctrl="yes" Alt="no" Shift="no" Key="&k;"/>
</InternalCommands></NotepadPlus>"#;
        let got = parse(xml.as_bytes()).unwrap();
        assert!(got.entries.is_empty());
        assert_eq!(got.skipped, 1);
    }

    #[test]
    fn virtual_keys() {
        for (vk, name) in [
            (65, "A"),
            (90, "Z"),
            (48, "0"),
            (57, "9"),
            (0x60, "0"),
            (0x69, "9"),
            (112, "F1"),
            (123, "F12"),
            (37, "ArrowLeft"),
            (38, "ArrowUp"),
            (39, "ArrowRight"),
            (40, "ArrowDown"),
            (36, "Home"),
            (35, "End"),
            (33, "PageUp"),
            (34, "PageDown"),
            (9, "Tab"),
            (13, "Enter"),
            (27, "Escape"),
            (32, "Space"),
            (8, "Backspace"),
            (46, "Delete"),
            (0xBC, "Comma"),
            (0xBE, "Period"),
            (0xBF, "Slash"),
            (0xBA, "Semicolon"),
            (0xBB, "Equals"),
            (0xBD, "Minus"),
            (0xDB, "OpenBracket"),
            (0xDD, "CloseBracket"),
            (0x6B, "Plus"),
            (0x6D, "Minus"),
        ] {
            let got = key_name(vk);
            assert_eq!(got.as_deref(), Some(name), "{vk}");
            assert!(Chord::parse(&format!("Mod+{name}")).is_some(), "{name}");
        }
        for vk in [
            0,
            1,
            0x10,
            0x11,
            0x12,
            0x5B,
            0x7C,
            0xDC,
            0xDE,
            0xC0,
            300,
            u32::MAX,
        ] {
            assert_eq!(key_name(vk), None, "{vk}");
        }
    }

    #[test]
    fn tables_have_unique_notepadpp_ids() {
        for table in [INTERNAL_COMMANDS, SCINTILLA_COMMANDS] {
            for (i, (n, _)) in table.iter().enumerate() {
                assert!(table[i + 1..].iter().all(|(m, _)| m != n), "{n}");
            }
        }
    }
}
