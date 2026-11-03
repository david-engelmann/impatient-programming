//! The checker the guard suite shares (IP-12). The repo's
//! product is prose; this library holds only what the tests
//! need to read the documents and run the folded copy bar
//! scan over them. There is no binary and no other public
//! API.

use std::path::PathBuf;

use unicode_normalization::UnicodeNormalization;

/// The repo root, the directory holding the documents.
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Read one of the repo's documents as strict UTF-8. The
/// Python suite reads the documents at import and a failure
/// is fatal there, so a failure here panics, naming the
/// file, rather than being recovered from.
pub fn read_document(name: &str) -> String {
    let path = root().join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "cannot read the document {name} at {}: {err}",
            path.display()
        )
    });
    String::from_utf8(bytes)
        .unwrap_or_else(|err| panic!("the document {name} is not valid UTF-8: {err}"))
}

/// The text folded under Unicode NFKC, the form the copy
/// bar also runs against, because compatibility forms of
/// the barred characters fold to the barred characters
/// themselves (IP-2 as amended, IP-14).
pub fn folded(text: &str) -> String {
    text.nfkc().collect()
}
