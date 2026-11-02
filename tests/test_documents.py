"""IP-1 to IP-6, the document obligations."""
import pathlib

ROOT = pathlib.Path(__file__).resolve().parent.parent
README = (ROOT / "README.md").read_text(encoding="utf-8")
MANIFESTO = (ROOT / "MANIFESTO.md").read_text(encoding="utf-8")

PRINCIPLES = [
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
]

TOOLS = [
    "flighttape", "retroforge", "bluepencil", "unheard",
    "onrecord", "wallofwrong", "secondbill", "secondtake",
    "verdictloop", "impatient-programming",
]

FORBIDDEN = [":", ";", "\u2014", "\u2013", "\u2015", "\uff1a",
             "\uff1b", "\ufe58", "\ufe54"]


# One distinctive phrase per tool, taken from that tool's own
# README, so two tools' lines cannot be swapped or blurred
# without a test failing.
TOOL_PHRASES = {
    "flighttape": "rolled up",
    "retroforge": "pull request",
    "bluepencil": "diff for a human",
    "unheard": "obedience",
    "onrecord": "receipts",
    "wallofwrong": "grades the sentence",
    "secondbill": "itemized monthly statement",
    "secondtake": "keep, tune, or revert",
    "verdictloop": "experiment registry",
    "impatient-programming": "statement of the practice",
}


def test_the_copy_bar_holds_in_both_documents():
    import unicodedata
    for name, text in (("README.md", README),
                       ("MANIFESTO.md", MANIFESTO)):
        # Compatibility forms fold to the barred characters
        # under NFKC, so the check runs on the folded text as
        # well as the raw text.
        folded = unicodedata.normalize("NFKC", text)
        for char in FORBIDDEN:
            assert char not in text, f"{name} contains {char!r}"
            assert char not in folded, f"{name} folds to {char!r}"


def test_the_readme_states_whose_take_this_is():
    assert "David Engelmann's own take" in README
    assert "impatientprogramming.org" in README
    assert "This repo is not that method" in README
    # The only mention of that method is the disambiguation
    # itself. A sentence claiming the mantle breaks this.
    assert README.count("that method") == 1


def test_every_principle_is_named_in_the_readme():
    assert "## Principles" in README
    for principle in PRINCIPLES:
        assert f"**{principle}.**" in README, principle
        bullet = next(line for line in README.splitlines()
                      if f"**{principle}.**" in line)
        # A name with no statement after it is not a principle.
        assert len(bullet) > 120, principle


def test_every_principle_has_a_manifesto_section():
    sections = MANIFESTO.split("\n## ")
    for principle in PRINCIPLES:
        assert f"## {principle}" in MANIFESTO, principle
        body = next(part for part in sections
                    if part.startswith(principle))
        # A heading over a gutted paragraph is not an argument.
        assert len(body) > 400, principle


def test_the_tools_section_names_all_ten_repos():
    section = README.split("## The tools", 1)[1].split("## ", 1)[0]
    for tool in TOOLS:
        assert f"**{tool}**" in section, tool


def test_the_tools_lines_carry_each_repos_own_job():
    section = README.split("## The tools", 1)[1].split("## ", 1)[0]
    bullets = {}
    for line in section.splitlines():
        for tool in TOOLS:
            if line.startswith(f"- **{tool}**"):
                bullets[tool] = line
    assert set(bullets) == set(TOOLS)
    for tool, phrase in TOOL_PHRASES.items():
        assert phrase in bullets[tool], tool
    # Each state sits on its own tool's line, so two states
    # cannot be swapped without a test failing.
    assert "Nine of one hundred rounds closed" in bullets[
        "flighttape"]
    assert "One of one hundred rounds closed" in bullets[
        "retroforge"]
    for tool in TOOLS:
        if tool in ("flighttape", "retroforge",
                    "impatient-programming"):
            continue
        assert "At setup, rounds not yet started" in bullets[tool], tool
    assert "of one hundred rounds closed" in bullets[
        "impatient-programming"]


CODE_SUFFIXES = {".py", ".js", ".mjs", ".jsx", ".ts", ".tsx",
                 ".go", ".rs", ".rb", ".sh", ".java", ".c",
                 ".cc", ".cpp", ".swift", ".php"}
SKIPPED_PARTS = {".git", "scripts", "tests", "venv", ".venv",
                 "env", "node_modules", "__pycache__"}


def test_no_product_code_lives_in_the_repo():
    code = [path for path in ROOT.rglob("*")
            if path.is_file()
            and path.suffix in CODE_SUFFIXES
            and not SKIPPED_PARTS.intersection(path.parts)]
    assert code == []


def test_the_wider_portfolio_sentence_is_present():
    assert ("The same method runs across the rest of the "
            "portfolio") in README


# The states below are pins on what the sibling repos report,
# refreshed by hand whenever a sibling closes a round and
# checked against those repos by the round reviews.
def test_the_tool_states_are_pinned_to_the_repos_reports():
    assert "Nine of one hundred rounds closed" in README
    assert README.count("At setup, rounds not yet started") == 7
