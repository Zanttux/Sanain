/// How a document's text maps to bytes on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Utf8Bom,
    Utf16LeBom,
    Utf16BeBom,
    /// UTF-16 without a byte order mark (detected or chosen; saved without
    /// one, as found).
    Utf16Le,
    Utf16Be,
    /// A non-Unicode encoding the user chose explicitly. Only offered for
    /// files whose bytes it reproduces exactly.
    Legacy(&'static encoding_rs::Encoding),
}

impl TextEncoding {
    /// Name for the status bar (encoding names are not translated).
    pub fn name(self) -> &'static str {
        match self {
            TextEncoding::Utf8 => "UTF-8",
            TextEncoding::Utf8Bom => "UTF-8 BOM",
            TextEncoding::Utf16LeBom => "UTF-16 LE BOM",
            TextEncoding::Utf16BeBom => "UTF-16 BE BOM",
            TextEncoding::Utf16Le => "UTF-16 LE",
            TextEncoding::Utf16Be => "UTF-16 BE",
            TextEncoding::Legacy(e) => e.name(),
        }
    }
}

/// Legacy encodings offered for "Reopen with Encoding", most common first.
pub const LEGACY: &[&encoding_rs::Encoding] = &[
    encoding_rs::WINDOWS_1252,
    encoding_rs::ISO_8859_15,
    encoding_rs::WINDOWS_1250,
    encoding_rs::ISO_8859_2,
    encoding_rs::WINDOWS_1251,
    encoding_rs::KOI8_R,
    encoding_rs::WINDOWS_1253,
    encoding_rs::WINDOWS_1254,
    encoding_rs::WINDOWS_1255,
    encoding_rs::WINDOWS_1256,
    encoding_rs::WINDOWS_1257,
    encoding_rs::WINDOWS_874,
    encoding_rs::SHIFT_JIS,
    encoding_rs::EUC_JP,
    encoding_rs::GB18030,
    encoding_rs::BIG5,
    encoding_rs::EUC_KR,
];

