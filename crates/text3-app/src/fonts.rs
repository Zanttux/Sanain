//! System fonts for the editor font setting: the installed monospaced
//! families, and the bytes of a family's regular face. Fonts are only
//! read, never installed or changed.

/// Installed monospaced font families, sorted, each once. Scans the
/// system font folders, so call it off the UI thread.
pub fn monospace_families() -> Vec<String> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let mut names: Vec<String> = db
        .faces()
        .filter(|f| f.monospaced)
        .filter_map(|f| f.families.first().map(|(n, _)| n.clone()))
        .filter(|n| !n.starts_with('.'))
        .collect();
    names.sort_by_key(|n| n.to_lowercase());
    names.dedup();
    names
}

/// The regular face of `family`: (file bytes, face index in the file).
pub fn load_family(family: &str) -> Option<(Vec<u8>, u32)> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(family)],
        ..Default::default()
    };
    let id = db.query(&query)?;
    let face = db.face(id)?;
    let bytes = match &face.source {
        fontdb::Source::File(path) => std::fs::read(path).ok()?,
        fontdb::Source::Binary(data) => data.as_ref().as_ref().to_vec(),
    };
    Some((bytes, face.index))
}

#[cfg(test)]
mod tests {
    #[test]
    fn unknown_family_is_none() {
        assert!(super::load_family("No Such Font Family 1234").is_none());
    }
}
