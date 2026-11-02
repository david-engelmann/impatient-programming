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