/// A character that the target encoding cannot represent.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{character:?} (character {index}) cannot be saved as {encoding}")]
pub struct EncodeError {
    pub character: char,
    /// Char index in the text.
    pub index: usize,
    pub encoding: &'static str,
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
    #[error("UTF-32 is not supported")]
    Utf32,
    #[error("byte {offset} is not valid in {encoding}")]
    Malformed {
        offset: usize,
        encoding: &'static str,
    },
    /// Decoding works but saving would not reproduce the same bytes.
    #[error("{encoding} cannot reproduce this file exactly")]
    NotReversible { encoding: &'static str },
}

/// Detect the encoding from the BOM and decode strictly.
pub fn decode(bytes: &[u8]) -> Result<(String, TextEncoding), DecodeError> {
    // UTF-32 BOMs; the LE one starts with the UTF-16 LE BOM, so check first.
    if bytes.starts_with(&[0xFF, 0xFE, 0, 0]) || bytes.starts_with(&[0, 0, 0xFE, 0xFF]) {
        return Err(DecodeError::Utf32);
    }
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

/// Decode with an encoding the user chose. Never replaces bytes; a legacy
/// encoding is accepted only if encoding the result gives back `bytes`.
pub fn decode_as(bytes: &[u8], encoding: TextEncoding) -> Result<String, DecodeError> {
    match encoding {
        TextEncoding::Utf8 => utf8(bytes, 0),
        TextEncoding::Utf8Bom => utf8(
            bytes.strip_prefix(BOM_UTF8).unwrap_or(bytes),
            BOM_UTF8.len(),
        ),
        TextEncoding::Utf16LeBom => utf16(
            bytes.strip_prefix(BOM_UTF16LE).unwrap_or(bytes),
            u16::from_le_bytes,
        ),
        TextEncoding::Utf16BeBom => utf16(
            bytes.strip_prefix(BOM_UTF16BE).unwrap_or(bytes),
            u16::from_be_bytes,
        ),
        TextEncoding::Utf16Le => utf16(bytes, u16::from_le_bytes),
        TextEncoding::Utf16Be => utf16(bytes, u16::from_be_bytes),
        TextEncoding::Legacy(enc) => {
            let mut decoder = enc.new_decoder_without_bom_handling();
            let cap = decoder
                .max_utf8_buffer_length_without_replacement(bytes.len())
                .unwrap_or(bytes.len().saturating_mul(3));
            let mut out = String::with_capacity(cap);
            let (result, read) =
                decoder.decode_to_string_without_replacement(bytes, &mut out, true);
            match result {
                encoding_rs::DecoderResult::InputEmpty => {}
                encoding_rs::DecoderResult::Malformed(bad, _) => {
                    return Err(DecodeError::Malformed {
                        offset: read.saturating_sub(usize::from(bad)),
                        encoding: enc.name(),
                    });
                }
                encoding_rs::DecoderResult::OutputFull => {
                    return Err(DecodeError::Malformed {
                        offset: read,
                        encoding: enc.name(),
                    });
                }
            }
            if encode(&out, encoding).ok().as_deref() != Some(bytes) {
                return Err(DecodeError::NotReversible {
                    encoding: enc.name(),
                });
            }
            Ok(out)
        }
    }
}

/// Encode `text`, including the BOM where the encoding has one. Fails if a
/// character cannot be represented: nothing is ever substituted.
pub fn encode(text: &str, encoding: TextEncoding) -> Result<Vec<u8>, EncodeError> {
    Ok(match encoding {
        TextEncoding::Utf8 => text.as_bytes().to_vec(),
        TextEncoding::Utf8Bom => [BOM_UTF8, text.as_bytes()].concat(),
        TextEncoding::Utf16LeBom => with_units(BOM_UTF16LE, text, u16::to_le_bytes),
        TextEncoding::Utf16BeBom => with_units(BOM_UTF16BE, text, u16::to_be_bytes),
        TextEncoding::Utf16Le => with_units(&[], text, u16::to_le_bytes),
        TextEncoding::Utf16Be => with_units(&[], text, u16::to_be_bytes),
        TextEncoding::Legacy(enc) => {
            let mut encoder = enc.new_encoder();
            let mut out = Vec::with_capacity(text.len());
            let mut rest = text;
            let mut chars_done = 0;
            loop {
                out.reserve(rest.len() * 4 + 16);
                let (result, read) =
                    encoder.encode_from_utf8_to_vec_without_replacement(rest, &mut out, true);
                match result {
                    encoding_rs::EncoderResult::InputEmpty => break,
                    encoding_rs::EncoderResult::OutputFull => {
                        chars_done += rest[..read].chars().count();
                        rest = &rest[read..];
                    }
                    encoding_rs::EncoderResult::Unmappable(c) => {
                        let index = chars_done + rest[..read].chars().count() - 1;
                        return Err(EncodeError {
                            character: c,
                            index,
                            encoding: enc.name(),
                        });
                    }
                }
            }
            out
        }
    })
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

/// Guess the encoding of bytes that are not valid UTF-8 and have no BOM:
/// UTF-16 without a BOM (most bytes of one parity are zero, as in mostly
/// ASCII text) or, failing that, a legacy encoding guessed from byte
/// statistics. Only encodings that decode `bytes` exactly are returned.
/// A guess, to be shown to the user, except for UTF-16 (see
/// [`is_confident`]).
pub fn guess(bytes: &[u8]) -> Option<TextEncoding> {
    let sample = &bytes[..bytes.len().min(64 * 1024)];
    let pairs = sample.len() / 2;
    if pairs >= 4 && bytes.len().is_multiple_of(2) {
        let zero_even = sample.iter().step_by(2).filter(|b| **b == 0).count();
        let zero_odd = sample
            .iter()
            .skip(1)
            .step_by(2)
            .filter(|b| **b == 0)
            .count();
        let candidate = if zero_odd * 10 >= pairs * 4 && zero_even * 20 < pairs {
            Some(TextEncoding::Utf16Le)
        } else if zero_even * 10 >= pairs * 4 && zero_odd * 20 < pairs {
            Some(TextEncoding::Utf16Be)
        } else {
            None
        };
        if let Some(enc) = candidate.filter(|e| decode_as(bytes, *e).is_ok()) {
            return Some(enc);
        }
    }
    let mut detector = chardetng::EncodingDetector::new(chardetng::Iso2022JpDetection::Deny);
    detector.feed(sample, sample.len() == bytes.len());
    let enc = detector.guess(None, chardetng::Utf8Detection::Deny);
    let enc = TextEncoding::Legacy(enc);
    (enc != TextEncoding::Legacy(encoding_rs::UTF_8) && decode_as(bytes, enc).is_ok())
        .then_some(enc)
}

/// Whether a [`guess`] is safe to apply without asking: UTF-16 decodes
/// losslessly and the zero-byte pattern is unambiguous.
pub fn is_confident(enc: TextEncoding) -> bool {
    matches!(enc, TextEncoding::Utf16Le | TextEncoding::Utf16Be)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(bytes: &[u8], expected: TextEncoding) {
        let (text, enc) = decode(bytes).expect("decodes");
        assert_eq!(enc, expected);
        assert_eq!(encode(&text, enc).unwrap(), bytes, "bytes must round-trip");
    }

    #[test]
    fn guesses_utf16_without_bom_and_legacy_text() {
        let text = "Hello, Sanain!\r\nUTF-16 without a byte order mark.\r\n";
        let le = encode(text, TextEncoding::Utf16Le).unwrap();
        let be = encode(text, TextEncoding::Utf16Be).unwrap();
        assert!(!le.starts_with(BOM_UTF16LE) && !be.starts_with(BOM_UTF16BE));
        assert_eq!(guess(&le), Some(TextEncoding::Utf16Le));
        assert_eq!(guess(&be), Some(TextEncoding::Utf16Be));
        assert_eq!(decode_as(&le, TextEncoding::Utf16Le).unwrap(), text);
        assert!(is_confident(TextEncoding::Utf16Le));
        // Finnish in Windows-1252 / ISO-8859-1 style bytes.
        let latin = encoding_rs::WINDOWS_1252
            .encode("Hyvää päivää, äidinkieli ja öljy. Ääkköset ovat tärkeitä.")
            .0
            .into_owned();
        let g = guess(&latin).expect("a legacy guess");
        assert!(matches!(g, TextEncoding::Legacy(_)), "{g:?}");
        assert!(!is_confident(g));
        assert_eq!(
            decode_as(&latin, g).unwrap(),
            "Hyvää päivää, äidinkieli ja öljy. Ääkköset ovat tärkeitä."
        );
        // Japanese Shift_JIS.
        let sjis = encoding_rs::SHIFT_JIS
            .encode("日本語のテキストです。これはテストです。")
            .0
            .into_owned();
        assert_eq!(
            guess(&sjis),
            Some(TextEncoding::Legacy(encoding_rs::SHIFT_JIS))
        );
    }

    #[test]
    fn round_trips_supported_encodings() {
        round_trip(b"", TextEncoding::Utf8);
        round_trip("a\r\nb\nc\r é🙂".as_bytes(), TextEncoding::Utf8);
        round_trip(b"\xEF\xBB\xBFhi\r\n", TextEncoding::Utf8Bom);
        round_trip(b"\xEF\xBB\xBF", TextEncoding::Utf8Bom);
        round_trip(
            &encode("a\r\n🙂", TextEncoding::Utf16LeBom).unwrap(),
            TextEncoding::Utf16LeBom,
        );
        round_trip(
            &encode("a\n漢", TextEncoding::Utf16BeBom).unwrap(),
            TextEncoding::Utf16BeBom,
        );
    }

    #[test]
    fn legacy_round_trip_and_refusals() {
        use encoding_rs::{SHIFT_JIS, WINDOWS_1252};
        let w = TextEncoding::Legacy(WINDOWS_1252);
        // "café €" in Windows-1252, plus every byte value.
        let mut bytes = b"caf\xE9 \x80\r\n".to_vec();
        bytes.extend(0u8..=255);
        let text = decode_as(&bytes, w).unwrap();
        assert!(text.starts_with("café €\r\n"));
        assert_eq!(encode(&text, w).unwrap(), bytes);
        let err = encode("ok 漢", w).unwrap_err();
        assert_eq!((err.character, err.index), ('漢', 3));
        let sj = TextEncoding::Legacy(SHIFT_JIS);
        let jp = encode("日本語テキスト", sj).unwrap();
        assert_eq!(decode_as(&jp, sj).unwrap(), "日本語テキスト");
        assert!(matches!(
            decode_as(b"\x82", sj),
            Err(DecodeError::Malformed { .. })
        ));
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
    fn utf32_is_rejected_not_misread() {
        assert_eq!(decode(b"\xFF\xFE\0\0a\0\0\0"), Err(DecodeError::Utf32));
        assert_eq!(decode(b"\0\0\xFE\xFF\0\0\0a"), Err(DecodeError::Utf32));
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
