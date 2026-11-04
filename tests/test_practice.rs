//! IP-7 to IP-10, the practice document. A one for one
//! port of tests/test_practice.py at e031741, with the
//! Python's small regular expressions replaced by hand
//! matchers of the same match semantics (IP-15).

use std::collections::BTreeSet;

use impatient_programming::{folded, is_word_char, read_document};

const FORBIDDEN: [char; 9] = [
    ':', ';', '\u{2014}', '\u{2013}', '\u{2015}', '\u{ff1a}', '\u{ff1b}', '\u{fe58}', '\u{fe54}',
];

const REAL_COMMANDS: [(&str, &[&str]); 9] = [
    (
        "flighttape",
        &[
            "convert",
            "convert-dir",
            "validate",
            "play",
            "digest",
            "verify",
            "compare",
            "rollup",
            "slice",
            "screen",
        ],
    ),
    ("retroforge", &["propose"]),
    ("bluepencil", &["propose", "diff"]),
    ("unheard", &["score"]),
    ("onrecord", &["add", "get", "subject", "due", "check"]),
    (
        "wallofwrong",
        &["confess", "grade", "ledger", "wall", "tally"],
    ),
    ("secondbill", &["statement"]),
    ("secondtake", &["compare"]),
    (
        "verdictloop",
        &["open", "result", "close", "list", "show", "check"],
    ),
];

fn practice() -> String {
    read_document("PRACTICE.md")
}

fn readme() -> String {
    read_document("README.md")
}

/// The text before the first occurrence of the needle, or
/// the whole text when the needle never occurs.
fn before_first<'a>(text: &'a str, needle: &str) -> &'a str {
    match text.split_once(needle) {
        Some((head, _)) => head,
        None => text,
    }
}

/// Whitespace in the interpreter's sense: the Unicode
/// White_Space property, plus the four separator controls
/// U+001C to U+001F that CPython also counts.
fn py_is_space(char: char) -> bool {
    char.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&char)
}

/// The words of a text, split on runs of whitespace, as
/// str.split() with no arguments splits.
fn split_words(text: &str) -> Vec<&str> {
    text.split(py_is_space)
        .filter(|word| !word.is_empty())
        .collect()
}

/// Whitespace collapsed to single spaces, as
/// " ".join(text.split()) collapses.
fn collapsed(text: &str) -> String {
    split_words(text).join(" ")
}

/// Strip whitespace in the interpreter's sense from both
/// ends, as str.strip() strips.
fn py_strip(text: &str) -> &str {
    text.trim_matches(py_is_space)
}

/// The Principles section of the README.
fn principles_section(readme: &str) -> &str {
    let after = readme
        .split_once("## Principles")
        .map(|(_, tail)| tail)
        .expect("the README carries a principles heading");
    before_first(after, "## ")
}

/// The next bold name after the cursor, in the shape the
/// regex - \*\*([^*]+)\.\*\* matches: the text between
/// "- **" and the following ".**", holding no asterisk.
fn bold_name_at(section: &str, start: usize) -> Option<(String, usize)> {
    let name = &section[start..];
    let end = name.find(".**")?;
    let name = &name[..end];
    if name.is_empty() || name.contains('*') {
        return None;
    }
    Some((name.to_string(), start + end + 3))
}

/// The principle names as the README itself states them.
fn readme_principles() -> Vec<String> {
    let readme = readme();
    let section = principles_section(&readme);
    let mut names = Vec::new();
    let mut cursor = 0;
    while let Some(rel) = section[cursor..].find("- **") {
        let candidate = cursor + rel;
        let start = candidate + 4;
        match bold_name_at(section, start) {
            Some((name, next)) => {
                names.push(name);
                cursor = next;
            }
            // A failed candidate resumes the scan one
            // character past its opening, as re.findall
            // advances one character past a failed match
            // attempt; a match resumes at its end.
            None => cursor = candidate + 1,
        }
    }
    names
}

/// The principle bullets' statements: the rest of each
/// bullet line after the bold name and its space, as the
/// regex - \*\*[^*]+\.\*\* (.+) captures.
fn readme_bullets() -> Vec<String> {
    let readme = readme();
    let section = principles_section(&readme);
    let mut bullets = Vec::new();
    let mut cursor = 0;
    while let Some(rel) = section[cursor..].find("- **") {
        let candidate = cursor + rel;
        let start = candidate + 4;
        let Some((_, next)) = bold_name_at(section, start) else {
            cursor = candidate + 1;
            continue;
        };
        // The pattern continues with a space and the rest
        // of the line; a candidate without them is no
        // match, and the scan resumes one character past
        // its opening. A matched bullet is consumed to
        // the end of its line, the match's end, as
        // findall resumes there.
        let line = section[next..]
            .strip_prefix(' ')
            .map(|tail| tail.split('\n').next().unwrap_or(""));
        match line {
            Some(line) if !line.is_empty() => {
                bullets.push(line.to_string());
                cursor = next + 1 + line.len();
            }
            _ => cursor = candidate + 1,
        }
    }
    bullets
}

