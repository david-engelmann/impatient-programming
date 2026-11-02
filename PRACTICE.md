# The practice

This document describes the practice as worked on 2026-11-02, one section per principle in the order the README lists them. It describes mechanisms, not results, and it claims no round counts for any repo. The counts live in each repo's own README and move only when that repo closes a round.

## Speed is the point

Work runs in parallel lanes. Each lane holds one unit of work, one worktree, and one branch, and lanes do not wait on each other unless a real dependency ties them. A decision that has been made is executed at once, and a lane that finishes starts the next unit without being told. Heavy builds are serialised on purpose. One build at a time runs at full strength while the other lanes do the reading and writing that need no build time, and a lane that waits for the machine says so plainly instead of racing and corrupting the signal for everyone. Waiting is treated as information. When a lane waits, it names what it waits on, and a wait that repeats becomes the next thing fixed.

## Local gates before CI

Nothing is pushed until the full suite has run locally on the exact head that will be pushed. The run is captured with its exit code and its test count, and the tree that produced those numbers is the tree that ships. CI is read afterwards as a backstop. A green CI run on a head the local gate never saw proves nothing, and a red CI run whose logs nobody has read is not a fact yet. When CI goes red on the main branch for its own reasons, fixing it becomes the first priority, and no merge proceeds on a head whose own CI is known red.

## Three reviews or it is not done

A checkpoint is a finished feature, a closed round, or the final slice of a stack, and before any of them counts, the work gets three reviews. The first is a red team re-audit from fresh eyes, reviewers who did not build the thing, checking every claim against the code and running what can be run. The second is a line by line review of the actual diff on two axes that run in parallel and never see each other's findings. One axis asks whether the diff implements the contract it cites, hunk by hunk. The other asks whether it meets the repo's standards, its conventions, its copy bar, and the rule that every test must be able to fail for a real reason. The third review is the owner's own pass, re-running the key commands and re-checking the key claims instead of trusting a report that says verified. Every finding is fixed, or set aside with a written reason, before the checkpoint closes. An automated review bot reads every pull request as an advisory pass, and its findings are settled before the merge. Its valid findings get fixed like anyone's, and its approval is never counted as one of the three.

## Contract before code

The obligations a change must meet are written before the change, each with the test that will judge it. The contract is prose a test can kill. It names the behaviour, the refusal cases, and the exact test file that pins them. Code written before its contract exists is treated as a draft, however good it looks. A bug found in shipped work earns its obligation first. The obligation is added to the contract, a test is written that fails on the shipped behaviour, the test is shown to fail, and only then is the fix written. Amendments to a contract are recorded in the contract itself, with the reason, in the round that discovered them.

## Count honestly

Progress is reported in closed rounds and nothing else. A round is one full cycle of work, audit, review, red team, and approval, with the findings fixed. Research passes are not rounds. Test runs are not rounds. Log entries are not rounds, and a programme that logs a great many entries has done however many rounds it closed, which is a smaller and more honest number. Each repo's README carries its own count. The count is written into a closure commit after the round's three reviews and the owner's verification of the branch, and that commit reaches the main branch only through the merge, so the count a README shows on main always belongs to a round that has landed. The round itself is not counted closed until merged main verifies from a fresh clone, and the dossier records that verification. Where one repo republishes another repo's count, the republication is refreshed in the same pass that lands the sibling round, and a stale republication is a defect like any other. When a count is corrected, the correction is stated as plainly as the count was.

## Measure everything

Every run leaves its numbers behind. A converted tape records its token totals and counts the lines it skipped, each with the reason it was skipped. A slice, which is also a tape, omits the token totals it cannot honestly carry and records the source it was cut from instead. A run that excludes a file names the file and the reason in its report, instead of absorbing it into a cleaner total. A review writes its findings into the round's record with their counts by severity, and a finding set aside is kept with its reason beside it. A spend is logged when it happens, with the running total kept current, however small the amount. Counts are published with their denominators beside them, and a rate is never printed without the population it came from. When two views of the same numbers are both useful, both are kept and both are reported, because the consumers of most of this data are agents choosing what to do next, and a view dropped for tidiness is a choice they no longer get to make. A limitation is stated as a limitation, in a sentence, and the work proceeds. It is never cured by quietly narrowing what gets measured.

## Verify the real thing

A claim is checked against the artifact itself. A merged round is verified from a fresh clone of the merged main, with the suite run there. A lighter landing is verified by reading the remote tree back through the API and comparing it with the local tree, and the record states which kind of verification each landing received. A page is verified rendered, at the width and on the device where it will be read, with the interactions exercised the way a hand exercises them. Counts and hashes and green badges are evidence about a thing. They are never the thing itself, and a verification that never touched the artifact is reported as what it is, a proxy, or not reported as verification at all.

## Work survives the worker

Every lane works in a worktree under the workspace, never in scratch space that a restart can sweep away. Commits land early and often in a worktree that persists on disk, and the finished branch is pushed in one pass before its pull request opens, so a worker that vanishes mid task leaves the work in its worktree for the next worker to pick up. The state of every programme lives in files the next worker reads, a running status file kept current at every landing, and a written record per round holding its reviews and how each finding was settled. Nothing that matters exists in only one place, and finished work is backed up somewhere the owner can open without the agent.

## The perfection bar

Work that carries the owner's name is examined until a full pass finds nothing. Public copy is read end to end against the copy bar before it ships, every time, because copy that ships unread ships with its mistakes in it. Findings are fixed without argument, including the review bot's, and a dismissal is written down with its reason rather than performed in a comment thread. The standard is not a number of passes. The standard is consecutive clean passes, and the owner sees the final artifact before anything publishes under their name.

## Agents work, the human judges

Agents run the lanes. They do the research, write the code, review each other, and keep the records. The human's part is the judgement only a human can supply, and it is asked for in a form that respects their attention. A decision is put as options, with a recommendation and the reasoning in plain words, and when the human picks against the recommendation, the pick is implemented without relitigation. What only the human can supply is asked for plainly and early. Everything else is figured out by the agents, and an agent that routes a solvable problem to the human has failed at its part of the practice.
