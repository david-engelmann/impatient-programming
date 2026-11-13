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
- IP-32. Amendment, from the Round 6 red team
  (Minor 1). The tools inventory is exact:
  the multiset of tool bullet names the
  section carries, each name normalized by
  lowercasing and stripping one trailing
  period, equals the ten pinned tool names,
  each exactly once. A lookalike duplicate,
  a case variant, or an extra bullet for a
  name no pin knows all fail the suite. The
  content pins keep their exact literal
  openers; this clause governs the section's
  inventory only.
- IP-33. Amendment, from the Round 6 red team
  (Minor 2). The IP-29 watch reads the
  MANIFESTO's whitespace-joined text, the
  same collapse the bullet watch uses under
  IP-27, so a state frame split across a
  newline in the MANIFESTO is seen whole.
- IP-34. Amendment, from the Round 6 red team
  (Minor 3), a disposition and no code. The
  frame and claims matchers follow the
  interpreter's whitespace definition
  exactly. Zero-width characters are outside
  the matchers' alphabet, and a state
  sentence glued by a zero-width character
  is not a frame in the source semantics or
  in the record. This stands as the boundary
  of text matching for both suites.
- IP-35. Amendment, from the Round 6 red team
  (Minor 4). The practice claims matcher bars
  a set word appearing within the two tokens
  preceding the word rounds, and the set
  gains the plural scale forms dozens,
  hundreds, thousands, millions, billions,
  and trillions, forty words in all. The
  clause and the test comment claim exactly
  this construction and no more. A
  hyphenated singular adjective use, as in
  twenty-one-round cycles, stands outside
  it by construction.
- IP-36. The CodeRabbit disposition owed
  from Round 6 discharges here on evidence,
  and the standing expectation was wrong.
  Read back on 2026-11-07 through the list
  endpoints, the Round 6 pull request
  carries three inline findings, one review
  (COMMENTED, submitted 2026-11-05T16:40:58Z,
  reporting three actionable comments), and
  a summary shell that completed with a
  walkthrough. The pull request merged at
  2026-11-05T16:41:15Z, seventeen seconds
  after the review submitted, so the
  findings stood unread at the merge, and
  the bot's later reply records only that
  the pull request is closed. All three
  findings are Minor, all three sit in the
  guard's semantics, and each is valid on
  verification against the code. They
  repair under IP-37, IP-38, and IP-39,
  with the factual replies owed on the
  thread at landing.
- IP-37. Amendment, from the Round 6
  standards review (its Note 1) and the
  first of the IP-36 findings, one escape
  seen by two reviews. A tool bullet's
  block does not end at a blank line when
  the next content line is indented. Blank
  lines inside a bullet are skipped, and
  the block continues at the indented
  paragraph, so a state frame standing
  there is read whole, like any frame on a
  continuation line. A blank line followed
  by a line that is not indented still
  ends the block. This is the document's
  own list semantics, and it retires the
  reading under which a frame could stand
  in a bullet unseen. Both suites carry
  the same scan.
- IP-38. Amendment, from the second of
  the IP-36 findings. The tools inventory
  reads every bullet the section carries,
  under any of the three list markers this
  markdown can open a bullet with, dash,
  asterisk, or plus: the inventory scan
  strips any of the three before the bold
  name, so an extra bullet opened with an
  asterisk or a plus no longer escapes the
  exact inventory of IP-32. The record's
  own bullets keep the dash, and a pinned
  tool's bullet under another marker
  already fails loudly at the content
  pins, so this clause governs the
  inventory scan only, as IP-32 governed
  it.
- IP-39. Amendment, from the third of the
  IP-36 findings. The claims tokenizer
  reads the zero-width characters as
  nothing: U+200B, U+200C, U+200D, U+2060,
  and U+FEFF are neither token content nor
  token separators, so a word glued by one
  of them stays one token and manufactures
  no rounds token the text does not
  carry. This makes good the boundary
  IP-34 stated for both suites. Both
  suites drop the same five characters in
  the same place.
- IP-40. The portfolio record refreshes
  from the landed READMEs, read back
  through the contents API on 2026-11-07:
  flighttape 15; unheard, onrecord,
  tetlock, and secondtake 7; retroforge,
  bluepencil, muda, and touchstone 6. The
  state pins and the aggregate counts move
  to exactly these states, Fifteen once,
  Seven four times, and Six four times
  over the nine siblings.
- IP-41. The record names the tools as
  they are. The repo this record has
  called secondbill is muda on GitHub,
  renamed on 2026-11-06 with its id and
  history intact, and the bullet, the name
  pins, the phrase pin, and the practice
  suite's command table take the name muda
  in this round, on the precedent of the
  tetlock rename scrub. Dated clauses that
  name secondbill stand as written; they
  record the states of their rounds.
- IP-42. The README comes to the fleet
  README standard, adapted where this
  repo's own law binds harder. The first
  screen carries the title, the lede, one
  short paragraph, and one block of real
  captured output, in that order. The lede
  is the repo's GitHub description
  verbatim, and the captured output is the
  codepoint scan's one line, the only
  output this repo produces that its copy
  bar lets it quote. The README carries no
  badge: the bar of IP-2 bars the colon in
  these documents in any encoding, code
  blocks included, and every badge address
  carries one in its scheme, so the badge
  set is empty by the repo's own founding
  test and the captured output stands in
  its place. Install, Quickstart, Usage,
  Status, and License follow. Install and
  Quickstart are the clone and the guard
  run, stated plainly as what they are for
  a repo of documents; no block quotes
  cargo's output, whose result lines carry
  colons, and the test count is stated in
  prose and traces to this round's gate
  log. Usage is the reading guide to the
  three documents and the guard, the
  status stands in one section, and the
  license line links the LICENSE file.
  The principles, the tools record, and
  the wider portfolio sentence stand. The
  copy repair rulings of 2026-11-05 stand
  untouched, no name story and no
  practitioner framing, and the test that
  pins their absence keeps passing.
- IP-43. The package metadata carries the
  same one line. Cargo.toml gains the
  description, verbatim the lede and the
  GitHub description, so the one line
  stands identical in all three homes at
  once.
- IP-44. This round's guard amendments
  are proven by mutation parity: the
  Python transcription of the suite,
  amended identically clause by clause,
  and the Rust suite agree on every
  planted defect and every witness,
  FAILURES 0, plants laid in place and
  restored from snapshots, never from a
  checkout. The gate is the standing four
  commands on the final tree. README
  records 7 of 100 only in the closure
  commit written after this round's three
  reviews, fixes, and owner verification.
- IP-45. Amendment, from the Round 7
  red team (its Minor 1). The bullet
  inventory reads the ordered list
  markers too. A bullet can open with
  one to nine digits and a period or
  a closing parenthesis, and an extra
  bullet in that form, carrying a
  name no pin holds and a state
  frame no block reads, escaped the
  inventory of IP-32 and the frame
  pins of IP-23 alike, IP-38's three
  markers being only the unordered
  forms. The inventory scan strips
  the ordered marker before the bold
  name, so the exact inventory runs
  over every bullet form this
  markdown can open. Both suites
  carry the same scan, and the
  record's own bullets keep the
  dash.
- IP-46. Amendment, from the Round 7
  red team (its Minor 2). The
  name-story test reads its markers
  over the lowered text. The markers
  stood in their recorded case, so a
  marker at a sentence head, wearing
  the capital the sentence gives
  it, escaped the absence the test
  pins, and a sentence head is the
  place a writer would most
  naturally put one. Both suites
  lower the document and match the
  markers in lowercase, the reading
  the practice suite already gives
  its claims. The recorded markers
  fail as before, and their capital
  forms fail with them.
