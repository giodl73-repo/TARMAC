# Pulse 01: WP-001 `tarmac-network` air-network kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The air-network graph kernel — the pipeline primitive every other crate depends on.
Implements the load-bearing identity, connectivity, and typed demand basis (peak/average)
invariants required by REQ-007.

## Planned changes

- `Cargo.toml` workspace (member `crates/tarmac-network`).
- `crates/tarmac-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/tarmac-network/src/lib.rs`: `Airport`, `Route` (with typed `DemandBasis` enum),
  `Network`, `NetworkError`; `add_airport`/`add_route` (identity + validation);
  `airport_count`, `route_count`, `degree`, `is_connected`, `has_diverse_path`,
  `incident_ops`.

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria

- Workspace compiles; `cargo test -p tarmac-network` green.
- Tests cover: build network; degree; connectivity vs gap; incident ops; demand basis
  preserved (peak/average); `has_diverse_path` true on a ring/mesh and false on a single-path
  chain; duplicate-airport, non-positive ops, unknown-airport typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test -p tarmac-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented;
WORK_PACKAGES WP-001 → done; unblock WP-002.

## Status

Completed — the six-crate workspace and validation baseline are implemented.
