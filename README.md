# impatient-programming

Impatient Programming for the agent era. The values and practices of Extreme Programming, reworked for one human directing a fleet of agents.

This repo is a method statement, not a tool. It holds three documents that state a working method for one human directing a fleet of agents, and a Rust test suite that guards the documents so their claims cannot drift from what the contract requires. The method is read here and worked across a portfolio of small tools, each built around a single job and listed below with the state its own README reports.

```
python3 scripts/codepoint_scan.py
codepoint scan U+2014=0 U+2013=0
```

## Install

There is nothing to install. The repo is three documents and the test suite that guards them, and reading it needs only the files themselves. Taking a copy with the guard included takes the GitHub CLI, and running the guard takes a Rust toolchain and Python 3 for the scan.

```
gh repo clone david-engelmann/impatient-programming
cd impatient-programming
```

## Quickstart

The smallest complete run is the guard suite itself, run from the repo root.

```
cargo test
```

Twenty tests pass on the current tree, thirteen over the documents and seven over the practice. The suite reads the documents as the interpreter reads them and fails on any pinned claim the documents no longer match. The pins live in this repo and nothing compares them with the sibling repos, so a sibling can change while the suite still passes, and that change surfaces only when a refresh reads the siblings back and re-pins their states. The scan below runs over every prose, source, and workflow file in the repo and prints one line.

```
python3 scripts/codepoint_scan.py
codepoint scan U+2014=0 U+2013=0
```

## Usage

Read MANIFESTO.md for the argument, principle by principle, in the order the principles bite during a working day. Read PRACTICE.md for the same method as mechanisms, what a lane actually does, stated as of the date on its first page. Read the principles and the tools record below for the short form, and for the state of the portfolio that works the method.

Run the guard after any edit to the documents. The documents suite pins the copy bar, the principle names, the tool inventory, and every state the tools record carries. The practice suite pins the practice document's sections, the commands it names, and its claim that it counts nothing. An edit that moves a claim moves the pin with it in the same change, or the suite fails and the edit is not done.

## Principles

Every repo in the portfolio is built and reviewed against these ten principles, as its rounds proceed.

- **Speed is the point.** Work runs in parallel lanes at the fastest pace the verification can hold. Waiting on a queue, on a permission already granted, or on a result nobody is watching is a defect in the system, not a fact of life.
- **Local gates before CI.** The full suite runs locally before anything is pushed. CI is a backstop that catches what the local run missed, never the first check and never an excuse.
- **Three reviews or it is not done.** Before a checkpoint counts, the work gets a red team re-audit from fresh eyes, a line by line review on two separate axes, and the owner's own verification with the key commands rerun from scratch. One green run is not a review.
- **Contract before code.** The obligations a change must meet are written first, in a form a test can kill. A bug found in shipped work earns its obligation and its failing test before it earns its fix.
- **Count honestly.** Progress is counted in closed rounds only, and a round is one full cycle of work, audit, review, red team, and approval. Sub steps, passes, and attempts are not rounds and are never reported as rounds.
- **Measure everything.** If a signal can be captured, it is captured, and the numbers are published with their uncertainty beside them. Detail is never dropped to make a chart simpler, and a tie is a measurement, not a problem.
- **Verify the real thing.** A claim is checked against the artifact itself, rendered, at the size and on the device where it will be seen. Counts, hashes, and green badges are evidence about a thing, never the thing itself.
- **Work survives the worker.** Every lane commits early and pushes often, and nothing that matters exists in only one place. Any worker, human or agent, can vanish mid task and the work loses nothing but time.
- **The perfection bar.** Work that carries the owner's name is examined until it is spotless, researched, reviewed, red teamed, and approved, as many passes as that takes. Good enough is a decision the owner makes out loud, never a default.
- **Agents work, the human judges.** Agents run the lanes, do the research, write the code, and check each other. The human sets the bar, supplies what only a human can, and makes the calls that are theirs alone.

## The tools

The state under each name is the rounds that repo has closed under this record's counting rule, exactly as its own README reports them.

- **flighttape** records an agent session, and every run from every harness lands as one flight tape that can be validated, played back, digested, compared, sliced, screened, and rolled up over a whole directory into a set level record. Sixteen of one hundred rounds closed.
- **retroforge** reads a period of flight tapes and writes one proposal file for the agent setup, proposing rules file edits, hook changes, and roster notes, every hunk annotated with the trace moments that justify it. Eight of one hundred rounds closed.
- **bluepencil** edits the rules file itself, striking the rules no agent follows, rewriting the ones agents misread with the misreading quoted, and promoting the buried ones, delivered by its diff as a readable list of edits and never applied on its own. Eight of one hundred rounds closed.
- **unheard** scores standing instructions by observed obedience across a corpus of runs and names the ones agents systematically ignore, with counts and never impressions, and writes the observations file bluepencil consumes. Nine of one hundred rounds closed.
- **onrecord** keeps claims with their receipts across runs, so what was asserted can be retrieved, superseded, and challenged. Eight of one hundred rounds closed.
- **tetlock** takes a plain sentence confession of what is about to happen before anything irreversible runs, then watches what actually happens and grades the sentence against it. Nine of one hundred rounds closed.
- **muda** meters the tax a fleet pays in rework, reverted diffs, redone tasks, and fix-forward commits, and bills it to the commit that caused it, as an itemized statement for one stated calendar month. Eight of one hundred rounds closed.
- **secondtake** compares tapes from before and after a setup change, matched by task, on cost, time, rework, and rescue rate, ending in a keep, tune, or revert proposal with the numbers attached, and abstaining for insufficient evidence below its pair floor. Nine of one hundred rounds closed.
- **touchstone** is the experiment registry for the whole family, where a proposed setup change from any of these tools becomes a tracked experiment with a baseline, a result, and a public verdict of keep, kill, or rerun. Eight of one hundred rounds closed.
- **impatient-programming** is this repo, the statement of the practice the other nine embody. Thirteen of one hundred rounds closed.

The same method runs across the rest of the portfolio, every repo held to the same gates and the same counting.

## Status

The method is stated in full in this README and in MANIFESTO.md and PRACTICE.md. The tools that embody it are under construction across the portfolio.

- Closed perfection rounds, 13 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.
- Contract, IP-1 to IP-84. The obligations the documents and the guard must meet, each with the test that judges it.
- Local gate, python3 scripts/codepoint_scan.py, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test. CI runs the same commands.

## License

MIT. See [LICENSE](LICENSE).