- IP-47. Amendment, from the Round 7
  standards review (its Minor). The
  README's Install prose names what
  its copy command needs. The block
  copies the repo with the GitHub
  CLI, the one clone form the copy
  bar lets the README print, and
  the prose said only that a copy
  takes one command, so a reader
  holding git and a Rust toolchain,
  the prerequisites the section
  implies, met a command their
  machine could not run. The prose
  names the GitHub CLI beside the
  Rust toolchain, and the one
  command claim goes with the
  wording that hid the dependency.
- IP-48. The refresh of IP-40
  completes against the landed
  state. The read-backs behind
  IP-40 were fetched before
  retroforge's seventh round
  merged on 2026-11-07, and
  retroforge's README, read back
  through the contents API after
  that merge for this clause,
  reports seven of one hundred
  rounds closed. The retroforge
  pin moves from Six to Seven in
  the README and in both suites,
  and the aggregate counts move
  with it, Seven five times and
  Six three times over the nine
  siblings. Every other state of
  IP-40 stands as read.
- IP-49. Three CodeRabbit findings
  on the Round 7 pull request,
  posted before the merge and
  each valid, repaired on the
  branch before it lands. The
  first is IP-45's own hole: the
  ordered marker's rest was read
  only after exactly one space,
  while the markdown lets a list
  marker stand one to four
  spaces before its content, so
  an ordered bullet spaced two
  wide escaped the inventory the
  repair had just built. The
  helper in both suites takes
  one to four spaces now, and
  five or more still open no
  bullet here, as they open a
  code block in the markdown
  itself. The second corrects
  the retroforge bullet: it said
  the tool opens one pull
  request, and retroforge opens
  nothing, it writes one
  proposal file for a human to
  review and apply or not. The
  bullet and the phrase pin in
  both suites say proposal file
  now, the words retroforge's
  own README uses for its
  product. The third qualifies
  the Quickstart's drift claim:
  the suite was said to fail on
  any claim that has drifted,
  and what it checks is this
  repo's own pins, which nothing
  compares with the sibling
  repos, so a sibling can change
  while the suite still passes
  and the change surfaces only
  when a refresh reads the
  siblings back and re-pins
  their states. The sentence
  says that now.
- IP-50. The Round 7 pull request is
  read back in full in this round.
  Seven reviews, nine review
  comments, and four issue
  comments stand on it, and every
  body is read to its end, the
  follow-up shells and the bot's
  acknowledgment replies included.
  IP-49's three findings stand
  disposed there and answered.
  Two further items from the same
  reviewer stand disposed nowhere,
  and the landed comment on that
  pull request counted only the
  three. This round carries both,
  in IP-51 and IP-52.
- IP-51. The Docstring Coverage
  check on that pull request is
  dismissed, on the ground the
  fleet's record states for this
  same check wherever it fires.
  It names no function. The repo
  carries no docstring
  configuration or threshold of
  its own, and the check applies
  the review tool's default
  profile, whose threshold this
  repo never adopted. The summary
  shell's own detail counts seven
  functions across two files and
  names none of them, so no named
  gap stands to repair, and both
  suites carry the comments their
  clauses need, as the reviews of
  this record have judged them
  round by round.
- IP-52. The Install prose names
  Python 3. A further comment in
  the same review's body holds
  that a reader of the Install
  section is told the guard takes
  a Rust toolchain and is never
  told the scan takes Python 3,
  though the gate this repo
  stands on runs that scan under
  python3. Read back against the
  landed README, the word Python
  stands nowhere in it while the
  python3 command stands three
  times. The comment is valid.
  The Install prose names Python
  3 beside the Rust toolchain, as
  IP-47 named the GitHub CLI
  beside it, for the same reason.
- IP-53. The tools record is
  re-pinned to the landed state,
  measured in this round from
  each sibling's own README read
  back through the contents API,
  never from a local clone and
  never from memory. flighttape
  stands at Sixteen of one
  hundred rounds closed. unheard,
  tetlock, and secondtake stand
  at Nine. retroforge, bluepencil,
  onrecord, muda, and touchstone
  stand at Eight. The aggregate
  counts over the nine siblings
  move with them, Sixteen once,
  Nine three times, and Eight
  five times. Four phrase pins
  move to words standing in the
  siblings' current READMEs, the
  old words having left those
  pages. flighttape's phrase is
  lands as one flight tape,
  bluepencil's is a readable
  list of edits, muda's is
  itemized statement for one
  stated calendar month, and
  secondtake's is ending in a
  keep, tune, or revert
  proposal. The other five
  phrases stand as pinned. The
  command tables of the practice
  suite are re-derived from each
  sibling's landed help surface
  and dispatch, and all nine
  stand exactly as pinned, so no
  command pin moves. The record
  keeps its numbers rather than
  dropping them, because every
  sibling still states its count
  in the same status form this
  record's frame reads, and a
  record that can stay honest in
  its standing shape stays in it.
- IP-54. The guard's surfaces are
  measured fresh at the shipped
  guard, on a scratch copy of the
  landed tree. Each barred
  character in raw form, and each
  codepoint that folds to a
  barred one under NFKC without
  being barred itself, the Greek
  question mark, the double
  colon, and the presentation
  and small forms of the set,
  stands planted in each of the
  three documents at the start,
  in the middle, at the end, and
  inside a word, and the copy-bar
  test guarding the planted
  document convicts all 192
  plantings. The untouched tree
  passes the whole suite, and
  every file stands byte
  identical after the run, the
  guard reading and never
  writing. The boundary stands
  as measured. The cargo copy
  bar reads the three documents
  and nothing else. The
  codepoint scan reads the whole
  tree by its suffixes and bars
  the two dashes only, convicting
  a dash planted in CONTRACT.md,
  in a plain text file, and in
  the source, and passing a colon
  planted in CONTRACT.md, which
  the cargo suite passes too,
  the contract standing inside
  the scan's sweep and outside
  the copy bar's three documents.
  No conviction failed, so no
  repair rides this round, and
  the battery's numbers are this
  round's evidence for the
  reviews to re-run.
- IP-55. This round's amendments
  are proven by mutation parity:
  the Python transcription of the
  suite, amended identically
  clause by clause, and the Rust
  suite agree on every planted
  defect and every witness,
  FAILURES 0, plants laid in
  place and restored from
  snapshots, never from a
  checkout. The gate is the
  standing four commands on the
  final tree. README records 8 of
  100 only in the closure commit
  written after this round's
  three reviews, fixes, and owner
  verification.
- IP-56. The Round 8 pull request is
  read back in full in this round.
  One review, no review comments,
  and five issue comments stand on
  PR 13, and every body is read to
  its end. The one review
  (5827759324, the review bot,
  COMMENTED, on the reviewed head
  a3955a9d) carries no actionable
  comments and no inline findings.
  Its walkthrough shell (issue
  comment 6743127158) reports Merge
  Risk Minimal and pre-merge checks
  at 4 passed, 1 failed. The one
  failure is Docstring Coverage,
  reported by the shell at 33.33%
  against an 80.00% threshold, 3
  functions across 1 file and 2
  skipped as unsupported, naming no
  function. The owner's disposition
  stands at 6743350693, dismissing
  that check on the IP-51 ground and
  answering the shell's merge-risk
  note on the IP-55 standing shape,
  and the owner's landing comment
  stands last at 6745285284,
  recording the three reviews and
  the merge at 6373a87a. Between the
  trigger (6743128706) and the
  disposition stands the bot's
  auto-reply at 6743325585, action
  not completed because the last
  commit stood already reviewed, the
  standing incremental
  acknowledgment, which carries no
  finding and owes no disposition.
  Nothing stands after the landing
  comment, and the census closes
  with nothing undisposed. One
  discrepancy in the record stands
  recorded rather than smoothed
  over. IP-55 describes the Round 8
  closure commit as written after
  that round's three reviews, fixes,
  and owner verification, while the
  sequence the pull request records
  is the closure commit riding the
  branch first, the owner's three
  reviews following it, and the
  merge last, the same tension the
  shell's merge-risk note raised and
  the owner's disposition answered
  on the standing shape. The two
  dispositions themselves stand
  unfalsified, and this round states
  its own sequence as it runs, at
  IP-60.
