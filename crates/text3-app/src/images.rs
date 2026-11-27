//! Loading local images for the Markdown preview, with hard limits so a
//! hostile file cannot exhaust memory (decompression bombs).

use std::path::{Path, PathBuf};

pub const MAX_FILE_BYTES: u64 = 20 * 1024 * 1024;
pub const MAX_DIMENSION: u32 = 8_192;
const MAX_ALLOC: u64 = 256 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rgba {
    pub width: u32,
    pub height: u32,
    /// Unpremultiplied RGBA8 pixels.
    pub pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    /// Message ID for the UI.
    Unsupported,
    TooLarge,
    Unreadable,
}

impl ImageError {
    pub fn message_id(&self) -> &'static str {
        match self {
            ImageError::Unsupported => "image-unsupported",
            ImageError::TooLarge => "image-too-large",
            ImageError::Unreadable => "image-unreadable",
        }
    }
}

/// Where an image reference points, decided before any file is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageSource {
    Local(PathBuf),
    /// An existing file outside the document's folder (canonical path):
    /// shown only after the user allows its folder.
    Outside(PathBuf),
    /// Remote images are never fetched.
    Remote,
    /// Outside the document's folder, or an unsupported scheme.
    Blocked,
}

/// Resolve an image URL from a document in `doc_dir`. Only files inside
/// that folder (or below it) are allowed.
pub fn resolve(url: &str, doc_dir: Option<&Path>) -> ImageSource {
    let u = url.trim();
    let lower = u.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("//") {
        return ImageSource::Remote;
    }
    if lower.contains(':') && !(lower.len() > 2 && lower.as_bytes()[1] == b':') {
        return ImageSource::Blocked; // data:, file:, javascript:, …
    }
    if text3_markdown::is_network_path(u) {
        // `\\host\share\x.png`: even resolving it would send the user's
        // Windows credentials to that host, before anything is shown.
        return ImageSource::Blocked;
    }
    let Some(dir) = doc_dir else {
        return ImageSource::Blocked;
    };
    let rel = u.split(['#', '?']).next().unwrap_or("").replace("%20", " ");
    let candidate = dir.join(rel);
    match (candidate.canonicalize(), dir.canonicalize()) {
        (Ok(file), Ok(root)) if file.starts_with(&root) && file.is_file() => {
            ImageSource::Local(file)
        }
        (Ok(file), Ok(_)) if file.is_file() => ImageSource::Outside(file),
        (Ok(_), Ok(_)) => ImageSource::Blocked,
        _ => ImageSource::Local(candidate), // missing: reported as unreadable
    }
}

/// Read and decode an image within the limits.
pub fn load(path: &Path) -> Result<Rgba, ImageError> {
    let meta = std::fs::metadata(path).map_err(|_| ImageError::Unreadable)?;
    if meta.len() > MAX_FILE_BYTES {
        return Err(ImageError::TooLarge);
    }
    let bytes = std::fs::read(path).map_err(|_| ImageError::Unreadable)?;
    decode(&bytes)
}

pub fn decode(bytes: &[u8]) -> Result<Rgba, ImageError> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|_| ImageError::Unreadable)?;
    if reader.format().is_none() {
        return Err(ImageError::Unsupported);
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DIMENSION);
    limits.max_image_height = Some(MAX_DIMENSION);
    limits.max_alloc = Some(MAX_ALLOC);
    reader.limits(limits);
    let img = reader.decode().map_err(|e| match e {
        image::ImageError::Limits(_) => ImageError::TooLarge,
        image::ImageError::Unsupported(_) => ImageError::Unsupported,
        _ => ImageError::Unreadable,
    })?;
    let rgba = img.to_rgba8();
    Ok(Rgba {
        width: rgba.width(),
        height: rgba.height(),
        pixels: rgba.into_raw(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(w: u32, h: u32) -> Vec<u8> {
        let img = image::RgbaImage::from_pixel(w, h, image::Rgba([10, 20, 30, 255]));
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    #[test]
    fn decodes_png() {
        let r = decode(&png(3, 2)).unwrap();
        assert_eq!((r.width, r.height, r.pixels.len()), (3, 2, 24));
        assert_eq!(&r.pixels[..4], &[10, 20, 30, 255]);
    }

    #[test]
    fn refuses_huge_dimensions_and_garbage() {
        assert_eq!(
            decode(&png(MAX_DIMENSION + 1, 1)),
            Err(ImageError::TooLarge)
        );
        assert_eq!(decode(b"not an image"), Err(ImageError::Unsupported));
        assert_eq!(decode(&png(4, 4)[..20]), Err(ImageError::Unreadable));
    }

    #[test]
    fn resolve_stays_inside_the_document_folder() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("img")).unwrap();
        std::fs::write(dir.path().join("img/a.png"), png(1, 1)).unwrap();
        let outside = tempfile::tempdir().unwrap();
        std::fs::write(outside.path().join("x.png"), png(1, 1)).unwrap();
        let d = Some(dir.path());
        assert!(matches!(resolve("img/a.png", d), ImageSource::Local(_)));
        assert!(
            matches!(resolve("img/a%20b.png", d), ImageSource::Local(_)),
            "missing → unreadable later"
        );
        assert_eq!(resolve("https://x.org/a.png", d), ImageSource::Remote);
        assert_eq!(
            resolve("data:image/png;base64,AAAA", d),
            ImageSource::Blocked
        );
        assert_eq!(resolve("file:///etc/passwd", d), ImageSource::Blocked);
        let escape = format!(
            "../{}/x.png",
            outside.path().file_name().unwrap().to_string_lossy()
        );
        assert_eq!(
            resolve(&escape, d),
            ImageSource::Outside(outside.path().join("x.png").canonicalize().unwrap())
        );
        assert_eq!(resolve("a.png", None), ImageSource::Blocked);
        for unc in [
            r"\\attacker\share\x.png",
            r"\\?\UNC\attacker\share\x.png",
            r"/\attacker\share\x.png",
        ] {
            assert_eq!(resolve(unc, d), ImageSource::Blocked, "{unc}");
        }
    }
}
