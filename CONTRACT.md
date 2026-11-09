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