- IP-57. The guard measured fresh at
  the shipped tree, the Round 8
  merge 6373a87a, every blob of it
  verified against the remote tree
  before a test ran. The full suite
  passes end to end, 20 tests, 13
  over the documents and 7 over the
  practice, with clippy clean over
  all targets under -D warnings and
  fmt clean. The prose surfaces the
  suite covers are scanned two ways.
  The suite's own copy-bar tests
  convict nothing on the clean tree,
  and an independent scan with the
  interpreter counts every barred
  character at zero in each of
  README.md, MANIFESTO.md, and
  PRACTICE.md, raw and NFKC-folded,
  all nine characters of the set in
  each form. The fold set is
  re-derived, not merely recounted.
  The interpreter the pin names,
  CPython 3.12.3 with unicodedata
  15.0.0, folds the whole codepoint
  space and returns exactly the
  pinned sixteen, U+003A, U+003B,
  U+037E, U+2013, U+2014, U+2015,
  U+2A74, U+FE13, U+FE14, U+FE31,
  U+FE32, U+FE54, U+FE55, U+FE58,
  U+FF1A, and U+FF1B, seven of them
  folding to a barred character
  without being barred themselves,
  and the guard's own fold-set test
  derives the same sixteen through
  the normalization crate and
  passes. The repository codepoint
  scan over the whole tree reports
  U+2014=0 and U+2013=0 and exits 0.
  No conviction failed anywhere in
  the measurement, so no repair
  rides this item.
- IP-58. The Install block re-run as
  printed. The block's commands take
  a copy with the GitHub CLI and run
  the guard in it, and the taking is
  the one step this lane's
  environment cannot perform as
  printed, because the GitHub CLI on
  this box holds no login and
  refuses the clone, exiting 4 with
  its instruction to log in. The
  copy was therefore taken through
  the API, and it is the same copy
  the block fetches, all 14 blobs of
  the remote tree at 6373a87a
  verified one by one against the
  tree listing. In that clean copy
  the guard ran under exactly the
  prerequisites the block names as
  repaired by IP-52, a Rust
  toolchain and Python 3 for the
  scan. The scan ran under python3,
  version 3.12.3, and printed the
  summary line the README
  reproduces, codepoint scan
  U+2014=0 U+2013=0, exit 0, and
  cargo test ran the full suite
  there, 20 passed. The block as
  printed needs nothing it does not
  name for the guard, whose
  prerequisites stand exactly as
  named, and the Python 3 naming
  IP-52 repaired still holds as
  printed. For the copy the block
  carries one prerequisite it names
  only as the tool. The clone
  command presumes a GitHub CLI
  holding a login, and a reader
  whose CLI holds none is refused
  exactly as this box was. That
  unnamed prerequisite stands
  recorded here for the round that
  next amends the Install prose.
- IP-59. The practice exercised at
  the volume its own text
  contemplates, the line's
  realistic-data rule kept with
  synthetic content only. The
  guard's checks are whole-document
  checks over a book's companion
  documents, so the documents were
  carried to book scale with their
  own text and the outcomes compared
  exit for exit against the suite's
  expectations on the true
  documents. The practice document
  repeated 10 times (82,920 bytes)
  and 100 times (829,200 bytes), an
  input shape its own section
  semantics name, since a repeated
  section keeps its first position
  and takes the later body, passes
  the full practice suite at both
  sizes. On the 100-times document,
  a count claim of the shape the
  claims test bars, the words eleven
  rounds of appended at the very
  end, is convicted by it, and an em
  dash as the document's final
  character is convicted by the
  practice copy-bar test. The three
  documents padded past a million
  bytes each with their own repeated
  text, README.md at 1,002,327
  bytes, MANIFESTO.md at 1,000,008
  bytes, and PRACTICE.md at
  1,003,332 bytes, pass both
  copy-bar tests clean, and the full
  practice suite passes on the
  padded practice document. Each of
  the sixteen fold-set codepoints
  planted singly at character offset
  900,000 in each padded document is
  convicted by its guarding copy-bar
  test, 48 of 48 plantings, and an
  em dash appended to the padded
  README is convicted by the
  repository scan, which exits 1
  reporting U+2014=1. Every
  exercised tree stands
  byte-identical after its runs, the
  guard reading and never writing at
  volume as on the true tree. No
  conviction failed, so no repair
  rides this item.
- IP-60. The round closes on the
  standing gate and the standing
  record. The gate on the final tree
  runs the repository codepoint
  scan, cargo fmt --check, cargo
  clippy --all-targets -- -D
  warnings, and cargo test, and
  stands at 20 tests passed with the
  scan at zero on both barred
  dashes. README records 9 of 100
  only in the closure commit, the
  standing shape under IP-55 as
  under IP-44, with this round's
  three reviews the owner's step
  after this branch and the landing
  the owner's act alone, and the
  Status line carries the contract
  range to IP-60 in the same commit.
  The storefront's third item is
  reported from evidence at the
  head, not from habit. The GitHub
  description, the README lede, and
  the Cargo.toml description are one
  line in all three homes,
  byte-identical at the base head
  6373a87a and untouched by this
  round, and the description
  question stands answered outright
  on this repo under IP-43. The
  round's lane report was read
  whole by this round's spec and
  standards reviewers before a
  box restart wiped the lane's
  scratch record.
  The gate and battery figures
  stand corroborated in this
  round's review record; the gate
  log, battery log, and derivation
  script themselves were not
  preserved, and no off-box copy
  of them was made.

## Round 10. The record read back whole again, the
guard at its joints, and the Install question carried
open (IP-61 to IP-65)

- IP-61. The Round 9 pull request is
  read back in full in this round.
  One review, three review comments,
  and four issue comments stand on
  PR 14, and every body is read to
  its end. The one review
  (5848588770, the review bot,
  COMMENTED, on the slice head
  a3b88ea4) posts three actionable
  Minors, all on CONTRACT.md. The
  first (4529926727) asks that the
  closure-timing discrepancy stand
  recorded in IP-56, and it is valid.
  The step 0 amendment 4e958f4d
  records it, and the comment itself
  carries the bot's addressed note
  naming that commit. The second
  (4529926747) names the clone
  authentication requirement. Its
  first arm is valid and repaired in
  the same amendment, which records
  at IP-58 that the clone command
  presumes a GitHub CLI holding a
  login. Its second arm, amending
  the Install prose itself or
  offering a clone method that needs
  no login, stands undischarged in
  Round 9, disclosed in IP-58 and in
  the pull request body and routed
  to the round that next amends the
  prose. This round takes it up at
  IP-63. The third (4529926757) asks
  that the closed count stay at
  eight while the reviews run, and
  it is dismissed on the settled
  closure shape, the count riding
  the branch and reaching main only
  through the owner's merge after
  the three reviews, the same note
  the PR 13 record answered at
  disposition 6743350693. The four
  issue comments are the owner's
  review trigger (6777769547), the
  bot's auto-reply (6778103763,
  action not completed because the
  last commit stood already
  reviewed, the standing incremental
  acknowledgment, which carries no
  finding and owes no disposition),
  the walkthrough shell (6777764065),
  and the owner's landing comment
  (6779281270), which stands last.
  The shell stands in a later form
  than the Round 9 record read. The
  Round 9 lane read a form of 11,169
  characters, and the shell as it
  stands at this round's read is
  11,632 characters, edited after
  the final repair head landed. In
  its final form the walkthrough
  covers the final head dab35e06,
  the recent review (run
  718e477b-48bb-493c-89b2-db9ae4049870,
  over the range a3b88ea4 to
  dab35e06, one file selected and
  one file skipped as similar)
  reports no actionable comments,
  Merge Risk stands Low up to dab35
  with its note naming the login
  follow-up and the count wording,
  and the pre-merge checks counter
  stands at 4 passed, the rows
  Description Check, Title check,
  Linked Issues check, and Out of
  Scope Changes check, all passed,
  with no Docstring row standing in
  the form and the word itself
  nowhere in it. In its first form
  (the review 5848588770, run
  6154f0fd-36fb-419c-9443-213245034416,
  over the base 6373a87a to a3b88ea4,
  two files selected) the review
  carried the three actionable
  Minors above; the walkthrough
  shell is a separate comment and
  carried none of them. The Round 9
  statements about the shell were
  true of the form that lane read,
  and no disposition turns on the
  difference. Five commits stand on
  the pull request, the slice
  32ebacd7, the closure a3b88ea4,
  the step 0 amendment 4e958f4d, and
  the two repairs eb760d80 and
  dab35e06, and the merge de696837
  carries tree f4fe9c70, equal to
  the final head's tree. Nothing
  stands after the landing comment.
  The census closes with the one
  routed arm standing open by name
  at IP-63, and no Round 9
  disposition is falsified by the
  record.
