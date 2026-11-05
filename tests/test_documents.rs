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
        "flighttape" => "the directory rollup",
        "retroforge" => "pull request",
        "bluepencil" => "diff for a human",
        "unheard" => "obedience",
        "onrecord" => "receipts",
        "wallofwrong" => "grades the sentence",
        "secondbill" => "itemized monthly statement",
        "secondtake" => "keep, tune, revert, or an honest abstain",
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

/// The tools section's bullet lines, in document order,
/// one entry per line that opens a tool bullet.
fn tool_bullets(section: &str) -> Vec<(&str, &str)> {
    let mut bullets = Vec::new();
    for line in splitlines(section) {
        for tool in TOOLS {
            if line.starts_with(&format!("- **{tool}**")) {
                bullets.push((tool, line));
            }
        }
    }
    bullets
}

/// The state frames a bullet carries: every match of
/// the frame a state is stated in, a number word or a
/// digit run followed by "of one hundred rounds
/// closed", the match running from the number's first
/// character to the frame's end. Matches never
/// overlap; the scan resumes at a match's end.
fn state_frames(bullet: &str) -> Vec<&str> {
    const TAIL: &str = " of one hundred rounds closed";
    let bytes = bullet.as_bytes();
    let mut frames = Vec::new();
    let mut cursor = 0;
    while let Some(rel) = bullet[cursor..].find(TAIL) {
        let tail_start = cursor + rel;
        let mut start = tail_start;
        while start > 0 && bytes[start - 1].is_ascii_alphanumeric() {
            start -= 1;
        }
        if start < tail_start {
            frames.push(&bullet[start..tail_start + TAIL.len()]);
        }
        cursor = tail_start + TAIL.len();
    }
    frames
}

/// The number a state token names: a digit run reads as
/// itself, a number word over the words a count in this
/// record can be spelled with (IP-24's set).
fn state_value(token: &str) -> usize {
    if !token.is_empty() && token.bytes().all(|byte| byte.is_ascii_digit()) {
        return token.parse().expect("a digit state token");
    }
    match token.to_lowercase().as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "thirteen" => 13,
        "fourteen" => 14,
        "fifteen" => 15,
        "sixteen" => 16,
        "seventeen" => 17,
        "eighteen" => 18,
        "nineteen" => 19,
        "twenty" => 20,
        "thirty" => 30,
        "forty" => 40,
        "fifty" => 50,
        "sixty" => 60,
        "seventy" => 70,
        "eighty" => 80,
        "ninety" => 90,
        "hundred" => 100,
        _ => panic!("no state value for {token:?}"),
    }
}

/// One tool's bullet line. The Python collects the bullets
/// in a dict, so a duplicated tool bullet is judged by its
/// last line; the lookup takes the last match for the same
/// reason.
fn bullet_of<'a>(bullets: &[(&'a str, &'a str)], tool: &str) -> &'a str {
    bullets
        .iter()
        .rev()
        .find(|(name, _)| *name == tool)
        .map(|(_, line)| *line)
        .expect("a tool bullet")
}

#[test]
fn test_the_tools_lines_carry_each_repos_own_job() {
    let readme = readme();
    let section = tools_section(&readme);
    let bullets = tool_bullets(section);
    let named: BTreeSet<&str> = bullets.iter().map(|(tool, _)| *tool).collect();
    let all: BTreeSet<&str> = TOOLS.iter().copied().collect();
    assert_eq!(named, all);
    for tool in TOOLS {
        assert!(
            bullet_of(&bullets, tool).contains(tool_phrase(tool)),
            "{tool}"
        );
    }
    // Each state sits on its own tool's line, so two states
    // cannot be swapped without a test failing.
    assert!(bullet_of(&bullets, "flighttape").contains("Thirteen of one hundred rounds closed"));
    for tool in [
        "retroforge",
        "bluepencil",
        "unheard",
        "onrecord",
        "wallofwrong",
        "secondtake",
    ] {
        assert!(
            bullet_of(&bullets, tool).contains("Five of one hundred rounds closed"),
            "{tool}"
        );
    }
    assert!(bullet_of(&bullets, "secondbill").contains("Four of one hundred rounds closed"));
    assert!(bullet_of(&bullets, "verdictloop").contains("Four of one hundred rounds closed"));
    assert!(bullet_of(&bullets, "impatient-programming").contains("of one hundred rounds closed"));
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

// The states below are pins on what the sibling repos report.
// They are refreshed by hand in batched refresh rounds like
// Rounds 4 and 5, not each time a sibling closes a round, so the
// record can sit stale between refreshes; the round reviews
// check the pins against those repos. The aggregate count
// runs over the nine sibling bullets only, so this repo's
// own state never enters it and a closure here cannot trip
// the pin that watches the siblings.
#[test]
fn test_the_tool_states_are_pinned_to_the_repos_reports() {
    let readme = readme();
    let section = tools_section(&readme);
    let bullets = tool_bullets(section);
    assert!(bullet_of(&bullets, "flighttape").contains("Thirteen of one hundred rounds closed"));
    let count_state = |state: &str| -> usize {
        TOOLS
            .iter()
            .filter(|tool| **tool != "impatient-programming")
            .map(|tool| bullet_of(&bullets, tool).matches(state).count())
            .sum()
    };
    assert_eq!(count_state("Five of one hundred rounds closed"), 6);
    assert_eq!(count_state("Four of one hundred rounds closed"), 2);
    // IP-23. A bullet states its state exactly once: the
    // frame's matches on each sibling bullet number
    // exactly one, and that match is the pinned state, so
    // a second state sentence cannot stand beside the
    // pinned one unseen.
    let pins: [(&str, &str); 9] = [
        ("flighttape", "Thirteen of one hundred rounds closed"),
        ("retroforge", "Five of one hundred rounds closed"),
        ("bluepencil", "Five of one hundred rounds closed"),
        ("unheard", "Five of one hundred rounds closed"),
        ("onrecord", "Five of one hundred rounds closed"),
        ("wallofwrong", "Five of one hundred rounds closed"),
        ("secondbill", "Four of one hundred rounds closed"),
        ("secondtake", "Five of one hundred rounds closed"),
        ("verdictloop", "Four of one hundred rounds closed"),
    ];
    for (tool, state) in pins {
        assert_eq!(state_frames(bullet_of(&bullets, tool)), [state], "{tool}");
    }
}

/// IP-25. The README states this repo's own state twice,
/// in the own bullet's state word and in the Programme
/// status line's digits, and the two must agree. The
/// check compares the numbers, not a literal, so it
/// holds at whatever value a closure sets in both
/// places.
#[test]
fn test_the_readmes_own_state_statements_agree() {
    let readme = readme();
    let section = tools_section(&readme);
    let bullets = tool_bullets(section);
    let own = bullet_of(&bullets, "impatient-programming");
    let frames = state_frames(own);
    assert_eq!(frames.len(), 1, "the own bullet states its state once");
    let word = frames[0].split(' ').next().expect("a state word");
    let status_tail = readme
        .split_once("Closed perfection rounds, ")
        .map(|(_, tail)| tail)
        .expect("the README carries a programme status line");
    let digits: String = status_tail
        .chars()
        .take_while(|char| char.is_ascii_digit())
        .collect();
    let status: usize = digits.parse().expect("a digit status count");
    assert_eq!(state_value(word), status);
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
