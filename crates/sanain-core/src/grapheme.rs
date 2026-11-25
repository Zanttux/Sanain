//! Grapheme cluster boundaries over a rope, without materialising lines.
//! Adapted from the approach in ropey's documentation examples.

use ropey::{Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

/// The char index of the next grapheme boundary after `char_idx`, or the
/// document length.
pub fn next_boundary(rope: &Rope, char_idx: usize) -> usize {
    let slice = rope.slice(..);
    let len = slice.len_bytes();
    let byte_idx = slice.char_to_byte(char_idx.min(slice.len_chars()));
    let (mut chunk, mut chunk_byte, _, _) = slice.chunk_at_byte(byte_idx);
    let mut cursor = GraphemeCursor::new(byte_idx, len, true);
    loop {
        match cursor.next_boundary(chunk, chunk_byte) {
            Ok(None) => return slice.len_chars(),
            Ok(Some(b)) => return slice.byte_to_char(b),
            Err(GraphemeIncomplete::NextChunk) => {
                chunk_byte += chunk.len();
                chunk = slice.chunk_at_byte(chunk_byte).0;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                provide_context(&mut cursor, slice, n);
            }
            // Other variants are not produced by next_boundary.
            Err(_) => return slice.len_chars(),
        }
    }
}

/// The char index of the previous grapheme boundary before `char_idx`, or 0.
pub fn prev_boundary(rope: &Rope, char_idx: usize) -> usize {
    let slice = rope.slice(..);
    let byte_idx = slice.char_to_byte(char_idx.min(slice.len_chars()));
    let (mut chunk, mut chunk_byte, _, _) = slice.chunk_at_byte(byte_idx);
    let mut cursor = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);
    loop {
        match cursor.prev_boundary(chunk, chunk_byte) {
            Ok(None) => return 0,
            Ok(Some(b)) => return slice.byte_to_char(b),
            Err(GraphemeIncomplete::PrevChunk) => {
                let (c, start, _, _) = slice.chunk_at_byte(chunk_byte - 1);
                chunk = c;
                chunk_byte = start;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                provide_context(&mut cursor, slice, n);
            }
            Err(_) => return 0,
        }
    }
}

pub fn is_boundary(rope: &Rope, char_idx: usize) -> bool {
    let slice = rope.slice(..);
    let byte_idx = slice.char_to_byte(char_idx.min(slice.len_chars()));
    let (mut chunk, mut chunk_byte, _, _) = slice.chunk_at_byte(byte_idx);
    let mut cursor = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);
    loop {
        match cursor.is_boundary(chunk, chunk_byte) {
            Ok(b) => return b,
            Err(GraphemeIncomplete::PreContext(n)) => {
                provide_context(&mut cursor, slice, n);
                // Retry with the same chunk.
                let c = slice.chunk_at_byte(byte_idx);
                chunk = c.0;
                chunk_byte = c.1;
            }
            Err(_) => return true,
        }
    }
}

fn provide_context(cursor: &mut GraphemeCursor, slice: RopeSlice<'_>, n: usize) {
    let (ctx, ctx_byte, _, _) = slice.chunk_at_byte(n - 1);
    cursor.provide_context(ctx, ctx_byte);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boundaries(text: &str) -> Vec<usize> {
        let rope = Rope::from_str(text);
        let mut out = vec![0];
        let mut i = 0;
        while i < rope.len_chars() {
            i = next_boundary(&rope, i);
            out.push(i);
        }
        out
    }

    #[test]
    fn clusters_are_single_steps() {
        // e + combining acute, family emoji (ZWJ sequence), flag, CRLF.
        let text = "e\u{301}👨‍👩‍👧🇫🇮\r\nx";
        let b = boundaries(text);
        assert_eq!(b, vec![0, 2, 7, 9, 11, 12]);
        let rope = Rope::from_str(text);
        let mut back = vec![rope.len_chars()];
        let mut i = rope.len_chars();
        while i > 0 {
            i = prev_boundary(&rope, i);
            back.push(i);
        }
        back.reverse();
        assert_eq!(back, b);
        assert!(!is_boundary(&rope, 1));
        assert!(is_boundary(&rope, 2));
        assert!(!is_boundary(&rope, 10), "inside CRLF");
    }

    #[test]
    fn works_across_rope_chunks() {
        // Long text forces multiple chunks; clusters straddle chunk edges.
        let unit = "a👍🏽e\u{301}";
        let text = unit.repeat(2000);
        let b = boundaries(&text);
        assert_eq!(b.len() - 1, 2000 * 3);
    }

    #[test]
    fn clamps_out_of_range() {
        let rope = Rope::from_str("ab");
        assert_eq!(next_boundary(&rope, 99), 2);
        assert_eq!(prev_boundary(&rope, 0), 0);
    }
}
