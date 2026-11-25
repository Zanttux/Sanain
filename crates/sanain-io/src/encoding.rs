/// Encodings supported by the first slice. Legacy single-byte and CJK
/// encodings arrive with explicit user selection in a later slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Utf8Bom,
    Utf16LeBom,
    Utf16BeBom,
}

const BOM_UTF8: &[u8] = &[0xEF, 0xBB, 0xBF];
const BOM_UTF16LE: &[u8] = &[0xFF, 0xFE];
const BOM_UTF16BE: &[u8] = &[0xFE, 0xFF];

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    /// Not valid UTF-8 and no BOM identified another encoding. The caller
    /// must ask the user how to proceed; nothing has been replaced.
    #[error("invalid UTF-8 at byte {offset}")]
    InvalidUtf8 { offset: usize },
    #[error("UTF-16 data has an odd number of bytes")]
    OddUtf16Length,
    #[error("unpaired UTF-16 surrogate at byte {offset}")]
    UnpairedSurrogate { offset: usize },
}

/// Detect the encoding from the BOM and decode strictly.
pub fn decode(bytes: &[u8]) -> Result<(String, TextEncoding), DecodeError> {
    if let Some(rest) = bytes.strip_prefix(BOM_UTF8) {
        return utf8(rest, BOM_UTF8.len()).map(|s| (s, TextEncoding::Utf8Bom));
    }
    if let Some(rest) = bytes.strip_prefix(BOM_UTF16LE) {
        return utf16(rest, u16::from_le_bytes).map(|s| (s, TextEncoding::Utf16LeBom));
    }
    if let Some(rest) = bytes.strip_prefix(BOM_UTF16BE) {
        return utf16(rest, u16::from_be_bytes).map(|s| (s, TextEncoding::Utf16BeBom));
    }
    utf8(bytes, 0).map(|s| (s, TextEncoding::Utf8))
}

/// Encode `text`, including the BOM where the encoding has one.
pub fn encode(text: &str, encoding: TextEncoding) -> Vec<u8> {
    match encoding {
        TextEncoding::Utf8 => text.as_bytes().to_vec(),
        TextEncoding::Utf8Bom => [BOM_UTF8, text.as_bytes()].concat(),
        TextEncoding::Utf16LeBom => with_units(BOM_UTF16LE, text, u16::to_le_bytes),
        TextEncoding::Utf16BeBom => with_units(BOM_UTF16BE, text, u16::to_be_bytes),
    }
}

fn utf8(bytes: &[u8], base: usize) -> Result<String, DecodeError> {
    String::from_utf8(bytes.to_vec()).map_err(|e| DecodeError::InvalidUtf8 {
        offset: base + e.utf8_error().valid_up_to(),
    })
}

fn utf16(bytes: &[u8], read: fn([u8; 2]) -> u16) -> Result<String, DecodeError> {
    let (pairs, rest) = bytes.as_chunks::<2>();
    if !rest.is_empty() {
        return Err(DecodeError::OddUtf16Length);
    }
    let units = pairs.iter().map(|&pair| read(pair));
    let mut out = String::with_capacity(bytes.len() / 2);
    let mut offset = 2; // after the BOM
    for r in char::decode_utf16(units) {
        match r {
            Ok(c) => {
                offset += c.len_utf16() * 2;
                out.push(c);
            }
            Err(_) => return Err(DecodeError::UnpairedSurrogate { offset }),
        }
    }
    Ok(out)
}

fn with_units(bom: &[u8], text: &str, write: fn(u16) -> [u8; 2]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bom.len() + text.len() * 2);
    out.extend_from_slice(bom);
    for u in text.encode_utf16() {
        out.extend_from_slice(&write(u));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(bytes: &[u8], expected: TextEncoding) {
        let (text, enc) = decode(bytes).expect("decodes");
        assert_eq!(enc, expected);
        assert_eq!(encode(&text, enc), bytes, "bytes must round-trip");
    }

    #[test]
    fn round_trips_supported_encodings() {
        round_trip(b"", TextEncoding::Utf8);
        round_trip("a\r\nb\nc\r é🙂".as_bytes(), TextEncoding::Utf8);
        round_trip(b"\xEF\xBB\xBFhi\r\n", TextEncoding::Utf8Bom);
        round_trip(b"\xEF\xBB\xBF", TextEncoding::Utf8Bom);
        round_trip(
            &encode("a\r\n🙂", TextEncoding::Utf16LeBom),
            TextEncoding::Utf16LeBom,
        );
        round_trip(
            &encode("a\n漢", TextEncoding::Utf16BeBom),
            TextEncoding::Utf16BeBom,
        );
    }

    #[test]
    fn utf16le_known_bytes() {
        let (t, e) = decode(b"\xFF\xFEa\x00\n\x00").unwrap();
        assert_eq!((t.as_str(), e), ("a\n", TextEncoding::Utf16LeBom));
    }

    #[test]
    fn invalid_utf8_is_reported_not_replaced() {
        assert_eq!(
            decode(b"ok\xFFno"),
            Err(DecodeError::InvalidUtf8 { offset: 2 })
        );
        assert_eq!(
            decode(b"\xEF\xBB\xBFx\xC3"),
            Err(DecodeError::InvalidUtf8 { offset: 4 })
        );
    }

    #[test]
    fn malformed_utf16_is_reported() {
        assert_eq!(decode(b"\xFF\xFEa"), Err(DecodeError::OddUtf16Length));
        // Lone high surrogate D800 after "a".
        assert_eq!(
            decode(b"\xFF\xFEa\x00\x00\xD8"),
            Err(DecodeError::UnpairedSurrogate { offset: 4 })
        );
    }
}