- IP-62. The guard measured fresh at
  the shipped tree, the Round 9
  merge de696837, every blob of it
  verified against the remote tree
  before a test ran. The Python
  oracle stands verified first. The
  tree at e0317417681f7e5a16d7c83517ac2f8020f20314
  (tree 81cac2bb) carries ten blobs,
  and the local extraction of it
  matches all ten blob for blob.
  The full suite passes end to end,
  20 tests, 13 over the documents
  and 7 over the practice, with
  clippy clean over all targets
  under -D warnings and fmt clean.
  An independent scan with the
  interpreter counts every barred
  character at zero in each of
  README.md, MANIFESTO.md, and
  PRACTICE.md, raw and NFKC-folded,
  all nine characters of the set in
  each form. The fold set is
  re-derived, not merely recounted.
  CPython 3.12.3 with unicodedata
  15.0.0 folds the whole codepoint
  space and returns exactly the
  pinned sixteen, U+003A, U+003B,
  U+037E, U+2013, U+2014, U+2015,
  U+2A74, U+FE13, U+FE14, U+FE31,
  U+FE32, U+FE54, U+FE55, U+FE58,
  U+FF1A, and U+FF1B, seven of them
  folding to a barred character
  without being barred themselves,
  and the guard's own fold-set test
  derives the same sixteen through
  the normalization crate and
  passes. The repository codepoint
  scan over the whole tree reports
  U+2014=0 and U+2013=0 and exits 0.
  The joints are probed singly.
  Each of the sixteen codepoints
  stands planted inside a word past
  the middle of each of the three
  documents, 48 cells, and the
  copy-bar test guarding the planted
  document convicts every cell, and
  the interpreter's own recomputation
  of the bar from the oracle's
  definition convicts every cell,
  with no disagreement between the
  two arms on any cell. The near
  codepoints are decided at their
  exact codepoints, not bracketed
  from neighbors. The neighbor set,
  every codepoint within two of a
  fold-set member and not itself in
  the set, counts 34 codepoints,
  U+0038, U+0039, U+003C, U+003D,
  U+037C, U+037D, U+037F, U+0380,
  U+2011, U+2012, U+2016, U+2017,
  U+2A72, U+2A73, U+2A75, U+2A76,
  U+FE11, U+FE12, U+FE15, U+FE16,
  U+FE2F, U+FE30, U+FE33, U+FE34,
  U+FE52, U+FE53, U+FE56, U+FE57,
  U+FE59, U+FE5A, U+FF18, U+FF19,
  U+FF1C, and U+FF1D. Each one is
  computed on its own and folds to
  no barred character, and each one
  stands planted inside a word in
  README.md and in PRACTICE.md, 68
  cells, and neither arm convicts
  any cell. The empty document is
  decided as its own shape. An
  emptied README.md and an emptied
  PRACTICE.md pass their copy-bar
  tests on both arms, nothing
  standing in them to bar, while
  the full practice suite run over
  the emptied practice document
  fails, three of its seven tests
  failing, the section-order test,
  the section-substance test, and
  the as-of-date and claims test,
  while its copy-bar test passes. No
  conviction failed anywhere in the
  measurement, so no repair rides
  this item.
- IP-63. The routed Install question
  is read back whole and carried
  open. The question stands in
  three places in the Round 9
  record. IP-58 closes by recording
  that the clone command presumes a
  GitHub CLI holding a login, for
  the round that next amends the
  Install prose. The finding that
  raised it (4529926747) offers two
  arms, that the prose document the
  authentication or that it offer a
  clone method needing no login.
  The Round 9 standards review
  records the second arm as not
  discharged and routed onward, a
  disclosed and reasoned deferral.
  No verbatim sentence for the
  README stands anywhere in that
  record, in the contract, in the
  pull request body, in the review
  reports, or in the landing
  comment. The record names the
  fact the prose must cure and the
  two arms that could cure it, and
  it takes neither. This round
  therefore leaves the Install
  prose untouched, word for word,
  and the question stands open by
  name. What is missing is stated
  exactly. A choice between the two
  arms is missing, and under the
  chosen arm the exact sentence the
  Install prose is to carry is
  missing, and neither is this
  round's to invent. The choice
  between the two arms, and under
  the chosen arm the exact sentence
  the Install prose is to carry,
  are the repository owner's to
  supply, and the question closes
  on his word and not before. The
  underlying fact is re-verified
  fresh in this round. The GitHub CLI on this box
  holds no login, and the clone
  command as printed refuses at
  exit 4 with its instruction to
  log in, reproduced in this round
  as in Round 9. The guard's own
  prerequisites stand exactly as
  named, and the Python 3 naming
  IP-52 repaired still holds as
  printed.
- IP-64. The realistic pass, the
  line's realistic-data rule kept
  with synthetic content only. The
  shapes are stated from the
  material itself. The three
  documents padded past a million
  bytes each with their own
  repeated text, README.md at
  1,002,204 bytes in 123 whole
  copies, MANIFESTO.md at 1,000,008
  bytes in 129 whole copies, and
  PRACTICE.md at 1,003,332 bytes in
  121 whole copies, pass their
  copy-bar tests clean on both
  arms. Each of the sixteen
  fold-set codepoints planted
  singly at character offset
  900,000 in each padded document
  is convicted by its guarding
  copy-bar test, 48 of 48
  plantings, and the interpreter's
  arm convicts every one. The
  practice document at 100 whole
  copies (829,200 bytes) carries a
  count claim of the barred shape,
  the words eleven rounds appended
  at the very end, convicted by the
  claims test, and an em dash as
  its final character, convicted by
  the practice copy-bar test. The
  padded README with one em dash
  appended is convicted by the
  repository scan, which exits 1
  reporting U+2014=1. The scan
  surface is exercised over many
  documents. A staged tree carrying
  the three true documents and 60
  synthetic method notes, each note
  twelve repetitions of a short
  method passage, passes the scan
  at exit 0, and the same tree with
  one em dash standing in the 47th
  note is convicted by the scan at
  exit 1 reporting U+2014=1. A
  mixed practice document, 90
  paragraphs drawn in rotation from
  the three documents, passes the
  practice copy-bar test on both
  arms. The claim for that cell is
  the copy-bar verdict only. The
  section pins judge the true
  document's shape and are not
  claimed over mixed material. A
  boundary document built from a
  passage carrying each of the 34
  near codepoints of IP-62 beside
  plain punctuation, repeated eight
  times at 22,672 bytes,
  passes the copy-bar tests as
  README.md and as PRACTICE.md on
  both arms, and the same document
  with one U+FE55 planted inside a
  word is convicted on both arms in
  both placings. Across IP-62 and
  this item the two arms agree on
  every copy-bar cell, 175 cells,
  with no disagreement. Every
  exercised tree stands
  byte-identical after its runs,
  the guard reading and never
  writing at volume as on the true
  tree. No conviction failed, so
  no repair rides this item.
