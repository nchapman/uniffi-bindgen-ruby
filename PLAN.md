# UniFFI Ruby Bindgen Plan (Instance)

## Purpose
Build an idiomatic, production-grade Ruby UniFFI backend using the same repeatable process used for Dart.

## Template Variables
| Variable | Ruby Value |
|---|---|
| `LANG_NAME` | `Ruby` |
| `LANG_ID` | `ruby` |
| `BINARY_NAME` | `uniffi-bindgen-ruby` |
| `CONFIG_TABLE` | `[bindings.ruby]` |
| `HOST_FORMAT_CMD` | `bundle exec rubocop` |
| `HOST_ANALYZE_CMD` | `bundle exec rubocop` |
| `HOST_TEST_CMD` | `bundle exec rspec` |
| `HOST_PACKAGE_FILE` | `Gemfile` |
| `OFFICIAL_INTEROP_REF` | `https://github.com/rust-lang/rust-bindgen` |

## Progress Snapshot (March 1, 2026)
### Completed
- Phase 0: Bootstrap scaffold (workspace, CLI, docs, fixture/golden test harness).

### In Progress
- Phase 1+: Full UniFFI feature parity implementation using UDL coverage ledger.

## Quality Bar
1. Deterministic generation outputs for golden-tested fixtures.
2. Runtime fixture suite green.
3. Idiomatic Ruby API surface (not Rust-shaped Ruby).
4. Strict lint/test gates enforced before merges.

## UDL Coverage Ledger
Track parity row-by-row against `/Users/nchapman/Drive/Code/lessisbetter/refs/uniffi-rs`.

| UDL Unit | Status | Notes |
|---|---|---|
| Top-level functions (sync) | Not started | |
| Top-level functions (`[Throws]`) | Not started | |
| Records (defaults/mutability) | Not started | |
| Enums (flat/data-carrying) | Not started | |
| Objects/interfaces lifecycle | Not started | |
| Trait methods | Not started | |
| Async futures | Not started | |
| Callback interfaces (sync/async) | Not started | |
| Custom types | Not started | |
| External/remote types | Not started | include `*FfiCodec`/exception codec cross-package contract |
| Rename/exclude/docstrings | Not started | |
| Regression rows (`regressions/*`) | Not started | |

## Git Commit Workflow
- Initialize Git at project start and keep history linear.
- Commit continuously as coherent units of change; do not batch unrelated work.
- Use descriptive commit messages that explain what changed and why.
- Do not use commit messages framed as milestone or step progress.
- Run relevant tests before each commit that changes behavior.