/// The sections of PRACTICE, split where a line begins
/// "## " as the regex ^## with MULTILINE splits, the
/// first part dropped, each part's name its first line
/// stripped and its body the remainder after the first
/// newline. A repeated name keeps its first position and
/// takes the later body, as a dict assignment does.
fn sections() -> Vec<(String, String)> {
    let practice = practice();
    let bytes = practice.as_bytes();
    let mut boundaries: Vec<usize> = vec![0];
    for index in 1..bytes.len() {
        if bytes[index - 1] == b'\n' && practice[index..].starts_with("## ") {
            boundaries.push(index);
        }
    }
    boundaries.push(practice.len());
    // The first part is dropped, unless the text itself
    // opens with a heading, where the split leaves an
    // empty first part and the heading's part is real.
    let dropped = if practice.starts_with("## ") { 0 } else { 1 };
    let mut found: Vec<(String, String)> = Vec::new();
    for window in boundaries.windows(2).skip(dropped) {
        let part = &practice[window[0] + 3..window[1]];
        let (name, body) = match part.split_once('\n') {
            Some((name, body)) => (py_strip(name).to_string(), body.to_string()),
            None => (py_strip(part).to_string(), String::new()),
        };
        if let Some(entry) = found.iter_mut().find(|(held, _)| *held == name) {
            entry.1 = body;
        } else {
            found.push((name, body));
        }
    }
    found
}

fn section_body<'a>(found: &'a [(String, String)], name: &str) -> &'a str {
    found
        .iter()
        .find(|(held, _)| held == name)
        .map(|(_, body)| body.as_str())
        .expect("a practice section")
}

#[test]
fn test_the_copy_bar_holds_over_practice() {
    let practice = practice();
    let folded_text = folded(&practice);
    for char in FORBIDDEN {
        assert!(!practice.contains(char), "barred character {char:?}");
        assert!(!folded_text.contains(char), "barred character {char:?}");
    }
}

#[test]
fn test_the_copy_bar_is_the_same_bar_round_1_enforces() {
    // The Python reads the documents test module with its
    // ast and compares the FORBIDDEN assignment as a set.
    // The port reads the documents test source and collects
    // the char literals in its FORBIDDEN definition.
    let source = read_document("tests/test_documents.rs");
    let definition = source
        .split_once("const FORBIDDEN")
        .map(|(_, tail)| tail)
        .expect("the documents suite defines FORBIDDEN");
    let list = definition
        .split_once('=')
        .map(|(_, tail)| tail)
        .expect("the definition assigns its list");
    let list = list
        .split_once('[')
        .map(|(_, tail)| tail)
        .expect("the definition opens its list");
    let list = before_first(list, "]");
    let mut theirs: BTreeSet<char> = BTreeSet::new();
    let chars: Vec<char> = list.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '\'' {
            let inner: String = chars[index + 1..]
                .iter()
                .take_while(|c| **c != '\'')
                .collect();
            if let Some(hex) = inner
                .strip_prefix("\\u{")
                .and_then(|rest| rest.strip_suffix('}'))
            {
                let value = u32::from_str_radix(hex, 16).expect("a hex escape");
                theirs.insert(char::from_u32(value).expect("a scalar escape"));
            } else {
                let mut inner_chars = inner.chars();
                if let Some(only) = inner_chars.next() {
                    assert!(inner_chars.next().is_none(), "a one char literal");
                    theirs.insert(only);
                }
            }
            index += inner.chars().count() + 2;
        } else {
            index += 1;
        }
    }
    let mine: BTreeSet<char> = FORBIDDEN.into_iter().collect();
    assert_eq!(theirs, mine);
}

#[test]
fn test_one_section_per_principle_in_readme_order() {
    let names = readme_principles();
    assert_eq!(names.len(), 10);
    let found = sections();
    let found_names: Vec<&str> = found.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(
        found_names,
        names.iter().map(|name| name.as_str()).collect::<Vec<_>>()
    );
}

#[test]
fn test_every_section_is_practice_not_restatement() {
    let bullets = readme_bullets();
    let found = sections();
    for (name, bullet) in readme_principles().iter().zip(bullets.iter()) {
        let body = section_body(&found, name);
        assert!(body.chars().count() >= 400, "{name}");
        let probe: String = collapsed(bullet).chars().take(60).collect();
        assert!(!collapsed(body).contains(&probe), "{name}");
    }
}

#[test]
fn test_the_document_states_its_as_of_date_and_claims_no_counts() {
    let practice = practice();
    assert!(practice.contains("as worked on 2026-11-02"));
    let lowered: String = practice.chars().flat_map(char::to_lowercase).collect();
    for phrase in [
        "of 100",
        "rounds closed",
        "hundred rounds",
        "thousand rounds",
        "300 rounds",
        "three hundred",
    ] {
        assert!(!lowered.contains(phrase), "{phrase}");
    }
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    for word in counted_rounds_words(&lowered) {
        assert!(
            !numbers.contains(&word.as_str()) && !word.chars().all(|c| c.is_ascii_digit()),
            "{word} rounds"
        );
    }
}

