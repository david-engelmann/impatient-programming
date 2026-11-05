# impatient-programming

Impatient Programming as one practitioner runs it. This repo is David Engelmann's own take on the method, and the tools that embody it are under construction beside it.

A note on the name. Another method travels under the same name at impatientprogramming.org, written by Obie Fernandez. This repo is not that method and does not speak for it. The overlap is the name. Everything else here is a separate practice, stated on its own terms.

## What this is

Extreme Programming answered a question for its time. What changes when the code is written by people in a room together, all day, at human speed? The room has changed. It now holds one human and a fleet of agents, the code arrives at machine speed, and the bottleneck moved with it. The bottleneck is no longer typing. It is waiting, and it is human attention, and it is knowing whether the work is actually good.

This repo holds the statement of the practice. The practice itself runs across a portfolio of small tools, each one being built around a single job. The principles below state the rules, and the tools are those rules at work.

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

The state under each name is the rounds that repo has closed under the counting rule above, exactly as its own README reports them.

- **flighttape** records an agent session as a tape that can be validated, played back, digested, compared, sliced, screened, and rolled into a set level record by the directory rollup. Fourteen of one hundred rounds closed.
- **retroforge** reads a period of flight tapes and opens one pull request against the agent setup, proposing rules file edits, hook changes, and roster notes, every hunk annotated with the trace moments that justify it. Six of one hundred rounds closed.
- **bluepencil** edits the rules file itself, striking the rules no agent follows, rewriting the ones agents misread with the misreading quoted, and promoting the buried ones, delivered as a diff for a human and never applied on its own. Six of one hundred rounds closed.
- **unheard** scores standing instructions by observed obedience across a corpus of runs and names the ones agents systematically ignore, with counts and never impressions, and writes the observations file bluepencil consumes. Six of one hundred rounds closed.
- **onrecord** keeps claims with their receipts across runs, so what was asserted can be retrieved, superseded, and challenged. Five of one hundred rounds closed.
- **wallofwrong** takes a plain sentence confession of what is about to happen before anything irreversible runs, then watches what actually happens and grades the sentence against it. Six of one hundred rounds closed.
- **secondbill** meters the tax a fleet pays in rework, reverted diffs, redone tasks, and fix-forward commits, and bills it to the commit that caused it, as an itemized monthly statement. Five of one hundred rounds closed.
- **secondtake** compares tapes from before and after a setup change, matched by task, on cost, time, rework, and rescue rate, and ends in one outcome, keep, tune, revert, or an honest abstain when the evidence cannot carry a proposal, with the numbers attached. Five of one hundred rounds closed.
- **verdictloop** is the experiment registry for the whole family, where a proposed setup change from any of these tools becomes a tracked experiment with a baseline, a result, and a public verdict of keep, kill, or rerun. Five of one hundred rounds closed.
- **impatient-programming** is this repo, the statement of the practice the other nine embody. Six of one hundred rounds closed.

The same method runs across the rest of the portfolio, every repo held to the same gates and the same counting.

## Status

The name stays. This repo is one practitioner's take, and the work is under way.

## Programme status

- Closed perfection rounds, 6 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.
- Contract, IP-1 to IP-10. The first six were written in Round 1 and met in the same round. IP-7 to IP-10 were written in Round 2 ahead of PRACTICE.md and met in the same round.
- Local gate, python3 scripts/codepoint_scan.py, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test. CI runs the same commands.

## Licence

MIT. See LICENSE.