- IP-65. The round closes on the
  standing gate and the standing
  record. The gate on the final
  tree runs the repository
  codepoint scan, cargo fmt
  --check, cargo clippy
  --all-targets -- -D warnings, and
  cargo test, and stands at 20
  tests passed with the scan at
  zero on both barred dashes.
  README records 10 of 100 only in
  the closure commit, with this
  round's three reviews the owner's
  step after this branch and the
  landing the owner's act alone,
  and the Status line carries the
  contract range to IP-65 in the
  same commit. The storefront's
  third item is reported from
  evidence at the head, not from
  habit. The GitHub description,
  the README lede, and the
  Cargo.toml description are one
  line in all three homes,
  byte-identical at the base head
  de696837 and untouched by this
  round, and the description
  question stands answered outright
  on this repo under IP-43. The
  Install prose stands untouched
  under IP-63, its question carried
  open by name. The lane staged its
  trees and scratch outside the
  workspace in this round, the
  workspace file-count cap refusing
  new files on this box, and the
  gate figures and battery figures
  recorded in IP-62 and IP-64 stand
  in those items themselves.
- IP-66. The repair record for this round's
  reviews. The spec fidelity review returned
  NOT PASS with one Minor and one Nit; the
  standards review NOT APPROVE with one
  Minor; the red team APPROVE with no
  findings and two notes. The spec Minor
  stood against the census in IP-64, whose
  stated parts sum to 175 copy-bar cells and
  not the 174 the item and the pull request
  body carried (IP-62 states 118, the 48
  fold plantings, the 68 neighbor plantings,
  and the 2 emptied documents; IP-64 states
  57, the 3 padded documents clean, the 48
  padded plantings, the em dash cell at 100
  copies, the mixed document, and the
  boundary document in its 2 clean and 2
  planted placings; and 175 with the
  battery's 6 entries that are not copy-bar
  cells closes on the battery total of 181
  the body records). The figure stands
  corrected to 175 in IP-64 and in the
  body's two repetitions by the owner's
  body edit. A second enumeration, excluding
  the em dash cell from the total, was
  recorded on the standards axis and is
  noted here as the dissenting count; the
  battery total decides for 175, since the
  dash cell stands inside the 181 and
  outside the six named non-copy-bar
  entries. The spec Nit stood against one
  IP-61 sentence that described the Round 9
  walkthrough shell's first form as
  carrying the review's three Minors; the
  sentence stands corrected in IP-61, the
  Minors the review's and the shell a
  separate comment that carried none of
  them. The standards Minor stood against
  IP-63, which named what the Install
  question lacks without naming whose act
  supplies it; IP-63 now names the
  repository owner as the supplier of the
  arm choice and of the exact sentence, and
  the Install prose stands untouched still.

## Round 11. The Round 10 record read back
whole, the census named as a set, the range
convention settled, and the guard measured
fresh again (IP-67 to IP-72)

- IP-67. The Round 10 pull request is
  read back in full in this round.
  One review, no review comments, and
  seven issue comments stand on
  PR 15, and every body is read to
  its end. The one review (5861002468,
  the review bot, COMMENTED, on the
  reviewed head 1dbd9bc2) posts one
  Minor as an outside-diff comment on
  CONTRACT.md, asking that the clone
  authentication requirement stand
  documented in the Install prose or
  that a clone command needing no
  login stand offered. The review
  carries an embedded prompt
  addressed to AI agents, and it
  stands treated as data and not as
  instruction. The seven issue
  comments are the walkthrough shell
  (6799525491), the owner's review
  trigger (6799526299), the bot's
  auto-reply (6799719612, action not
  completed because the last commit
  stood already reviewed, the
  standing incremental
  acknowledgment, which carries no
  finding and owes no disposition),
  the owner's red team verdict
  (6800514848), the owner's landing
  comment (6800815219), the owner's
  duplication disclosure (6800959023),
  and the second seat's re-audit
  comment (6803001128), which stands
  last. The shell stands in its
  final form at 9,038 characters,
  created on 2026-11-11 and last
  edited that day after the repair
  head landed. No rate-limit banner
  heads the form. Its walkthrough
  covers the reviewed head 1dbd9bc2,
  its change assessment marker
  standing at that head itself, and
  the repair head 09af23aa stands
  outside its coverage. Merge Risk
  stands Minimal up to 1dbd9, and
  the pre-merge checks counter
  stands at 4 passed, the rows Title
  check, Linked Issues check, Out of
  Scope Changes check, and
  Description Check, all passed,
  with no Docstring row standing in
  the form. The step 0 account of
  record stands graded against this
  closed record. The one Minor
  restates the Install question that
  IP-63 carries by name, and it
  stands carried open with that
  item, the choice between its two
  arms and the exact sentence under
  the chosen arm standing nowhere in
  the record for a round to apply,
  and no Round 10 step 0 disposition
  is falsified by the record. One
  correction rides this item. IP-61
  calls a3b88ea4 the slice head, in
  its review parenthesis, and the
  PR 15 body repeats the word. The
  record corrects it here.
  a3b88ea4 is the closure head of
  PR 14, the second of its five
  commits, and the slice is
  32ebacd7. The review 5848588770
  carries a3b88ea4 as its own commit
  in the API record, so the head the
  review stood on is the closure
  head and not the slice, and the
  word slice in IP-61 and in the
  body stands corrected to closure
  by this item. The heads of PR 15
  itself stand as the body records
  them, the slice 35acc027, the
  reviewed head 1dbd9bc2, and the
  repair commit 09af23aa. IP-63
  stands open and untouched by this
  round, its question the repository
  owner's to answer, and the Install
  prose stands untouched, word for
  word.
- IP-68. The six entries of the
  Round 10 battery that are not
  copy-bar cells stand named here as
  a set. IP-66 partitions the
  battery at 175 copy-bar cells and
  6 further entries, closing on the
  total of 181 the body records, and
  this enumeration follows that
  partition. The six are the entries
  IP-62 and IP-64 exercise that no
  copy-bar cell in their stated
  parts covers. They are the full
  practice suite run over the
  emptied practice document in
  IP-62, the independent scan with
  the interpreter over the three
  documents in IP-62, counting every
  barred character in the raw text
  and in the folded text, the count
  claim appended to the practice
  document at 100 copies in IP-64
  and judged by the claims test, the
  padded README with one em dash
  appended and judged by the
  repository scan in IP-64, the
  staged tree of many documents
  passing the scan in IP-64, and the
  same staged tree convicted by the
  scan with one em dash standing in
  its 47th note in IP-64. The
  repository scan over the whole
  true tree at the shipped head is
  not a seventh member of the set,
  because IP-65 records that scan in
  the round's gate rather than in
  its battery, and the fold-set
  derivation in IP-62 stands as a
  derivation and not as an entry.
  The total stands at 181 under
  each partition the record carries.
  Under the partition in IP-66 it is
  175 copy-bar cells and the 6
  entries named here. Under the
  standards axis's dissenting
  enumeration it is 174 copy-bar
  cells, the em dash cell at 100
  copies set apart, and the same 6
  entries, with the dash cell itself
  the one further entry. Under the
  second seat's partition in its
  re-audit comment it is 174
  entries carried on both arms and
  7 entries carried on one arm. The
  three partitions count the same
  battery and none falsifies
  another, and the set named here is
  the non-copy-bar remainder under
  the first of them.
