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
- IP-19a. Amendments, from the Round 4 reviews.
  The TOOL_PHRASES standard is the pin's own:
  each phrase is taken verbatim from that
  tool's own current README, and a phrase the
  sibling's wording has left behind is stale
  even when its meaning still fits. The
  aggregate state pin counts the sibling
  bullets only; this repo's own state line
  never enters it, so a closure that moves
  this repo's own count cannot trip the pin
  that watches the siblings.
- IP-19b. Clarification, from the Round 4
  standards review: the parity suite of IP-19
  is the oracle's tests with the same pin
  refresh applied to staged copies. The frozen
  oracle keeps the prior round's pins as the
  historical record and is never edited; run
  against a refreshed tree it fails exactly
  the refreshed pins, which is the stale-side
  evidence, not parity.
- IP-20. Round 5 refreshes the portfolio record
  again, under the Round 4 discipline (IP-17 to
  IP-19 as amended). Five sibling states moved
  since the Round 4 refresh landed: retroforge,
  wallofwrong, and bluepencil closed their fifth
  rounds, and onrecord, secondtake, and unheard
  already stood at five when Round 4's own pins
  were written against an earlier read. Every
  sibling state, command list, and tool phrase
  is re-measured from the siblings' own landed
  READMEs (API read-back) and shipped CLIs at
  this round's start; nothing is carried from
  Round 4's measurements.
- IP-21. The refresh obeys IP-19a and IP-19b as
  they stand: tool phrases verbatim from the
  siblings' current README text, the aggregate
  count pin scoped to the nine sibling bullets,
  and REAL_COMMANDS re-derived from each
  sibling's current --help surface.
- IP-22. Mutation parity is re-run on the
  refreshed tree (planted defects in the real
  worktree, restored from snapshots, never from
  git checkout), and README records 5 of 100
  only in the closure commit written after this
  round's three reviews, fixes, and owner
  verification.
- IP-23. Amendment, from the Round 5 red team
  (F1, Major). The state pins gain exclusivity.
  A sibling bullet states its state exactly
  once: for each of the nine sibling bullets,
  the matches of the state frame (a number word
  or a digit run, then "of one hundred rounds
  closed") number exactly one, and that one
  match equals the pinned state. A second state
  sentence on a pinned bullet fails the suite
  even when the pinned sentence still stands
  beside it. The mutation set gains the
  dual-state plant, the report's own shape, a
  second sentence claiming Six on secondbill's
  bullet beside its pinned Four, and parity
  proves both suites kill it.
- IP-24. Amendment, from the Round 5 red team
  (F2, Minor). The practice document's
  no-counts scan enforces its stated rule over
  the full set of number words a count can be
  spelled with: zero through twenty, the tens
  words thirty to ninety, and hundred, beside
  the digit forms already barred. A count
  spelled "eleven rounds" fails the claims no
  counts test as a digit count does. The
  mutation set gains the eleven rounds plant.
- IP-25. Amendment, from the Round 5 red team
  (F3, Minor). The README's two statements of
  this repo's own state must agree. One test
  reads the Programme status line's digit count
  and the own bullet's state word and compares
  them as the same number, so the record cannot
  contradict itself about its own state with
  the suite green. This is an agreement
  obligation, not a second literal pin: it
  holds at whatever value a closure sets in
  both places, and the parity set's closure
  simulation still passes both suites under it.
- IP-26. Round 6 refreshes the portfolio
  record to the measured truth, as Rounds 4
  and 5 did: every sibling state re-measured
  from the sibling's own landed README by API
  read-back, every distinctive phrase
  verbatim against the current README, and
  REAL_COMMANDS re-derived from the landed
  sources through the contents API, never
  from the local clones. The measured states
  at this round's opening are: flighttape 14;
  retroforge, wallofwrong, bluepencil, and
  unheard 6; onrecord, secondtake, secondbill,
  and verdictloop 5. The aggregate pin over
  the nine sibling bullets is four sixes and
  four fives beside flighttape's fourteen.
- IP-27. A bullet is its whole block: the
  opening line plus its indented continuation
  lines. State frames are counted over the
  block's whitespace-joined text, so a second
  frame standing on a continuation line, or
  a frame split across continuation lines, is
  seen and held to the exactly-one rule of
  IP-23. This decides the question filed at
  Round 5 (its R1) for block semantics,
  because the guard exists so the record
  cannot contradict itself anywhere in the
  bullet, not only on its first line.
- IP-28. The practice no-counts word set is
  extended by enumeration: dozen, thousand,
  million, billion, and trillion join the
  twenty-nine words of IP-24, thirty-four in
  all, and the clause claims exactly its
  enumeration and no more. This decides the
  question filed at Round 5 (its R2) for
  extension over narrowing.
- IP-29. The state watch extends to
  MANIFESTO: the state frame matcher finds
  no frame there. MANIFESTO states no
  per-tool counts by design, and a bare frame
  appended to it is a contradiction of that
  design whether or not it names a sibling.
  This decides the scope question filed at
  Round 5 (its R3) for extension.
- IP-30. The tools inventory requires
  exactly one bullet per tool: a duplicated
  tool bullet fails the suite. This
  supersedes, for this inventory, the
  last-line-wins reading the Python's dict
  semantics gave it under IP-11, and decides
  the standing question filed at Round 5
  (its F4) with open eyes: the record does
  not carry two bullets for one tool.
- IP-31. The CodeRabbit disposition owed
  from Round 5 discharges here on evidence:
  the Round 5 pull request carries zero
  inline findings on read-back (checked
  2026-11-04 in the rotation-wide late
  sweep), so there is nothing to fix and the
  absence is the disposition. README records
  6 of 100 only in the closure commit
  written after this round's three reviews,
  fixes, and owner verification.
