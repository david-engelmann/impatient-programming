# impatient-programming

Impatient Programming as one practitioner runs it. This repo is David Engelmann's own take on the method, written down and built out in the open, with the tools that embody it shipping beside it.

A note on the name. Another method travels under the same name at impatientprogramming.org, written by Obie Fernandez. This repo is not that method and does not speak for it. The overlap is the name and the conviction that work in the agent era needs its own discipline. Everything past that point is a separate practice, stated here on its own terms.

## What this is

Extreme Programming answered a question for its time. What changes when the code is written by people in a room together, all day, at human speed? The room has changed. It now holds one human and a fleet of agents, the code arrives at machine speed, and the bottleneck moved with it. The bottleneck is no longer typing. It is waiting, and it is human attention, and it is knowing whether the work is actually good.

This repo holds the statement of the practice. The practice itself runs across a portfolio of small tools, each one built to remove a specific wait or to make a specific claim checkable. The tools are the take made literal. The principles below say how the work runs, and the tools are the work running.

## Principles

Ten principles, each one a rule, each one practiced on every repo in the portfolio.

- **Speed is the point.** Work runs in parallel lanes at the fastest pace the verification can hold. Waiting on a queue, on a permission already granted, or on a result nobody is watching is a defect in the system, not a fact of life.
- **Local gates before CI.** The full suite runs locally before anything is pushed. CI is a backstop that catches what the local run missed, never the first check and never an excuse.
- **Three reviews or it is not done.** Before a checkpoint counts, the work gets a red team re-audit from fresh eyes, a line by line review on two separate axes, and the owner's own verification with the key commands rerun from scratch. One green run is not a review.
- **Contract before code.** The obligations a change must meet are written first, in a form a test can kill. A bug found in shipped work earns its obligation and its failing test before it earns its fix.
- **Count honestly.** Progress is counted in closed rounds only, and a round is one full cycle of work, audit, review, red team, and approval. Sub steps, passes, and attempts are not rounds and are never reported as rounds.
- **Measure everything.** If a signal can be captured, it is captured, and the numbers are published with their uncertainty beside them. Detail is never dropped to make a chart simpler, and a tie is a measurement, not a problem.
- **Verify the real thing.** A claim is checked against the artifact itself, rendered, at the size and on the device where it will be seen. Counts, hashes, and green badges are evidence about a thing, never the thing itself.
- **Work survives the worker.** Every lane commits early and pushes often, and nothing that matters exists in only one place. Any worker, human or agent, can vanish mid task and the work loses nothing but time.
- **The perfection bar.** Work that carries the owner's name gets gone over until it is spotless, researched, reviewed, red teamed, and approved, as many passes as that takes. Good enough is a decision the owner makes out loud, never a default.
- **Agents work, the human judges.** Agents run the lanes, do the research, write the code, and check each other. The human sets the bar, supplies what only a human can, and makes the calls that are theirs alone.

## The tools

Each tool removes one wait or makes one claim checkable. The state under each name is the rounds that repo has closed under the counting rule above, exactly as its own README reports them.

- **flighttape** records an agent session as a tape that can be validated, played back, digested, compared, and rolled up into a set level record. Eight rounds closed of one hundred.
- **retroforge** reads a week of tapes and proposes changes to the setup that produced them, every proposal tied to the evidence that raised it. At setup, rounds not yet started.
- **bluepencil** grades how well agents followed the rules file they were given and drafts the edits that would have prevented the misreadings. At setup, rounds not yet started.
- **unheard** measures whether instructions were heard at all, scoring obedience across a corpus of runs. At setup, rounds not yet started.
- **onrecord** keeps claims with their receipts across runs, so what was asserted can be retrieved, superseded, and challenged. At setup, rounds not yet started.
- **wallofwrong** holds predictions and confessions against their due dates and grades them when the dates arrive. At setup, rounds not yet started.
- **secondbill** prices the work a repo actually did, the reverts, the redone tasks, and the fixes that followed failures, as one itemized statement. At setup, rounds not yet started.
- **secondtake** registers a change and measures the runs before and after it on cost, time, rework, and rescues, then issues a verdict. At setup, rounds not yet started.
- **verdictloop** keeps the registry of those verdicts, so the baseline, result, and ruling of every experiment stays on record. At setup, rounds not yet started.
- **impatient-programming** is this repo, the statement of the practice the other nine embody.

The same method runs across the rest of the portfolio, the benchmark work, the skill programme, and the gate truth tools, each held to the same gates and the same counting.

## How the work runs

Every repo in the line runs the same loop. A round opens with a contract, the obligations written before the work begins. The work lands in small steps on its own branch, each step verified locally before it moves. Three reviews close the round, the findings get fixed, and the merged result is verified again on the main branch before the round is counted. The count lives in the README of each repo and moves only when a round has fully closed.

## Status

The naming question is settled by owner ruling. The name stays, the repo is the owner's own take, and the rounds proceed.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.
- Contract, IP-1 to IP-6, written in Round 1 and met in the same round.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
