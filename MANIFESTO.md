# Manifesto

## Why this exists

Software process was designed around the costs of its era. Writing code was slow, so process protected the writing. Review was expensive, so process rationed the review. Coordination was the hard part, so process built ceremonies for coordination. Every one of those costs has inverted. An agent writes code faster than a human can read it, review capacity is a fleet decision rather than a calendar decision, and coordination happens in written artifacts that machines keep current. A process designed for the old costs now spends its effort protecting the wrong things, and the bill arrives as waiting.

Impatient Programming, as practiced here, starts from the inversion. The scarce resources are human attention and verified truth. Everything in the method either removes a wait, protects attention, or makes a claim checkable. What follows is the argument principle by principle, in the order the principles bite during a working day.

## Speed is the point

An idle agent is a stalled production line, and a stalled line is a defect in the system rather than a fact of life. Most delay in agent era work is not thinking time and not writing time. It is queues, handoffs, permissions asked twice, and results sitting unwatched. The practice therefore runs work in parallel lanes, decides small questions without escalation, and treats every named wait as a bug with an owner. Speed here is not haste. The pace is set by how fast the verification can honestly run, and the rest of the principles exist to make that pace safe.

## Local gates before CI

A shared build queue is the most expensive place to learn that code does not compile. The full test suite runs on the machine doing the work, before anything is pushed, every time. Remote CI then does what it is good at, which is catching the environment differences and the mistakes that local habits share. Reversing the order trains a worse habit, pushing first and reading the results later, and it spends the fleet's attention on failures that a local run would have caught in seconds. The rule is simple. Nothing reaches the shared queue untested, and a green remote run is a backstop, never the first evidence.

## Three reviews or it is not done

The builder of a thing is the worst auditor of it, because the builder already believes it. Before any checkpoint counts as finished, the work gets three separate examinations. A red team re-audits the finished state from fresh eyes and tries to break it. A line by line review then reads the actual change on two axes that never merge, whether it does what was asked, and whether it is built the way this house builds. Finally the owner verifies the result directly, rerunning the key commands rather than trusting any report, including this programme's own. The failure this prevents is the quiet one, the change that passed its tests and still was not what was asked, or was not built to last. One green run has never once been a review.

## Contract before code

Work that starts from a vague intention ends with a vague argument about whether it is done. Every round here opens by writing the obligations the work must meet, in language a test can kill. The contract is short, public inside the repo, and written before the implementation, so the review at the end has something firmer than taste to judge against. When a bug is found in shipped work, the order matters even more. The obligation is written first, the test that fails for the right reason comes second, and the fix comes last. A fix without the obligation is a rumor that the bug is gone.

## Count honestly

Programmes rot from the inside when their numbers flatter them. The counting rule here is strict enough to hurt. A round is one full cycle of work, audit, review, red team, fixes, and approval, and nothing smaller is ever reported as a round. Not a research pass, not a test run, not a logged sub step, not an attempt. The counts in these repos are low and real, and they stay low and real on purpose, because a number the owner cannot trust is worse than no number. Anyone reading a repo in this portfolio can take its closed round count at face value. That trust is the asset the rule protects.

## Measure everything

Judgement improves when the record is complete, and it degrades quietly when the record is trimmed for convenience. The practice captures every signal that can be captured, keeps the detail even when the detail complicates the summary, and publishes numbers with their uncertainty attached. Artificial caps and forced choices between two useful views are refused. If two measures both inform a decision, both are kept and both are shown. When the data cannot settle a question, the honest output is the limitation, stated beside the numbers, never a quieter dataset.

## Verify the real thing

Most verification failures in agent era work are proxy failures. The count matched, the hash matched, the badge was green, and the thing itself was wrong in a way anyone would have seen by looking at it. The rule here is that claims are checked against the artifact itself, in the form its audience will meet it. A page is loaded and read at the size it will be seen. A document is rendered and inspected. A tool is run on real shaped input and its output read. Indirect evidence has its place, and that place is supporting a direct check, never replacing one.

## Work survives the worker

Agents end mid task. Machines restart. Sessions close without ceremony. A practice that loses work when a worker vanishes is a practice that will eventually lose important work at the worst moment. So every lane commits early and pushes often, worktrees and branches are cheap and named, and nothing that matters exists in only one place. The test of the rule is unsentimental. Any worker on any lane can disappear right now, and the work loses nothing but the minutes since the last push.

## The perfection bar

The owner's name is on this work, and names outlast sprints. Anything public gets gone over until it is spotless, researched, audited, reviewed, red teamed, and approved, in as many passes as spotless takes. This is not decoration. In a field flooded with generated output that nobody checked, checked work is the differentiator, and the check is visible in the artifact itself. The bar has one release valve and it is deliberate. The owner may decide, out loud and on the record, that a given piece is good enough for its purpose. What the bar forbids is the silent version, quality slipping because nobody chose to look.

## Agents work, the human judges

The fleet does the work. Agents research, write, test, review each other, and keep the records current, at a scale and a patience no human schedule matches. The human contribution is different in kind rather than in volume. The human sets the bar, decides what the work is for, supplies the access and the facts only a human holds, and makes the calls that carry real world consequence. Confusing the two roles in either direction wastes the scarce resource. A human doing agent work is attention spent at machine prices, and an agent making the human's calls is judgement spent without accountability.

## What this is not

This is not a claim that patience is a vice in the work itself. The reading is slow where reading is the work, and the thinking is slow where the decision deserves it. The impatience is aimed at everything around the work, the queues, the ceremonies, the unverified claims, and the numbers nobody should have trusted. Nor is this a finished doctrine. It is a practice under revision by its own rules, counted in rounds, corrected in public, and judged by whether the tools it produces keep proving themselves useful.
