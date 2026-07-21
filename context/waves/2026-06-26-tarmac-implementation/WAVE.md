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
| 01 | WP-001 `tarmac-network` | pending | Air-network kernel: identity, connectivity, centrality, capacity/delay helpers, typed demand basis. |
| 02 | WP-002 `tarmac-corpus` | pending | Corpus model + scale/market tags + schema + sources + evidence labels. |
| 03 | WP-003 `tarmac-score` | pending | Dimension scoring DIM-01..13 + rubric record. |
| 04 | WP-004 `tarmac-tier` | pending | Tier T1–T4 + SLA conformance + tier-SLA gap. |
| 05 | WP-005 `tarmac-gap` | pending | Gap analysis (scale-filtered) + null result. |
| 06 | WP-006 `tarmac-cli` | pending | CLI orchestration (`--scale`) + reproducible artifacts. |

## Success criteria

- Each work package meets its exit criteria and verification commands.
- Workspace stays green (`cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --workspace`) after every pulse.
- `proof check .` stays clean.
- VTRACE trace/verification rows updated as each WP closes.
