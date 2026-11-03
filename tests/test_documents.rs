//! IP-1 to IP-6, the document obligations. A one for one
//! port of tests/test_documents.py at e031741, plus the
//! IP-14 fold set test the port owes.

use std::collections::BTreeSet;
use std::path::PathBuf;

use impatient_programming::{folded, read_document, root, splitlines, suffix_of, walk};
use unicode_normalization::UnicodeNormalization;

const PRINCIPLES: [&str; 10] = [
    "Speed is the point",
    "Local gates before CI",
    "Three reviews or it is not done",
    "Contract before code",
    "Count honestly",
    "Measure everything",
    "Verify the real thing",
    "Work survives the worker",
    "The perfection bar",
    "Agents work, the human judges",
];

const TOOLS: [&str; 10] = [
    "flighttape",
    "retroforge",
    "bluepencil",
    "unheard",
    "onrecord",
    "wallofwrong",
    "secondbill",
    "secondtake",
    "verdictloop",
    "impatient-programming",
];

const FORBIDDEN: [char; 9] = [
    ':', ';', '\u{2014}', '\u{2013}', '\u{2015}', '\u{ff1a}', '\u{ff1b}', '\u{fe58}', '\u{fe54}',
];

/// One distinctive phrase per tool, taken from that tool's
/// own README, so two tools' lines cannot be swapped or
/// blurred without a test failing.
fn tool_phrase(tool: &str) -> &'static str {
    match tool {
        "flighttape" => "rolled up",
        "retroforge" => "pull request",
        "bluepencil" => "diff for a human",
        "unheard" => "obedience",
        "onrecord" => "receipts",
        "wallofwrong" => "grades the sentence",
        "secondbill" => "itemized monthly statement",
        "secondtake" => "keep, tune, or revert",
        "verdictloop" => "experiment registry",
        "impatient-programming" => "statement of the practice",
        _ => unreachable!("unknown tool {tool}"),
    }
}

fn readme() -> String {
    read_document("README.md")
}

fn manifesto() -> String {
    read_document("MANIFESTO.md")
}

/// The text before the first occurrence of the needle, or
/// the whole text when the needle never occurs. Python's
/// split with a count of one, taking the first part.
fn before_first<'a>(text: &'a str, needle: &str) -> &'a str {
    match text.split_once(needle) {
        Some((head, _)) => head,
        None => text,
    }
}

/// The tools section of the README: after the tools
/// heading, before the next section heading.
fn tools_section(readme: &str) -> &str {
    let after = readme
        .split_once("## The tools")
        .map(|(_, tail)| tail)
        .expect("the README carries a tools heading");
    before_first(after, "## ")
}

#[test]
fn test_the_copy_bar_holds_in_both_documents() {
    for (name, text) in [("README.md", readme()), ("MANIFESTO.md", manifesto())] {
        // Compatibility forms fold to the barred characters
        // under NFKC, so the check runs on the folded text as
        // well as the raw text.
        let folded_text = folded(&text);
        for char in FORBIDDEN {
            assert!(!text.contains(char), "{name} contains {char:?}");
            assert!(!folded_text.contains(char), "{name} folds to {char:?}");
        }
    }
}

#[test]
fn test_the_readme_states_whose_take_this_is() {
    let readme = readme();
    assert!(readme.contains("David Engelmann's own take"));
    assert!(readme.contains("impatientprogramming.org"));
    assert!(readme.contains("This repo is not that method"));
    // The only mention of that method is the disambiguation
    // itself. A sentence claiming the mantle breaks this.
    assert_eq!(readme.matches("that method").count(), 1);
}

#[test]
fn test_every_principle_is_named_in_the_readme() {
    let readme = readme();
    assert!(readme.contains("## Principles"));
    for principle in PRINCIPLES {
        let bold = format!("**{principle}.**");
        assert!(readme.contains(&bold), "{principle}");
        let bullet = splitlines(&readme)
            .into_iter()
            .find(|line| line.contains(&bold))
            .expect("a principle bullet line");
        // A name with no statement after it is not a principle.
        assert!(bullet.chars().count() > 120, "{principle}");
    }
}

#[test]
fn test_every_principle_has_a_manifesto_section() {
    let manifesto = manifesto();
    let sections: Vec<&str> = manifesto.split("\n## ").collect();
    for principle in PRINCIPLES {
        assert!(
            manifesto.contains(&format!("## {principle}")),
            "{principle}"
        );
        let body = sections
            .iter()
            .find(|part| part.starts_with(principle))
            .expect("a manifesto section");
        // A heading over a gutted paragraph is not an argument.
        assert!(body.chars().count() > 400, "{principle}");
    }
}