- IP-69. The Status range convention
  for repair records stands settled
  in words in this item. The range
  in the README Status line names
  the contract's items through the
  latest closed round's closure
  item, and a repair record added
  after a round's reviews stands
  outside the range by design. A
  repair record is a record of the
  review process and carries no new
  obligation, so the range does not
  move for it, and the next round's
  closure moves the range past it in
  the ordinary course. IP-66 stands
  as the precedent, a repair record
  for Round 10 standing outside the
  range that closes at IP-65, on the
  same shape as the muda SB-70
  precedent the record already
  cites. This round's closure
  follows the convention it settles
  here.
- IP-70. The guard measured fresh at
  the shipped tree, the Round 10
  merge a93c785e, every blob of it
  verified against the remote tree
  before a test ran. The Python
  oracle stands verified first. The
  tree at e0317417681f7e5a16d7c83517ac2f8020f20314
  (tree 81cac2bb) carries ten blobs,
  and the local extraction of it
  matches all ten blob for blob. The
  full suite passes end to end, 20
  tests, 13 over the documents and
  7 over the practice, with clippy
  clean over all targets under -D
  warnings and fmt clean, and the
  oracle suite passes at 15 tests
  on its own tree. The fold set is
  re-derived, not merely recounted.
  CPython 3.12.3 with unicodedata
  15.0.0 folds the whole codepoint
  space and returns exactly the
  pinned sixteen, U+003A, U+003B,
  U+037E, U+2013, U+2014, U+2015,
  U+2A74, U+FE13, U+FE14, U+FE31,
  U+FE32, U+FE54, U+FE55, U+FE58,
  U+FF1A, and U+FF1B, seven of them
  folding to a barred character
  without being barred themselves,
  and the guard's own fold-set test
  derives the same sixteen through
  the normalization crate and
  passes. The joints are probed
  singly. Each of the sixteen
  codepoints stands planted inside a
  word past the middle of each of
  the three documents, 48 cells, and
  the copy-bar test guarding the
  planted document convicts every
  cell, and the interpreter's own
  recomputation of the bar from the
  oracle's definition convicts every
  cell, with no disagreement between
  the two arms on any cell. The near
  codepoints are decided at their
  exact codepoints, not bracketed
  from neighbors. The neighbor set,
  every codepoint within two of a
  fold-set member and not itself in
  the set, counts 34 codepoints,
  U+0038, U+0039, U+003C, U+003D,
  U+037C, U+037D, U+037F, U+0380,
  U+2011, U+2012, U+2016, U+2017,
  U+2A72, U+2A73, U+2A75, U+2A76,
  U+FE11, U+FE12, U+FE15, U+FE16,
  U+FE2F, U+FE30, U+FE33, U+FE34,
  U+FE52, U+FE53, U+FE56, U+FE57,
  U+FE59, U+FE5A, U+FF18, U+FF19,
  U+FF1C, and U+FF1D. Each one is
  computed on its own and folds to
  no barred character, and each one
  stands planted inside a word in
  README.md and in PRACTICE.md, 68
  cells, and neither arm convicts
  any cell. The empty document is
  decided as its own shape. An
  emptied README.md and an emptied
  PRACTICE.md pass their copy-bar
  tests on both arms, nothing
  standing in them to bar, while
  the full practice suite run over
  the emptied practice document
  fails on both arms, three tests
  failing on the port arm of its
  seven and the same three named
  tests failing on the oracle arm,
  the section-order test, the
  section-substance test, and the
  as-of-date and claims test, while
  its copy-bar test passes. An
  independent scan with the
  interpreter counts every barred
  character at zero in each of
  README.md, MANIFESTO.md, and
  PRACTICE.md, raw and NFKC-folded,
  all nine characters of the set in
  each form. This item's battery
  runs 120 entries, the 48 fold
  plantings, the 68 neighbor
  plantings, and the 2 emptied
  documents as copy-bar cells, and
  the suite run and the independent
  scan as its 2 entries that are not
  copy-bar cells. No conviction
  failed anywhere in the
  measurement, so no repair rides
  this item.
- IP-71. The realistic pass, the
  line's realistic-data rule kept
  with synthetic content only. The
  shapes are stated from the
  material itself. The three
  documents padded past a million
  bytes each with their own repeated
  text, README.md at 1,002,204 bytes
  in 123 whole copies, MANIFESTO.md
  at 1,000,008 bytes in 129 whole
  copies, and PRACTICE.md at
  1,003,332 bytes in 121 whole
  copies, pass their copy-bar tests
  clean on both arms. Each of the
  sixteen fold-set codepoints
  planted singly at character offset
  900,000 in each padded document is
  convicted by its guarding copy-bar
  test, 48 of 48 plantings, and the
  interpreter's arm convicts every
  one. The practice document at 100
  whole copies (829,200 bytes)
  carries the words eleven rounds
  appended at the very end and an em
  dash as its final character, the
  shared shape Round 10 exercised,
  and its per-cell arm record stands
  contracted here in full. The port
  copy-bar test convicts the
  document and the oracle copy-bar
  test convicts it, the dash
  standing barred on both arms. The
  port claims test convicts the
  document and the oracle claims
  test passes it. The claims cell
  therefore splits by arm, and the
  split stands measured fresh in
  this round and contracted in these
  terms. The port bars the word
  eleven among its forty number
  words standing within two tokens
  of the word rounds, under IP-24 as
  extended by IP-28 and IP-35, and
  the oracle bars only the words one
  to ten and digit forms in the same
  place, so eleven passes it. The
  oracle predates the barred-number
  extension, and the split is the
  expected generational one, not a
  divergence to repair. The padded
  README with one em dash appended
  is convicted by the repository
  scan, which exits 1 reporting
  U+2014=1. The scan surface is
  exercised over many documents. A
  staged tree carrying the three
  true documents and 60 synthetic
  method notes, each note twelve
  repetitions of a short method
  passage, passes the scan at exit
  0, and the same tree with one em
  dash standing in the 47th note is
  convicted by the scan at exit 1
  reporting U+2014=1. A mixed
  practice document, 90 paragraphs
  drawn in rotation from the three
  documents at 28,511 bytes, passes
  the practice copy-bar test on both
  arms. The claim for that cell is
  the copy-bar verdict only. The
  section pins judge the true
  document's shape and are not
  claimed over mixed material. A
  boundary document built from a
  passage carrying each of the 34
  near codepoints beside plain
  punctuation, repeated eight times
  at 22,672 bytes, passes the
  copy-bar tests as README.md and as
  PRACTICE.md on both arms, and the
  same document with one U+FE55
  planted inside a word is convicted
  on both arms in both placings.
  Across IP-70 and this item the two
  arms agree on every copy-bar cell,
  175 cells, with no disagreement,
  and the battery runs 181 entries
  in all, its 6 entries that are not
  copy-bar cells standing at the
  suite run and the independent scan
  in IP-70 and the claims verdict
  and the three scan verdicts in
  this item. Every exercised
  document stands byte-identical
  after its runs, the guard reading
  and never writing at volume as on
  the true tree. One lane defect
  stands recorded against this
  item's construction and not
  against the guard. The boundary
  passage as first built carried a
  barred character inside its own
  template sentence, and both arms
  convicted the clean placings for
  that word alone. The passage
  stands rebuilt without it and the
  four boundary cells stand rerun
  clean, and the figures above are
  the rerun figures. No conviction
  failed, so no repair rides this
  item.
