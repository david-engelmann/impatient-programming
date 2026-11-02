# The practice

This document describes the practice as worked on 2026-11-02. It states how each principle in the README is actually done, in the order the README lists them. It describes mechanisms, not results, and it claims no round counts for any repo. The counts live in each repo's own README and move only when that repo closes a round.

## Speed is the point

Work runs in parallel lanes. Each lane holds one unit of work, one worktree, and one branch, and lanes do not wait on each other unless a real dependency ties them. A decision that has been made is executed at once, and a lane that finishes starts the next unit without being told. Heavy builds are serialized on purpose. One build at a time runs at full strength while the other lanes do the reading and writing that cost nothing, and a lane that waits on the build slot says so in plain words instead of racing and corrupting the signal for everyone. Waiting is treated as information. When a lane waits, it names what it waits on, and a wait that repeats becomes the next thing fixed.

## Local gates before CI

Nothing is pushed until the full suite has run locally on the exact head that will be pushed. The run is captured with its exit code and its test count, and the tree that produced those numbers is the tree that ships. Where a repo has two modes, both modes run. CI is read afterwards as a backstop. A green CI run on a head the local gate never saw proves nothing, and a red CI run whose logs nobody has read is not a fact yet. When CI goes red on the main branch for its own reasons, fixing it becomes the top lane, and no merge proceeds on a head whose own CI is known red.

## Three reviews or it is not done

A checkpoint is a finished feature, a closed round, or the final slice of a stack, and before any of them counts, the work gets three reviews. The first is a red team re-audit from fresh eyes, reviewers who did not build the thing, checking every claim against the code and running what can be run. The second is a line by line review of the actual diff on two axes that run in parallel and never see each other's findings. One axis asks whether the diff implements the contract it cites, hunk by hunk. The other asks whether it meets the repo's standards, its conventions, its copy bar, and the rule that every test must be able to fail for a real reason. The third review is the owner's own pass, re-running the key commands and re-checking the key claims instead of trusting a report that says verified. Every finding is fixed or dispositioned with a written reason before the checkpoint closes. An automated review bot runs earlier as an advisory pass. Its valid findings get fixed like anyone's, and its approval is never counted as one of the three.

## Contract before code

The obligations a change must meet are written before the change, each with the test that will judge it. The contract is prose a test can kill. It names the behavior, the refusal cases, and the exact test file that pins them. Code written before its contract exists is treated as a draft, however good it looks. A bug found in shipped work earns its obligation first. The obligation is added to the contract, a test is written that fails on the shipped behavior, the test is shown to fail, and only then is the fix written. Amendments to a contract are recorded in the contract itself, with the reason, in the round that discovered them.

## Count honestly

Progress is reported in closed rounds and nothing else. A round is one full cycle of work, audit, review, red team, and approval, with the findings fixed. Research passes are not rounds. Test runs are not rounds. Log entries are not rounds, and a programme that logs three hundred entries has done however many rounds it closed, which is a smaller and more honest number. Each repo's README carries its own count, and the count moves only in a closure commit written after the round's reviews and the owner's verification. When a count is corrected, the correction is stated as plainly as the count was.

## Measure everything

If a signal can be captured, it is captured. The consumers of most of this data are agents, and an agent deciding from a partial picture decides worse, so detail is not dropped to make a statistic easier. When two views of the same thing are both useful, both are kept and both are reported. Numbers are published with their uncertainty beside them, and a result too close to call is reported as the measurement it is. A limitation is stated as a limitation, in a sentence, and the work proceeds. It is never cured by quietly narrowing what gets measured.

## Verify the real thing

A claim is checked against the artifact itself. A merged pull request is verified from a fresh clone of the merged branch, with the suite run there. A pushed branch is verified by reading the remote tree back through the API and comparing it with the local tree. A page is verified rendered, at the width and on the device where it will be read, with the interactions exercised the way a hand exercises them. Counts and hashes and green badges are evidence about a thing. They are never the thing itself, and a verification that never touched the artifact is reported as what it is, a proxy, or not reported as verification at all.

## Work survives the worker

Every lane works in a worktree under the workspace, never in scratch space that a restart can sweep away. Commits land early and often, and branches are pushed as the work goes, so a worker that vanishes mid task takes nothing with it but time. The state of every programme lives in files the next worker reads, a status file with the live counts, a dossier per round with the reviews and the dispositions, and a goal file that is updated in the same change that moves the work. Nothing that matters exists in only one place, and finished work is backed up somewhere the owner can open without the agent.

## The perfection bar

Work that carries the owner's name is examined until a full pass finds nothing. Public copy is read end to end against the copy bar before it ships, every time, because unaudited copy has shipped before and the lesson was paid for once already. Findings are fixed without argument, including the ones that arrive from an advisory bot, and a dismissal is written down with its reason rather than performed in a comment thread. The standard is not a number of passes. The standard is consecutive clean passes, and the owner sees the final artifact before anything publishes under their name.

## Agents work, the human judges

Agents run the lanes. They do the research, write the code, review each other, and keep the records. The human's part is the judgement only a human can supply, and it is asked for in a form that respects their attention. A decision is put as options, with a recommendation and the reasoning in plain words, and when the human picks against the recommendation, the pick is implemented without relitigation. What only the human can supply is asked for plainly and early. Everything else is figured out by the agents, and an agent that routes a solvable problem to the human has failed at its part of the practice.