#[test]
fn test_the_tools_section_names_all_ten_repos() {
    let readme = readme();
    let section = tools_section(&readme);
    for tool in TOOLS {
        assert!(section.contains(&format!("**{tool}**")), "{tool}");
    }
}

#[test]
fn test_the_tools_lines_carry_each_repos_own_job() {
    let readme = readme();
    let section = tools_section(&readme);
    let mut bullets: Vec<(&str, &str)> = Vec::new();
    for line in splitlines(section) {
        for tool in TOOLS {
            if line.starts_with(&format!("- **{tool}**")) {
                bullets.push((tool, line));
            }
        }
    }
    let named: BTreeSet<&str> = bullets.iter().map(|(tool, _)| *tool).collect();
    let all: BTreeSet<&str> = TOOLS.iter().copied().collect();
    assert_eq!(named, all);
    // The Python collects the bullets in a dict, so a
    // duplicated tool bullet is judged by its last line;
    // the lookup takes the last match for the same reason.
    let bullet_of = |tool: &str| -> &str {
        bullets
            .iter()
            .rev()
            .find(|(name, _)| *name == tool)
            .map(|(_, line)| *line)
            .expect("a tool bullet")
    };
    for tool in TOOLS {
        assert!(bullet_of(tool).contains(tool_phrase(tool)), "{tool}");
    }
    // Each state sits on its own tool's line, so two states
    // cannot be swapped without a test failing.
    assert!(bullet_of("flighttape").contains("Nine of one hundred rounds closed"));
    assert!(bullet_of("retroforge").contains("One of one hundred rounds closed"));
    for tool in TOOLS {
        if ["flighttape", "retroforge", "impatient-programming"].contains(&tool) {
            continue;
        }
        assert!(
            bullet_of(tool).contains("At setup, rounds not yet started"),
            "{tool}"
        );
    }
    assert!(bullet_of("impatient-programming").contains("of one hundred rounds closed"));
}

const CODE_SUFFIXES: [&str; 16] = [
    ".py", ".js", ".mjs", ".jsx", ".ts", ".tsx", ".go", ".rs", ".rb", ".sh", ".java", ".c", ".cc",
    ".cpp", ".swift", ".php",
];

const SKIPPED_PARTS: [&str; 10] = [
    ".git",
    "scripts",
    "tests",
    "src",
    "target",
    "venv",
    ".venv",
    "env",
    "node_modules",
    "__pycache__",
];

#[test]
fn test_no_product_code_lives_in_the_repo() {
    let root = root();
    let mut files = Vec::new();
    walk(&root, &mut files);
    let code: Vec<&PathBuf> = files
        .iter()
        .filter(|path| {
            if !CODE_SUFFIXES.contains(&suffix_of(path).as_str()) {
                return false;
            }
            let relative = path.strip_prefix(&root).unwrap_or(path);
            !relative
                .components()
                .any(|part| SKIPPED_PARTS.contains(&part.as_os_str().to_str().unwrap_or("")))
        })
        .collect();
    assert!(code.is_empty(), "product code in the repo: {code:?}");
}

#[test]
fn test_the_wider_portfolio_sentence_is_present() {
    assert!(readme().contains("The same method runs across the rest of the portfolio"));
}

// The states below are pins on what the sibling repos report,
// refreshed by hand whenever a sibling closes a round and
// checked against those repos by the round reviews.
#[test]
fn test_the_tool_states_are_pinned_to_the_repos_reports() {
    let readme = readme();
    assert!(readme.contains("Nine of one hundred rounds closed"));
    assert_eq!(
        readme.matches("At setup, rounds not yet started").count(),
        7
    );
}

/// IP-14. The interpreter's fold set: every codepoint whose
/// NFKC normalization holds a barred character, computed by
/// CPython 3.12 (unicodedata 15.0.0) over the whole space.
/// The crate must produce the identical set.
#[test]
fn test_the_nfkc_fold_set_matches_the_interpreter() {
    let mut found: BTreeSet<u32> = BTreeSet::new();
    for cp in 0..=0x10FFFFu32 {
        if (0xD800..=0xDFFF).contains(&cp) {
            continue;
        }
        let Some(char) = char::from_u32(cp) else {
            continue;
        };
        let folded_text: String = char.nfkc().collect();
        if FORBIDDEN.iter().any(|barred| folded_text.contains(*barred)) {
            found.insert(cp);
        }
    }
    let expected: BTreeSet<u32> = [
        0x003A, 0x003B, 0x037E, 0x2013, 0x2014, 0x2015, 0x2A74, 0xFE13, 0xFE14, 0xFE31, 0xFE32,
        0xFE54, 0xFE55, 0xFE58, 0xFF1A, 0xFF1B,
    ]
    .into_iter()
    .collect();
    assert_eq!(found, expected);
}