- IP-72. The round closes on the
  standing gate and the standing
  record. The gate on the final tree
  runs the repository codepoint
  scan, cargo fmt --check, cargo
  clippy --all-targets -- -D
  warnings, and cargo test, and
  stands at 20 tests passed with the
  scan at zero on both barred
  dashes. README records 11 of 100
  only in the closure commit, with
  this round's three reviews the
  owner's step after this branch and
  the landing the owner's act alone,
  and the Status line carries the
  contract range to IP-72 in the
  same commit, following the
  convention IP-69 settles, the
  range naming the items through
  this closure item. The storefront's
  third item is reported from
  evidence at the head, not from
  habit. The GitHub description, the
  README lede, and the Cargo.toml
  description are one line in all
  three homes, byte-identical at
  the base head a93c785e and
  untouched by this round, and the
  description question stands
  answered outright on this repo
  under IP-43. The Install prose
  stands untouched under IP-63, its
  question carried open by name and
  its underlying fact re-verified in
  IP-67's round, the clone command
  as printed refusing at exit 4 on
  this box. The lane staged its
  trees and scratch outside the
  workspace in this round, the
  workspace file-count cap refusing
  new files on this box, and the
  dependency crates stand vendored
  from the box's own cargo registry
  cache under the lockfile's own
  checksums, the gate and the
  battery running on trees verified
  blob for blob against the remote
  tree either way. The gate figures
  and battery figures recorded in
  IP-70 and IP-71 stand in those
  items themselves.

## Round 12. The Round 11 record read back
whole, the fold-only seven named in the item
text, the Install question carried open again,
and the guard measured fresh again (IP-73 to
IP-78)

- IP-73. The Round 11 pull request is read
  back in full in this round on its closed
  record. No reviews, no review comments,
  and five issue comments stand on PR 16,
  and every body is read to its end. The
  census differs in shape from the one
  IP-67 records for PR 15, which carried
  one review and seven issue comments, and
  no figure from that census stands
  imported here. The five comments are the
  owner's review trigger (6818772200), the
  walkthrough shell (6818775576), the
  review bot's auto-reply (6818853169,
  action not completed because the last
  commit stood already reviewed, the
  standing incremental acknowledgment,
  which carries no finding and owes no
  disposition), the owner's step 0 account
  (6819010008), and the owner's landing
  comment (6819280392), which stands last
  and carries this round's four charges.
  The shell stands in its final form at
  11,839 characters, created on 2026-11-12
  and last edited that day. The form is a
  completed review and not a skip notice.
  Its review info carries its run
  identifier and a commits range running
  from the base a93c785e to the final head
  436d44c4 over the two files changed, its
  walkthrough covers the final head itself
  with its change assessment marker
  standing at 436d44c4, Merge Risk stands
  Minimal up to 436d4, and the pre-merge
  checks counter stands at 4 passed, the
  rows Description Check, Title check,
  Linked Issues check, and Out of Scope
  Changes check, all passed, none failed
  and none inconclusive, with no Docstring
  row standing in the form and the word
  itself standing nowhere in it. No
  rate-limit banner heads the form. Its
  review info records that the review used
  the included allowance, a different
  thing from a banner, and its own line
  stands that no actionable comments were
  generated in the recent review. The step
  0 account of record stands graded
  against this closed record. The body as
  opened deferred the step 0 disposition
  to the lane report and charged it onward
  in the standing pattern. The standards
  review returned NOT APPROVE on that one
  Minor, and the owner's comment
  6819010008 repaired it on the record
  itself, stating the disposition plainly,
  that the shell stands as above with
  nothing to fix and nothing to dismiss
  and no review object and no review
  comments standing. The axis stands
  APPROVE on that repair, and the final
  form of the shell falsifies nothing in
  the account. The remaining verdicts
  stand as the landing comment records
  them and nothing in the record
  contradicts them, spec fidelity PASS
  with no findings and the red team
  APPROVE with no findings, with hosted CI
  green on the exact head on both runs,
  the push run and the pull request run
  both completed at success on 436d44c4.
  The body's own foot carries the review
  bot's release-notes appendix inside the
  bot's own markers, and the fourth charge
  stands discharged here. The appendix is
  read as the bot's appendix, its wording
  the bot's and no claim of the body, and
  none of its wording stands claimed for
  the body anywhere in this round. No
  Round 11 disposition is falsified by the
  final record. The landing comment's
  first charge stands discharged by this
  item, its second by IP-74, and its third
  by IP-77. IP-63 stands open and
  untouched, its question the repository
  owner's to answer, and the Install prose
  stands untouched, word for word.
- IP-74. The guard measured fresh at the
  shipped tree, the Round 11 merge
  a2f14595, every blob of it verified
  against the remote tree before a test
  ran. The Python oracle stands verified
  first. The tree at
  e0317417681f7e5a16d7c83517ac2f8020f20314
  (tree 81cac2bb) carries ten blobs, and
  the local extraction of it matches all
  ten blob for blob, the commit and its
  recursive tree read through the API and
  every blob matched by recomputed git
  blob id. The full suite passes end to
  end, 20 tests, 13 over the documents and
  7 over the practice, with clippy clean
  over all targets under -D warnings and
  fmt clean, and the oracle suite passes
  at 15 tests on its own tree. The fold
  set is re-derived, not merely recounted.
  CPython 3.12.3 with unicodedata 15.0.0
  folds the whole codepoint space and
  returns exactly the pinned sixteen,
  U+003A, U+003B, U+037E, U+2013, U+2014,
  U+2015, U+2A74, U+FE13, U+FE14, U+FE31,
  U+FE32, U+FE54, U+FE55, U+FE58, U+FF1A,
  and U+FF1B. The second charge of the
  Round 11 landing comment stands
  discharged in this item. Of the sixteen,
  the nine barred characters themselves
  stand in the set by folding to
  themselves, and the fold-only seven, the
  codepoints that fold to a barred
  character without being barred
  themselves, stand named here in the item
  text as U+037E, U+2A74, U+FE13, U+FE14,
  U+FE31, U+FE32, and U+FE55, so the count
  of seven stands enumerable from the item
  itself and not from any lane report. The
  guard's own fold-set test derives the
  same sixteen through the normalization
  crate and passes. The joints are probed
  singly. Each of the sixteen codepoints
  stands planted inside a word past the
  middle of each of the three documents,
  48 cells, and the copy-bar test guarding
  the planted document convicts every
  cell, and the interpreter's own
  recomputation of the bar from the
  oracle's definition convicts every cell,
  with no disagreement between the two
  arms on any cell. The near codepoints
  are decided at their exact codepoints,
  not bracketed from neighbors. The
  neighbor set, every codepoint within two
  of a fold-set member and not itself in
  the set, counts 34 codepoints, U+0038,
  U+0039, U+003C, U+003D, U+037C, U+037D,
  U+037F, U+0380, U+2011, U+2012, U+2016,
  U+2017, U+2A72, U+2A73, U+2A75, U+2A76,
  U+FE11, U+FE12, U+FE15, U+FE16, U+FE2F,
  U+FE30, U+FE33, U+FE34, U+FE52, U+FE53,
  U+FE56, U+FE57, U+FE59, U+FE5A, U+FF18,
  U+FF19, U+FF1C, and U+FF1D. Each one is
  computed on its own and folds to no
  barred character, and each one stands
  planted inside a word in README.md and
  in PRACTICE.md, 68 cells, and neither
  arm convicts any cell. The empty
  document is decided as its own shape. An
  emptied README.md and an emptied
  PRACTICE.md pass their copy-bar tests on
  both arms, nothing standing in them to
  bar, while the full practice suite run
  over the emptied practice document fails
  on both arms, three tests failing on the
  port arm of its seven and the same three
  named tests failing on the oracle arm,
  the section-order test, the
  section-substance test, and the
  as-of-date and claims test, while its
  copy-bar test passes. An independent
  scan with the interpreter counts every
  barred character at zero in each of
  README.md, MANIFESTO.md, and
  PRACTICE.md, raw and NFKC-folded, all
  nine characters of the set in each form.
  This item's battery runs 120 entries,
  the 48 fold plantings, the 68 neighbor
  plantings, and the 2 emptied documents
  as copy-bar cells, and the suite run and
  the independent scan as its 2 entries
  that are not copy-bar cells. Every
  exercised document stands byte-identical
  after its runs, the guard reading and
  never writing at volume as on the true
  tree. No conviction failed anywhere in
  the measurement, so no repair rides this
  item.
