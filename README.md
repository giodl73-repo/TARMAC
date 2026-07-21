# TARMAC

**Aviation 2.0 — multi-scale air-network analysis and conceptual design.**

**An airport network is not a route map. It is a promise about access, delay, and connection.**

TARMAC scores airports, routes, and hub systems across delay, capacity,
connectivity, resilience, competition, access, and intermodal connection. It
separates a busy airport from a network that reliably serves its markets.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> TARMAC is a research and conceptual-design project. It is not an engineering
> study, airspace or procedure design, environmental review, slot allocation,
> route award, or advocacy brief, and it claims no FAA, airport-authority,
> airline, or ICAO endorsement.

## Why this matters

Adding a gate, route, or runway does not automatically repair missed
connections, regional isolation, slot concentration, weather fragility, or
ground-access failure. TARMAC evaluates the service network before selecting
the intervention.

The transferable principle is: **infrastructure earns its place by improving a
measured network promise, not by adding visible capacity.**

## What is implemented

| Crate | Responsibility |
|---|---|
| `tarmac-network` | Airport, route, hub, market, and scale-aware network contracts. |
| `tarmac-corpus` | Evidence-labelled aviation corpus parsing and validation. |
| `tarmac-score` | DIM-01..13 score artifacts. |
| `tarmac-tier` | Tier-SLA classification and shortfall reporting. |
| `tarmac-gap` | Scale-filtered gap analysis and null-result reporting. |
| `tarmac-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

## Evidence status

The implementation baseline is complete and fixture-backed. The next public
milestone is the first cited end-to-end aviation-network analysis with
reproducible source manifests.

## Quick start

```powershell
cargo run -p tarmac-cli -- --help
cargo run -p tarmac-cli -- gap --help
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

Every element carries a scale and market. Delay and capacity claims retain
their operating and weather basis.

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md) — scope, product shape, and next work.
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).
