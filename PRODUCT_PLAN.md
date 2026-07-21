# TARMAC Product Plan

## Thesis

Score air networks at a declared scale, identify measurable capacity,
connectivity, resilience, and access gaps, and design Aviation 2.0
interventions only where evidence supports them.

## Implemented product shape

- Six-crate Rust workspace covering network, corpus, score, tier, gap, and CLI.
- International, national, regional, and local scale contracts.
- DIM-01..13 scoring and tier-SLA shortfall artifacts.
- Tail-versus-systemic gap classification.
- Deterministic tests and machine-readable CLI outputs.

## Current evidence

The implementation and fixture baseline are complete. The first cited
end-to-end aviation-network run, source manifest, and findings report remain the
next publication milestone.

## Next public work

1. Select a bounded source-backed airport or route-market corpus.
2. Publish reproducible delay, capacity, connectivity, and access inputs.
3. Run tier, gap, and sensitivity analysis with explicit operating basis.
4. Review the first gap-targeted intervention through the full panel.

## Non-goals

- No airspace, procedure, slot, route-award, or environmental design.
- No forecast of what airports, airlines, or agencies will build or fly.
- No uncited delay, capacity, connectivity, access, fare, or cost claim.
- No aggregation across scales without an explicit comparison basis.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p tarmac-cli -- --help
```
