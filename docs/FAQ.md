# Frequently Asked Questions

### Why do `README.md`, `MANIFESTO.md`, and `PRACTICE.md` forbid colons, semicolons, and dashes?
Language models lean heavily on em dashes, colons, and semicolons to glue loose clauses together instead of writing direct declarative prose. Banning all 16 raw and NFKC-folded codepoints (`IP-2`) forces every sentence in the three primary texts to stand on clear syntax and immediately catches unedited model prose. Note that companion engineering docs under `docs/` only enforce the fleet-wide zero-dash (`U+2014` / `U+2013`) rule.

### How can `README.md` link to sibling repositories on GitHub without writing `https://` (which contains a colon)?
GitHub renders root-relative links such as `[flighttape](/david-engelmann/flighttape)` directly to `https://github.com/david-engelmann/flighttape` while containing zero colons (`U+003A`), satisfying `IP-2` cleanly!

### Why does `cargo install` fail with `publish = false`?
`impatient-programming` is a specification and document-guard repository, not an installable CLI tool (`IP-13`). Clone the repository to read the three documents locally or browse the zero-JavaScript web edition on GitHub Pages (`https://david-engelmann.github.io/impatient-programming/`).

### How are closed perfection rounds counted?
Under Principle 5 (`Count honestly`), a round counts only when a complete cycle of contract definition, implementation, local gate pass, three independent reviews (specification, standards, red-team), and owner verification has closed on `main`. Sub-steps, drafts, and unmerged passes never increment the round count.