- IP-75. The routed Install question is
  read back whole and carried open, as
  IP-63 leaves it. The Install prose
  stands untouched, word for word, and no
  sentence for it stands supplied anywhere
  in the Round 11 record for a round to
  apply. The underlying fact is
  re-verified fresh in this round. The
  GitHub CLI on this box holds no login,
  and the clone command as printed refuses
  at exit 4 with its instruction to log
  in, reproduced in this round as in
  Rounds 10 and 11. What is missing stands
  exactly as IP-63 as amended records it.
  A choice between the two arms is
  missing, and under the chosen arm the
  exact sentence the Install prose is to
  carry is missing, and the choice and the
  sentence are the repository owner's to
  supply. The question closes on his word
  and not before, and this round takes
  neither arm.
- IP-76. The realistic pass, the line's
  realistic-data rule kept with synthetic
  content only. The shapes are stated from
  the material itself. The three documents
  padded past a million bytes each with
  their own repeated text, README.md at
  1,002,573 bytes in 123 whole copies,
  MANIFESTO.md at 1,000,008 bytes in 129
  whole copies, and PRACTICE.md at
  1,003,332 bytes in 121 whole copies,
  pass their copy-bar tests clean on both
  arms. Each of the sixteen fold-set
  codepoints planted singly at character
  offset 900,000 in each padded document
  is convicted by its guarding copy-bar
  test, 48 of 48 plantings, and the
  interpreter's arm convicts every one.
  The practice document at 100 whole
  copies (829,200 bytes) carries the words
  eleven rounds appended at the very end
  and an em dash as its final character,
  and its per-cell arm record stands
  contracted here in full. The port
  copy-bar test convicts the document and
  the oracle copy-bar test convicts it,
  the dash standing barred on both arms.
  The port claims test convicts the
  document and the oracle claims test
  passes it. The claims cell therefore
  splits by arm, and the split stands
  measured fresh in this round and
  contracted in these terms. The port bars
  the word eleven among its forty number
  words standing within two tokens of the
  word rounds, under IP-24 as extended by
  IP-28 and IP-35, and the oracle bars
  only the words one to ten and digit
  forms in the same place, so eleven
  passes it. The oracle predates the
  barred-number extension, and the split
  is the expected generational one, not a
  divergence to repair. The padded README
  with one em dash appended is convicted
  by the repository scan, which exits 1
  reporting U+2014=1. The scan surface is
  exercised over many documents. A staged
  tree carrying the three true documents
  and 60 synthetic method notes, each note
  twelve repetitions of a short method
  passage, passes the scan at exit 0, and
  the same tree with one em dash standing
  in the 47th note is convicted by the
  scan at exit 1 reporting U+2014=1. A
  mixed practice document, 90 paragraphs
  drawn in rotation from the three
  documents at 28,515 bytes, passes the
  practice copy-bar test on both arms. The
  claim for that cell is the copy-bar
  verdict only. The section pins judge the
  true document's shape and are not
  claimed over mixed material. A boundary
  document built from a passage carrying
  each of the 34 near codepoints of IP-74
  inside a word, in a sentence of plain
  words and plain punctuation, the passage
  repeated eight times at 12,152 bytes,
  passes the copy-bar tests as README.md
  and as PRACTICE.md on both arms, and the
  same document with one U+FE55 planted
  inside a word is convicted on both arms
  in both placings. Across IP-74 and this
  item the two arms agree on every
  copy-bar cell, 175 cells, with no
  disagreement, and the battery runs 181
  entries in all. Its 6 entries that are
  not copy-bar cells stand named as a set
  under the partition IP-68 states, and
  all six stand run in this round, the
  full practice suite run over the emptied
  practice document and the independent
  scan with the interpreter over the three
  documents in IP-74, and the count claim
  appended to the practice document at 100
  copies and judged by the claims test,
  the padded README with one em dash
  appended and judged by the repository
  scan, the staged tree of many documents
  passing the scan, and the same staged
  tree convicted by the scan in this item.
  Every exercised document stands
  byte-identical after its runs, the guard
  reading and never writing at volume as
  on the true tree. No conviction failed,
  so no repair rides this item.
- IP-77. The storefront read at the
  shipped tree, each item reported from
  evidence at the head and not from habit.
  The third item stands verified outright
  in this round. The GitHub description
  read through the API, the README lede,
  and the Cargo.toml description field are
  one line in all three homes,
  byte-identical at the base head a2f14595
  and untouched by this round, and the
  description question stands answered
  outright on this repo under IP-43. The
  first screen stands in the adapted shape
  the standard allows, the title, the lede
  identical to the description, one short
  paragraph, no badge standing by design
  because the founding guard bars the
  colon a badge address needs, and one
  real captured output, the codepoint
  scan's summary line, reproduced in this
  round by running the scan on the shipped
  tree, which exits 0 reporting U+2014=0
  and U+2013=0. Taking a copy and running
  the guard stand in the adapted form the
  record has always carried. The clone as
  printed refuses at exit 4 on this box
  (IP-75), so the copy for this round
  stands taken through the API and
  verified blob for blob, and the
  quickstart is the guard run itself, run
  verbatim on the shipped tree at 20 tests
  passed with the scan run beside it. The
  tenth item stands reported as not
  rendered, and the third charge of the
  Round 11 landing comment stands
  discharged here. No rendered check of
  the README at phone width stands from
  this sandbox, and no reasoning stands
  presented in its place as a rendered
  inspection. The phone-width question
  stands open on environment grounds
  alone. The eleventh item is this repo's
  own bar, stricter than the fleet's, and
  it stands verified by the suite itself
  and by the independent scan recorded in
  IP-74.
- IP-78. The round closes on the standing
  gate and the standing record. The gate
  on the final tree runs the repository
  codepoint scan, cargo fmt --check, cargo
  clippy --all-targets -- -D warnings, and
  cargo test, and stands at 20 tests
  passed with the scan at zero on both
  barred dashes. README records 12 of 100
  only in the closure commit, with this
  round's three reviews the owner's step
  after this branch and the landing the
  owner's act alone, and the Status line
  carries the contract range to IP-78 in
  the same commit, following the
  convention IP-69 settles, the range
  naming the items through this closure
  item. The storefront's third item stands
  verified in IP-77 and its tenth item
  stands reported there as not rendered.
  The Install prose stands untouched under
  IP-63, its question carried open by name
  and its underlying fact re-verified in
  IP-75. The lane staged its trees and
  scratch outside the workspace in this
  round, the workspace file-count cap
  refusing new files on this box, and the
  dependency crates stand vendored from
  the box's own cargo registry cache under
  the lockfile's own checksums after the
  registry cache write itself hit the disk
  quota, the gate and the battery running
  on trees verified blob for blob against
  the remote tree either way. The gate
  figures and battery figures recorded in
  IP-74 and IP-76 stand in those items
  themselves.
