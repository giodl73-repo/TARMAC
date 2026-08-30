# Wave: TARMAC Implementation

## Goal

Build the TARMAC pipeline from the accepted work packages (WP-001..006), one work package per
pulse, each compiling and testing green before the next starts.

## Thesis

The left side of the V is settled (`docs/vtrace/`). This wave is the implementation build: turn
accepted work packages into tested Rust crates, bottom-up, with scale threaded through the
corpus and gap layers, and every pulse running the WP verification commands and recording
evidence back into the VTRACE trace.

## Pulse table

| Pulse | Work Package | Status | Outcome |
|------:|--------------|--------|---------|
| 01 | WP-001 `tarmac-network` | complete_fixture | Air-network kernel: identity, connectivity, centrality, capacity/delay helpers, typed demand basis. |
| 02 | WP-002 `tarmac-corpus` | complete_fixture | Corpus model + scale/market tags + schema + sources + evidence labels. |
| 03 | WP-003 `tarmac-score` | complete_fixture | Dimension scoring DIM-01..13 + rubric record. |
| 04 | WP-004 `tarmac-tier` | complete_fixture | Tier T1-T4 + SLA conformance + tier-SLA gap. |
| 05 | WP-005 `tarmac-gap` | complete_fixture | Gap analysis (scale-filtered) + null result. |
| 06 | WP-006 `tarmac-cli` | complete_fixture | CLI orchestration (`--scale`) + reproducible artifacts. |

## Success criteria

- Each work package meets its exit criteria and verification commands.
- Workspace stays green (`cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --workspace`) after every pulse.
- `proof check .` stays clean.
- VTRACE trace/verification rows updated as each WP closes.

## PITFALL Closeout

Portfolio PITFALL adoption on 2026-08-23 found stale implementation-wave and VTRACE rows
that still described the Rust workspace as pending greenfield work. The current fixture
workspace passes:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --locked` (34 tests)
- `cargo run -p tarmac-cli -- --help`

The remaining risk is L2 evidence: fixture results must not be promoted as a public
aviation-network finding without a cited corpus run and role review on the output packet.
