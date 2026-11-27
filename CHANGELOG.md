# Changelog

All notable milestones in `impatient-programming` are recorded here. Ongoing perfection rounds and obligation rulings are tracked in `CONTRACT.md` (`IP-1` onward).

## [Unreleased]

### Added

- `CITATION.cff` (Citation File Format 1.2.0) for citing *Impatient Programming*.
- `.github/workflows/pages.yml` rendering `README.md`, `MANIFESTO.md`, and `PRACTICE.md` into a zero-JavaScript static HTML/CSS reading edition on GitHub Pages.
- `.github/workflows/release.yml` building tag-triggered (`v*`) release archives, standalone HTML editions, and `SHA256SUMS`.
- Cross-platform CI matrix (`ubuntu-latest`, `macos-latest`, `windows-latest`) in `.github/workflows/ci.yml`.
- Package metadata (`license = "MIT"`, `publish = false`, `repository`, `homepage`, `readme`, `rust-version`, `exclude`) in `Cargo.toml` and `.cargo-home` exclusion in `scripts/codepoint_scan.py`.
- `CHANGELOG.md` and `CONTRIBUTING.md`.

## [0.1.0] - 2026-11-02

### Added

- `README.md`, `MANIFESTO.md`, and `PRACTICE.md` stating the ten principles and operational mechanics of Impatient Programming across the ten-repository portfolio.
- Pure-Rust document guard suite (`tests/test_documents.rs` and `tests/test_practice.rs`, 20 tests) enforcing the 16-codepoint raw and NFKC copy bar, principle order, tool inventory, command truthfulness, and zero product code in the repository (`IP-1` onward).
- `scripts/codepoint_scan.py` enforcing zero `U+2014` and `U+2013` across prose, source, and workflow files.
