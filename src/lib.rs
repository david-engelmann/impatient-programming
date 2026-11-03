//! The checker the guard suite shares (IP-12). The repo's
//! product is prose; this library holds only the pieces
//! the tests share: reading the documents as the
//! interpreter reads them, the NFKC fold the copy bar
//! scan runs against, the interpreter's line splitting
//! and word characters, and the tree walk the
//! no-product-code test runs. There is no binary and no
//! other public API.

use std::path::{Path, PathBuf};

use unicode_normalization::UnicodeNormalization;

mod word_ranges;

use word_ranges::WORD_RANGES;

/// The repo root, the directory holding the documents.
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Read one of the repo's documents as strict UTF-8, with
/// the interpreter's universal-newline translation
/// applied: Path.read_text turns CRLF and lone CR into
/// LF before any test sees the text, so this read does
/// the same. The Python suite reads the documents at
/// import and a failure is fatal there, so a failure
/// here panics, naming the file, rather than being
/// recovered from.
pub fn read_document(name: &str) -> String {
    let path = root().join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "cannot read the document {name} at {}: {err}",
            path.display()
        )
    });
    let text = String::from_utf8(bytes)
        .unwrap_or_else(|err| panic!("the document {name} is not valid UTF-8: {err}"));
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// The text folded under Unicode NFKC, the form the copy
/// bar also runs against, because compatibility forms of
/// the barred characters fold to the barred characters
/// themselves (IP-2 as amended, IP-14).
pub fn folded(text: &str) -> String {
    text.nfkc().collect()
}

/// The interpreter's line splitting, str.splitlines: it
/// breaks at LF, CR, CRLF taken as one break, VT, FF,
/// U+001C to U+001E, U+0085, U+2028, and U+2029, where
/// str::lines breaks only at LF and CRLF. A trailing
/// break yields no final empty piece, as splitlines
/// yields none.
pub fn splitlines(text: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((index, char)) = chars.next() {
        let width = match char {
            '\r' => {
                if matches!(chars.peek(), Some((_, '\n'))) {
                    chars.next();
                    2
                } else {
                    1
                }
            }
            '\n' | '\u{b}' | '\u{c}' | '\u{1c}' | '\u{1d}' | '\u{1e}' | '\u{85}' | '\u{2028}'
            | '\u{2029}' => char.len_utf8(),
            _ => continue,
        };
        lines.push(&text[start..index]);
        start = index + width;
    }
    if start < text.len() {
        lines.push(&text[start..]);
    }
    lines
}

/// A word character for the interpreter's \b and \w:
/// membership in the set CPython 3.12's \w matches,
/// pinned positively as the range table in word_ranges
/// (137,936 codepoints in 748 ranges). The standard
/// library's char classification is not consulted: it
/// drifts with the toolchain's Unicode version, so a
/// definition built on it, however corrected by an
/// exclusion table, changes meaning when the toolchain
/// does (Round 3: CI's newer toolchain classified
/// 4,672 codepoints as alphanumeric that this
/// toolchain's does not, and the exclusion shape of
/// this check let every one of them through).
pub fn is_word_char(char: char) -> bool {
    in_word_ranges(char as u32)
}

fn in_word_ranges(scalar: u32) -> bool {
    WORD_RANGES
        .binary_search_by(|&(start, end)| {
            if scalar < start {
                std::cmp::Ordering::Greater
            } else if scalar > end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

/// The suffix of a file name in pathlib's sense: the text
/// from the last dot, where a dot that opens the name names
/// no suffix.
pub fn suffix_of(path: &Path) -> String {
    let name = match path.file_name().and_then(|name| name.to_str()) {
        Some(name) => name,
        None => return String::new(),
    };
    match name.rfind('.') {
        Some(index) if index > 0 => name[index..].to_string(),
        _ => String::new(),
    }
}

/// Collect every file under a directory, recursively.
/// The interpreter's rglob does not descend into a
/// symlinked directory and neither does this walk, as
/// entry file types do not follow symlinks; a symlink to
/// a file is collected, as rglob collects it.
pub fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(kind) => kind,
            Err(_) => continue,
        };
        if file_type.is_dir() {
            walk(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}
