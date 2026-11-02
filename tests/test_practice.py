"""IP-7 to IP-10, the practice document."""
import pathlib
import unicodedata

ROOT = pathlib.Path(__file__).resolve().parent.parent
PRACTICE = (ROOT / "PRACTICE.md").read_text(encoding="utf-8")

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

FORBIDDEN = [":", ";", "\u2014", "\u2013", "\u2015", "\uff1a",
             "\uff1b", "\ufe58", "\ufe54"]


def test_the_copy_bar_holds_over_practice():
    folded = unicodedata.normalize("NFKC", PRACTICE)
    for char in FORBIDDEN:
        assert char not in folded, f"barred character {char!r}"


def test_one_section_per_principle_in_readme_order():
    positions = []
    for name in PRINCIPLES:
        heading = f"## {name}"
        assert PRACTICE.count(heading) == 1, name
        positions.append(PRACTICE.index(heading))
    assert positions == sorted(positions)


def test_the_document_states_its_as_of_date_and_claims_no_counts():
    assert "as worked on 2026-11-02" in PRACTICE
    assert "of 100" not in PRACTICE
    assert "rounds closed" not in PRACTICE


def test_no_command_is_named_that_this_repo_does_not_have():
    import re
    spans = re.findall(r"`([^`]+)`", PRACTICE)
    assert spans == []
