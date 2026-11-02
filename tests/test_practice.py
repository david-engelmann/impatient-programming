"""IP-7 to IP-10, the practice document."""
import pathlib
import re
import unicodedata

ROOT = pathlib.Path(__file__).resolve().parent.parent
PRACTICE = (ROOT / "PRACTICE.md").read_text(encoding="utf-8")
README = (ROOT / "README.md").read_text(encoding="utf-8")

FORBIDDEN = [":", ";", "\u2014", "\u2013", "\u2015", "\uff1a",
             "\uff1b", "\ufe58", "\ufe54"]

REAL_COMMANDS = {
    "flighttape": {"convert", "convert-dir", "validate", "play",
                   "digest", "verify", "compare", "rollup", "slice"},
    "retroforge": {"propose"},
    "bluepencil": set(), "unheard": set(), "onrecord": set(),
    "wallofwrong": set(), "secondbill": set(),
    "secondtake": set(), "verdictloop": set(),
}


def readme_principles():
    """The principle names as the README itself states them."""
    section = README.split("## Principles", 1)[1]
    section = section.split("## ", 1)[0]
    return re.findall(r"- \*\*([^*]+)\.\*\*", section)


def readme_bullets():
    section = README.split("## Principles", 1)[1]
    section = section.split("## ", 1)[0]
    return re.findall(r"- \*\*[^*]+\.\*\* (.+)", section)


def sections():
    parts = re.split(r"^## ", PRACTICE, flags=re.M)
    found = {}
    for part in parts[1:]:
        name, _, body = part.partition("\n")
        found[name.strip()] = body
    return found


def test_the_copy_bar_holds_over_practice():
    folded = unicodedata.normalize("NFKC", PRACTICE)
    for char in FORBIDDEN:
        assert char not in PRACTICE, f"barred character {char!r}"
        assert char not in folded, f"barred character {char!r}"


def test_the_copy_bar_is_the_same_bar_round_1_enforces():
    import ast
    source = (ROOT / "tests" / "test_documents.py").read_text(
        encoding="utf-8")
    for node in ast.walk(ast.parse(source)):
        if isinstance(node, ast.Assign) and any(
                isinstance(target, ast.Name)
                and target.id == "FORBIDDEN"
                for target in node.targets):
            theirs = ast.literal_eval(node.value)
            assert set(theirs) == set(FORBIDDEN)


def test_one_section_per_principle_in_readme_order():
    names = readme_principles()
    assert len(names) == 10
    found = sections()
    assert list(found) == names


def test_every_section_is_practice_not_restatement():
    bullets = readme_bullets()
    found = sections()
    for name, bullet in zip(readme_principles(), bullets):
        body = found[name]
        assert len(body) >= 400, name
        probe = " ".join(bullet.split())[:60]
        assert probe not in " ".join(body.split()), name


def test_the_document_states_its_as_of_date_and_claims_no_counts():
    assert "as worked on 2026-11-02" in PRACTICE
    lowered = PRACTICE.lower()
    for phrase in ("of 100", "rounds closed", "hundred rounds",
                   "thousand rounds", "300 rounds", "three hundred"):
        assert phrase not in lowered, phrase
    numbers = ("one", "two", "three", "four", "five", "six",
               "seven", "eight", "nine", "ten")
    for match in re.findall(r"\b[a-z0-9]+ rounds\b", lowered):
        word = match.split()[0]
        assert word not in numbers and not word.isdigit(), match


def test_no_command_is_named_that_does_not_exist():
    for span in re.findall(r"`([^`]+)`", PRACTICE):
        words = span.split()
        if words and words[0] in REAL_COMMANDS:
            assert len(words) >= 2 and words[1] in \
                REAL_COMMANDS[words[0]], span
        else:
            assert "/" not in span and " --" not in span \
                and not span.endswith(".py"), span
    for tool, real in REAL_COMMANDS.items():
        for match in re.findall(rf"\b{tool} ([a-z][a-z-]+)\b",
                                PRACTICE):
            assert match in real, f"{tool} {match}"