/// The words the regex \b[a-z0-9]+ rounds\b matches over
/// the lowered text: a run of ASCII letters and digits,
/// beginning at a word boundary, followed by " rounds"
/// and a word boundary.
fn counted_rounds_words(lowered: &str) -> Vec<String> {
    let chars: Vec<char> = lowered.chars().collect();
    let mut words = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        if chars[index].is_ascii_lowercase() || chars[index].is_ascii_digit() {
            let boundary = index == 0 || !is_word_char(chars[index - 1]);
            let mut end = index;
            while end < chars.len()
                && (chars[end].is_ascii_lowercase() || chars[end].is_ascii_digit())
            {
                end += 1;
            }
            if boundary {
                let tail: String = chars[end..].iter().take(7).collect();
                if tail == " rounds" && (end + 7 >= chars.len() || !is_word_char(chars[end + 7])) {
                    words.push(chars[index..end].iter().collect());
                }
            }
            index = end;
        } else {
            index += 1;
        }
    }
    words
}

/// The spans the regex `([^`]+)` finds: text between
/// pairs of backticks in order, a span holding at least
/// one character, an unclosed backtick starting the
/// search again one character later.
fn backtick_spans(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let next_backtick = |from: usize| {
        bytes[from..]
            .iter()
            .position(|byte| *byte == b'`')
            .map(|rel| from + rel)
    };
    let mut spans = Vec::new();
    let mut cursor = 0;
    while let Some(open) = next_backtick(cursor) {
        let Some(close) = next_backtick(open + 1) else {
            break;
        };
        if close == open + 1 {
            cursor = open + 1;
            continue;
        }
        spans.push(&text[open + 1..close]);
        cursor = close + 1;
    }
    spans
}

/// The captures of the regex \b{tool} ([a-z][a-z-]+)\b:
/// the tool named at a word boundary, a space, then a
/// run of lowercase letters and dashes whose cut, found
/// by the regex's backtracking, ends at a word boundary.
/// The scan follows findall's advancement: a match
/// resumes at the match's end, a failed attempt advances
/// one character.
fn tool_mentions(text: &str, tool: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let tool_chars: Vec<char> = tool.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index + tool_chars.len() <= chars.len() {
        let mut matched_end = None;
        if chars[index..index + tool_chars.len()] == tool_chars[..]
            && (index == 0 || !is_word_char(chars[index - 1]))
        {
            let after = index + tool_chars.len();
            if after < chars.len() && chars[after] == ' ' {
                let start = after + 1;
                if start < chars.len() && chars[start].is_ascii_lowercase() {
                    let mut end = start;
                    while end < chars.len()
                        && (chars[end].is_ascii_lowercase() || chars[end] == '-')
                    {
                        end += 1;
                    }
                    let run = &chars[start..end];
                    for cut in (2..=run.len()).rev() {
                        let last = run[cut - 1];
                        let next = if cut < run.len() {
                            Some(run[cut])
                        } else {
                            chars.get(end).copied()
                        };
                        if is_word_char(last) != next.is_some_and(is_word_char) {
                            found.push(run[..cut].iter().collect());
                            matched_end = Some(start + cut);
                            break;
                        }
                    }
                }
            }
        }
        index = matched_end.unwrap_or(index + 1);
    }
    found
}

/// The word character set the matchers above judge
/// boundaries with: a sweep of the whole codepoint
/// space must accept exactly the interpreter's 137,936
/// word characters, refusing the combining marks a
/// bare alphanumeric classification would accept.
#[test]
fn test_the_word_character_set_matches_the_interpreter() {
    let mut accepted = 0;
    for cp in 0..=0x10FFFFu32 {
        let Some(char) = char::from_u32(cp) else {
            continue;
        };
        if is_word_char(char) {
            accepted += 1;
        }
    }
    assert_eq!(accepted, 137_936);
    for refused in ['\u{345}', '\u{363}', '\u{5b0}'] {
        assert!(!is_word_char(refused), "{refused:?}");
    }
    assert!(is_word_char('_'));
    assert!(is_word_char('7'));
    assert!(is_word_char('\u{323af}'));
}

#[test]
fn test_no_command_is_named_that_does_not_exist() {
    let practice = practice();
    for span in backtick_spans(&practice) {
        let words = split_words(span);
        let named = words
            .first()
            .and_then(|first| REAL_COMMANDS.iter().find(|(tool, _)| tool == first));
        match named {
            Some((_, commands)) => {
                assert!(words.len() >= 2 && commands.contains(&words[1]), "{span}");
            }
            None => {
                assert!(
                    !span.contains('/') && !span.contains(" --") && !span.ends_with(".py"),
                    "{span}"
                );
            }
        }
    }
    for (tool, commands) in REAL_COMMANDS {
        for mention in tool_mentions(&practice, tool) {
            assert!(commands.contains(&mention.as_str()), "{tool} {mention}");
        }
    }
}
