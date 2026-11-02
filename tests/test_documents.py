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

FORBIDDEN = [":", ";", "\u2014", "\u2013"]


def test_the_copy_bar_holds_in_both_documents():
    for name, text in (("README.md", README),
                       ("MANIFESTO.md", MANIFESTO)):
        for char in FORBIDDEN:
            assert char not in text, f"{name} contains {char!r}"


def test_the_readme_states_whose_take_this_is():
    assert "David Engelmann's own take" in README
    assert "impatientprogramming.org" in README
    assert "This repo is not that method" in README


def test_every_principle_is_named_in_the_readme():
    for principle in PRINCIPLES:
        assert f"**{principle}.**" in README, principle


def test_every_principle_has_a_manifesto_section():
    for principle in PRINCIPLES:
        assert f"## {principle}" in MANIFESTO, principle


def test_the_tools_section_names_all_ten_repos():
    section = README.split("## The tools", 1)[1].split("## ", 1)[0]
    for tool in TOOLS:
        assert f"**{tool}**" in section, tool


def test_the_tool_states_are_the_ones_the_repos_report():
    assert "Eight rounds closed of one hundred" in README
    assert README.count("At setup, rounds not yet started") == 8
