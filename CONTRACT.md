# impatient-programming Contract, stub

Prose only. This stub records the obligations the setup already knows
it owes. The full Contract waits until the repo is unfrozen.

- IP-1. The repo carries prose only, no code beyond the setup guardrails.
- IP-2. Public copy follows the portfolio copy bar, no em dashes, no en
  dashes, no colons, no semicolons in public copy.
- IP-3. Nothing in the repo positions the work against any other author
  or method until the owner rules on naming.

## Full contract, Round 1 (the freeze lifted by owner ruling,
2026-11-01: no rename, this repo is the owner's own take on
Impatient Programming, and the portfolio's tools are that
take embodied)

- IP-1 (amended). The repo carries prose only. No product
  code; the setup guardrails and the tests that guard the
  documents are the only code. Test, tests/test_documents.py.
- IP-2 (carried). Public copy follows the portfolio copy
  bar. README.md and MANIFESTO.md contain no em dashes, no
  en dashes, no colons, and no semicolons. Test,
  tests/test_documents.py.
- IP-3 (replaced under the 2026-11-01 ruling). The README
  opening states plainly that this repo is David
  Engelmann's own take on Impatient Programming, names him
  as its author, and carries one plain sentence
  distinguishing this take from the method published under
  the same name at impatientprogramming.org. No copy
  presents this take as that method, or that method as
  this take. Test, tests/test_documents.py.
- IP-4. The README states the take's principles as a named
  list under Principles, each principle carrying a short
  statement of the rule as the owner practices it. The
  principles are Speed is the point, Local gates before
  CI, Three reviews or it is not done, Contract before
  code, Count honestly, Measure everything, Verify the
  real thing, Work survives the worker, The perfection
  bar, and Agents work, the human judges. Test,
  tests/test_documents.py.
- IP-5. MANIFESTO.md argues every principle from the
  README at length, why it exists, what failure it
  prevents, and how it is practiced, under a section per
  principle titled with the principle's name, so the two
  documents cannot drift apart. Test,
  tests/test_documents.py.
- IP-6. The README's tools section names the ten repos of
  this line with the job each one does in one line and an
  honest state for each, the rounds it has closed as its
  own README states them, and says the same method runs
  across the rest of the portfolio. No sentence claims a
  tool does more than its own repo claims. Test,
  tests/test_documents.py.

Amendments recorded in the Round 1 repair. IP-2, the copy
bar bars the dash, colon, and semicolon characters in any
encoding, not only in their ASCII and standard forms.
IP-6, a tool's one line job is stated in the words of its
own repo, checked line by line against that repo's README.

## Round 2. The practice (IP-7 to IP-10)

- IP-7. PRACTICE.md states how each of the ten principles
  from IP-4 is actually worked, one section per principle
  under a heading that carries the principle's exact name,
  in the same order as the README list. A section is
  practice, not restatement: it names the concrete moves,
  the artifacts, and the checks a reader could watch
  happening. No principle is added, renamed, or dropped.
  Test, tests/test_practice.py.
- IP-8. Every claim PRACTICE.md makes about the line's own
  programme is checkable against the public record it
  names: a round count it states matches the README of
  the repo it names at the time of writing, a mechanism
  it describes exists in the repo it is attributed to.
  The document states its own as-of date, and no claim in
  it presents a plan as a finished fact. Test,
  tests/test_practice.py, which pins the as-of date's
  presence and the absence of the line's known inflated
  phrasings.
- IP-9. The copy bar of IP-2 covers PRACTICE.md in full,
  every encoding, and the tests are extended to scan it.
  Test, tests/test_practice.py.
- IP-10. Round 2 is still prose and its guardrails: no
  product code, and nothing in PRACTICE.md instructs a
  reader to run a command that does not exist in the repo
  it is attributed to. Test, tests/test_practice.py.

## Round 3. The guard suite in Rust (IP-11 to IP-16)

Round 3 re-expresses the guard suite as cargo tests.
This repo is the line's argued misfit under the Rust
ruling: the product is prose, there is no Python
source to port, and a library API would be an
invention. The port is therefore the thinnest in
the line: the same checks, in Rust, over the same
documents, which do not change. The Python suite at
e031741 is the behavioral spec.

- IP-11. Exact reproduction of the guard semantics:
  every one of the 15 Python tests becomes a cargo
  test that can fail for the same real reason, with
  the same literals, thresholds, and counts. Tests,
  tests/test_documents.rs and tests/test_practice.rs.
- IP-12. The crate is a library whose only item is
  the checker the tests share (reading the
  documents, the folded copy bar scan, the tree
  walk), plus the two integration test files. No
  binary, and no public API beyond the checker.
  Tests, the whole suite.
- IP-13. IP-1 amended for the port: the guard's own
  home is src, tests, and scripts, and target is
  its build output; the tree walk bars product code
  anywhere else in the repo, with those skipped
  parts named in the test. The documents remain
  the product. Test, tests/test_documents.rs.
- IP-14. The folded check runs Unicode NFKC through
  the unicode-normalization crate. Its coverage is
  verified against the interpreter's unicodedata
  over the whole codepoint space for folds landing
  in the barred set, and any difference is a named
  deviation with the codepoints enumerated, never
  a silent narrowing of IP-2. Test,
  tests/test_documents.rs, and the round dossier.
- IP-15. The barred set stands defined independently
  in each test file, as in the Python, and the
  same-bar test compares the two definitions as
  sets so a drift in either fails. The Python's
  small regular expressions become hand matchers
  with the same match semantics. Test,
  tests/test_practice.rs.
- IP-16. The prose does not change in this round
  except the closure count in the README's own
  status. scripts/codepoint_scan.py stays as the
  typography instrument. The port report stands in
  the round dossier, restating the misfit argument,
  the line share, and the remainder register (the
  three documents and the scan, by design).
- IP-16a (amendment, recorded at the round's PR).
  The README Status section's local gate line names
  the commands the guard actually runs. Keeping
  that one line true as the suite's runner changes
  is apparatus maintenance, not a prose change, and
  IP-16 does not bar it. The line was corrected in
  this round from the retired pytest command to the
  cargo gate CI runs.
- IP-17. Round 4 refreshes the portfolio record
  against the siblings as they stand at refresh
  time. Each sibling's state is read from its
  own landed README and each tool's command
  list from its shipped CLI, never from this
  repo's memory of them; the README tools
  section and the test pins (states, phrases,
  REAL_COMMANDS) move to the measured truth in
  one pass.
- IP-18. The refresh changes record prose and
  pins only. The guard's rule set does not
  change, the ten principles do not change, and
  the practice document's bars (no round counts
  claimed there, no commands named that do not
  exist) bind the refreshed text exactly as
  they bound the original.
- IP-19. The round's verification is the full
  local gate plus mutation parity in the real
  worktree: planted defects in the refreshed
  states and command lists fail the Rust guard
  and the Python oracle's suite alike.
