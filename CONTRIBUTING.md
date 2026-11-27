# Contributing

`impatient-programming` is a prose method statement, not a software tool (`IP-1`, `IP-12`, `IP-13`). Its shipped artifact is the trio of Markdown documents (`README.md`, `MANIFESTO.md`, and `PRACTICE.md`), guarded by an internal Rust test suite (`src/`, `tests/test_documents.rs`, and `tests/test_practice.rs`) and `scripts/codepoint_scan.py`.

## Repository Rules

1. **Contract before edits**: Every change to the method documents or their guards begins as a numbered obligation in `CONTRACT.md`, paired with the test assertion that enforces it.
2. **Copy bar**:
   - `README.md`, `MANIFESTO.md`, and `PRACTICE.md` forbid colons (`:`), semicolons (`;`), em-dashes (`U+2014`), en-dashes (`U+2013`), horizontal bars (`U+2015`), and all compatibility codepoints that fold to them under Unicode NFKC (`16` codepoints total, enforced by `test_the_copy_bar_holds_in_both_documents` and `test_the_copy_bar_holds_over_practice`).
   - Every `.md`, `.txt`, `.rs`, `.toml`, `.yml`, `.py`, and `.lock` file in the repository must carry zero `U+2014` and zero `U+2013` codepoints (`scripts/codepoint_scan.py`).
3. **No product code outside test infrastructure**: `test_no_product_code_lives_in_the_repo` (`IP-13`) fails if any `.py`, `.js`, `.mjs`, `.jsx`, `.ts`, `.tsx`, `.go`, `.rs`, `.rb`, `.sh`, `.java`, `.c`, `.cc`, `.cpp`, `.swift`, or `.php` file exists outside `src/`, `tests/`, or `scripts/`.
4. **Mutable counts**: `MANIFESTO.md` and `PRACTICE.md` state zero per-repository round counts (`IP-8`, `IP-29`). Round counts live in `README.md` and move only when a full cycle of work, audit, two-axis review, red team, and owner verification closes.

## Running the Local Gate

Run all four commands locally from the repository root before opening or updating any branch:

```sh
python3 scripts/codepoint_scan.py
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All four commands must exit `0` with zero warnings and all 20 tests passing.
