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

/// A word character for the interpreter's \b and \w: an
/// alphanumeric or an underscore, with alphanumeric read
/// as CPython reads it. A full-codepoint sweep of
/// CPython 3.12's \w against this toolchain's
/// char::is_alphanumeric plus underscore (Round 3) found
/// the Python set a strict subset of the Rust one, with
/// 6,499 codepoints in 275 ranges where Rust reads a
/// combining mark or an enclosed letter as alphanumeric
/// and CPython does not. The semantics here are Rust's
/// test minus the swept ranges below.
pub fn is_word_char(char: char) -> bool {
    (char.is_alphanumeric() || char == '_') && !in_not_word_ranges(char as u32)
}

fn in_not_word_ranges(scalar: u32) -> bool {
    NOT_WORD_RANGES
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

/// The swept ranges where char::is_alphanumeric holds
/// and CPython 3.12's \w does not.
const NOT_WORD_RANGES: &[(u32, u32)] = &[
    (0x345, 0x345),
    (0x363, 0x36F),
    (0x5B0, 0x5BD),
    (0x5BF, 0x5BF),
    (0x5C1, 0x5C2),
    (0x5C4, 0x5C5),
    (0x5C7, 0x5C7),
    (0x610, 0x61A),
    (0x64B, 0x657),
    (0x659, 0x65F),
    (0x670, 0x670),
    (0x6D6, 0x6DC),
    (0x6E1, 0x6E4),
    (0x6E7, 0x6E8),
    (0x6ED, 0x6ED),
    (0x711, 0x711),
    (0x730, 0x73F),
    (0x7A6, 0x7B0),
    (0x816, 0x817),
    (0x81B, 0x823),
    (0x825, 0x827),
    (0x829, 0x82C),
    (0x897, 0x897),
    (0x8D4, 0x8DF),
    (0x8E3, 0x8E9),
    (0x8F0, 0x903),
    (0x93A, 0x93B),
    (0x93E, 0x94C),
    (0x94E, 0x94F),
    (0x955, 0x957),
    (0x962, 0x963),
    (0x981, 0x983),
    (0x9BE, 0x9C4),
    (0x9C7, 0x9C8),
    (0x9CB, 0x9CC),
    (0x9D7, 0x9D7),
    (0x9E2, 0x9E3),
    (0xA01, 0xA03),
    (0xA3E, 0xA42),
    (0xA47, 0xA48),
    (0xA4B, 0xA4C),
    (0xA51, 0xA51),
    (0xA70, 0xA71),
    (0xA75, 0xA75),
    (0xA81, 0xA83),
    (0xABE, 0xAC5),
    (0xAC7, 0xAC9),
    (0xACB, 0xACC),
    (0xAE2, 0xAE3),
    (0xAFA, 0xAFC),
    (0xB01, 0xB03),
    (0xB3E, 0xB44),
    (0xB47, 0xB48),
    (0xB4B, 0xB4C),
    (0xB56, 0xB57),
    (0xB62, 0xB63),
    (0xB82, 0xB82),
    (0xBBE, 0xBC2),
    (0xBC6, 0xBC8),
    (0xBCA, 0xBCC),
    (0xBD7, 0xBD7),
    (0xC00, 0xC04),
    (0xC3E, 0xC44),
    (0xC46, 0xC48),
    (0xC4A, 0xC4C),
    (0xC55, 0xC56),
    (0xC62, 0xC63),
    (0xC81, 0xC83),
    (0xCBE, 0xCC4),
    (0xCC6, 0xCC8),
    (0xCCA, 0xCCC),
    (0xCD5, 0xCD6),
    (0xCE2, 0xCE3),
    (0xCF3, 0xCF3),
    (0xD00, 0xD03),
    (0xD3E, 0xD44),
    (0xD46, 0xD48),
    (0xD4A, 0xD4C),
    (0xD57, 0xD57),
    (0xD62, 0xD63),
    (0xD81, 0xD83),
    (0xDCF, 0xDD4),
    (0xDD6, 0xDD6),
    (0xDD8, 0xDDF),
    (0xDF2, 0xDF3),
    (0xE31, 0xE31),
    (0xE34, 0xE3A),
    (0xE4D, 0xE4D),
    (0xEB1, 0xEB1),
    (0xEB4, 0xEB9),
    (0xEBB, 0xEBC),
    (0xECD, 0xECD),
    (0xF71, 0xF83),
    (0xF8D, 0xF97),
    (0xF99, 0xFBC),
    (0x102B, 0x1036),
    (0x1038, 0x1038),
    (0x103B, 0x103E),
    (0x1056, 0x1059),
    (0x105E, 0x1060),
    (0x1062, 0x1064),
    (0x1067, 0x106D),
    (0x1071, 0x1074),
    (0x1082, 0x108D),
    (0x108F, 0x108F),
    (0x109A, 0x109D),
    (0x1712, 0x1713),
    (0x1732, 0x1733),
    (0x1752, 0x1753),
    (0x1772, 0x1773),
    (0x17B6, 0x17C8),
    (0x1885, 0x1886),
    (0x18A9, 0x18A9),
    (0x1920, 0x192B),
    (0x1930, 0x1938),
    (0x1A17, 0x1A1B),
    (0x1A55, 0x1A5E),
    (0x1A61, 0x1A74),
    (0x1ABF, 0x1AC0),
    (0x1ACC, 0x1ACE),
    (0x1B00, 0x1B04),
    (0x1B35, 0x1B43),
    (0x1B80, 0x1B82),
    (0x1BA1, 0x1BA9),
    (0x1BAC, 0x1BAD),
    (0x1BE7, 0x1BF1),
    (0x1C24, 0x1C36),
    (0x1C89, 0x1C8A),
    (0x1DD3, 0x1DF4),
    (0x24B6, 0x24E9),
    (0x2DE0, 0x2DFF),
    (0xA674, 0xA67B),
    (0xA69E, 0xA69F),
    (0xA7CB, 0xA7CD),
    (0xA7DA, 0xA7DC),
    (0xA802, 0xA802),
    (0xA80B, 0xA80B),
    (0xA823, 0xA827),
    (0xA880, 0xA881),
    (0xA8B4, 0xA8C3),
    (0xA8C5, 0xA8C5),
    (0xA8FF, 0xA8FF),
    (0xA926, 0xA92A),
    (0xA947, 0xA952),
    (0xA980, 0xA983),
    (0xA9B4, 0xA9BF),
    (0xA9E5, 0xA9E5),
    (0xAA29, 0xAA36),
    (0xAA43, 0xAA43),
    (0xAA4C, 0xAA4D),
    (0xAA7B, 0xAA7D),
    (0xAAB0, 0xAAB0),
    (0xAAB2, 0xAAB4),
    (0xAAB7, 0xAAB8),
    (0xAABE, 0xAABE),
    (0xAAEB, 0xAAEF),
    (0xAAF5, 0xAAF5),
    (0xABE3, 0xABEA),
    (0xFB1E, 0xFB1E),
    (0x10376, 0x1037A),
    (0x105C0, 0x105F3),
    (0x10A01, 0x10A03),
    (0x10A05, 0x10A06),
    (0x10A0C, 0x10A0F),
    (0x10D24, 0x10D27),
    (0x10D40, 0x10D65),
    (0x10D69, 0x10D69),
    (0x10D6F, 0x10D85),
    (0x10EAB, 0x10EAC),
    (0x10EC2, 0x10EC4),
    (0x10EFC, 0x10EFC),
    (0x11000, 0x11002),
    (0x11038, 0x11045),
    (0x11073, 0x11074),
    (0x11080, 0x11082),
    (0x110B0, 0x110B8),
    (0x110C2, 0x110C2),
    (0x11100, 0x11102),
    (0x11127, 0x11132),
    (0x11145, 0x11146),
    (0x11180, 0x11182),
    (0x111B3, 0x111BF),
    (0x111CE, 0x111CF),
    (0x1122C, 0x11234),
    (0x11237, 0x11237),
    (0x1123E, 0x1123E),
    (0x11241, 0x11241),
    (0x112DF, 0x112E8),
    (0x11300, 0x11303),
    (0x1133E, 0x11344),
    (0x11347, 0x11348),
    (0x1134B, 0x1134C),
    (0x11357, 0x11357),
    (0x11362, 0x11363),
    (0x11380, 0x11389),
    (0x1138B, 0x1138B),
    (0x1138E, 0x1138E),
    (0x11390, 0x113B5),
    (0x113B7, 0x113C0),
    (0x113C2, 0x113C2),
    (0x113C5, 0x113C5),
    (0x113C7, 0x113CA),
    (0x113CC, 0x113CD),
    (0x113D1, 0x113D1),
    (0x113D3, 0x113D3),
    (0x11435, 0x11441),
    (0x11443, 0x11445),
    (0x114B0, 0x114C1),
    (0x115AF, 0x115B5),
    (0x115B8, 0x115BE),
    (0x115DC, 0x115DD),
    (0x11630, 0x1163E),
    (0x11640, 0x11640),
    (0x116AB, 0x116B5),
    (0x116D0, 0x116E3),
    (0x1171D, 0x1172A),
    (0x1182C, 0x11838),
    (0x11930, 0x11935),
    (0x11937, 0x11938),
    (0x1193B, 0x1193C),
    (0x11940, 0x11940),
    (0x11942, 0x11942),
    (0x119D1, 0x119D7),
    (0x119DA, 0x119DF),
    (0x119E4, 0x119E4),
    (0x11A01, 0x11A0A),
    (0x11A35, 0x11A39),
    (0x11A3B, 0x11A3E),
    (0x11A51, 0x11A5B),
    (0x11A8A, 0x11A97),
    (0x11BC0, 0x11BE0),
    (0x11BF0, 0x11BF9),
    (0x11C2F, 0x11C36),
    (0x11C38, 0x11C3E),
    (0x11C92, 0x11CA7),
    (0x11CA9, 0x11CB6),
    (0x11D31, 0x11D36),
    (0x11D3A, 0x11D3A),
    (0x11D3C, 0x11D3D),
    (0x11D3F, 0x11D41),
    (0x11D43, 0x11D43),
    (0x11D47, 0x11D47),
    (0x11D8A, 0x11D8E),
    (0x11D90, 0x11D91),
    (0x11D93, 0x11D96),
    (0x11EF3, 0x11EF6),
    (0x11F00, 0x11F01),
    (0x11F03, 0x11F03),
    (0x11F34, 0x11F3A),
    (0x11F3E, 0x11F40),
    (0x13460, 0x143FA),
    (0x16100, 0x1612E),
    (0x16130, 0x16139),
    (0x16D40, 0x16D6C),
    (0x16D70, 0x16D79),
    (0x16F4F, 0x16F4F),
    (0x16F51, 0x16F87),
    (0x16F8F, 0x16F92),
    (0x16FF0, 0x16FF1),
    (0x18CFF, 0x18CFF),
    (0x1BC9E, 0x1BC9E),
    (0x1CCF0, 0x1CCF9),
    (0x1E000, 0x1E006),
    (0x1E008, 0x1E018),
    (0x1E01B, 0x1E021),
    (0x1E023, 0x1E024),
    (0x1E026, 0x1E02A),
    (0x1E08F, 0x1E08F),
    (0x1E5D0, 0x1E5ED),
    (0x1E5F0, 0x1E5FA),
    (0x1E947, 0x1E947),
    (0x1F130, 0x1F149),
    (0x1F150, 0x1F169),
    (0x1F170, 0x1F189),
    (0x2EBF0, 0x2EE5D),
];

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
