# Architecture

## Scope

Repo: TARMAC

Architecture type: target (greenfield). Describes the intended system shape that satisfies
`REQUIREMENTS.md` and `SPECIFICATION_BASELINE.md`, including the multi-scale model. No
components are built yet; boundaries and dependency direction are decided here so work
packages can be allocated cleanly.

## Architecture Summary

TARMAC is a layered, scale-aware pipeline: public aviation data becomes a typed, labelled,
**scale-tagged** corpus; the corpus becomes an air-network graph; the graph and corpus feed
dimension scoring; scores feed tier classification and SLA-conformance; tier/score outputs
feed gap analysis (runnable at a chosen scale) and conceptual design; everything passes the
`.roles` review gate before promotion. The air-network graph kernel is the primitive every
other layer depends on, mirroring the portfolio. Layers depend strictly downward — no cycles.

## Components

| Component | Boundary ID | Responsibility | Requirement IDs | Interfaces | Evidence |
|---|---|---|---|---|---|
| `tarmac-network` (air-network kernel) | PKG-001 | Typed graph (Airport/Route), stable identity, connectivity/centrality (DIM-04), capacity/delay helpers with typed demand basis. | REQ-004, REQ-005, REQ-007 | IF-005 (lib API) | future VER-004/005/007 |
| `tarmac-corpus` (corpus + data) | PKG-002 | Corpus file IO + schema incl. `scale`/`market` tags, `data/sources.md` registry, evidence labels. | REQ-001, REQ-002, REQ-003, REQ-016 | IF-001, IF-002 | future VER-001/002/003/016 |
| `tarmac-score` (scoring) | PKG-003 | Dimension pool DIM-01..13, 0–10 scoring, rubric calibration + versioning. | REQ-006 | IF-003 | future VER-006 |
| `tarmac-tier` (tier/SLA) | PKG-004 | Tier classification T1–T4, SLA terms, DIM-13 conformance, tier-SLA gaps. | REQ-014, REQ-015 | IF-004 | future VER-014/015 |
| `tarmac-gap` (gap analysis) | PKG-005 | Plot dimension space, find under-served regions, scale-filtered runs, null results. | REQ-008, REQ-016 | (internal) | future VER-008/016 |
| `tarmac-cli` (orchestration) | PKG-006 | Commands that drive the pipeline (incl. `--scale`) and emit artifacts. | REQ-001 (regen path) | IF-006 (CLI) | future VER-001 |
| review layer (`.roles/`) | — (docs, not a crate) | Parliament + editorial gate, scope boundary. | REQ-009, REQ-010, REQ-011 | review records | EVID-009/010/011 |

## Package / Language Boundaries

Detailed inventory belongs in `PACKAGE_BOUNDARIES.md` (deferred). Summary:

| Boundary ID | Crate / Module | Language | Responsibility | Allowed Dependencies |
|---|---|---|---|---|
| PKG-001 | `tarmac-network` | Rust | Graph primitive + metrics | (none internal) |
| PKG-002 | `tarmac-corpus` | Rust | Corpus IO, scale/market tags, labels, sources | PKG-001 (types) |
| PKG-003 | `tarmac-score` | Rust | Dimension scoring | PKG-001, PKG-002 |
| PKG-004 | `tarmac-tier` | Rust | Tier + SLA conformance | PKG-001, PKG-003 |
| PKG-005 | `tarmac-gap` | Rust | Gap/null analysis, scale filter | PKG-003, PKG-004 |
| PKG-006 | `tarmac-cli` | Rust | Orchestration | PKG-001..005 |

Dependency direction is strictly downward (CLI → gap → tier → score → corpus → network). The
review layer is docs/process, not a build dependency.

## Data Flow

```text
public sources (BTS On-Time / T-100 / FAA OPSNET-ASPM / OpenFlights / ICAO)
  -> [FLETCH fetch + cache]            (planned external acquisition)
  -> tarmac-corpus  (typed, labelled, scale-tagged corpus entries; data/sources.md)
  -> tarmac-network (air-network graph; identity, connectivity, capacity/delay helpers)
  -> tarmac-score   (DIM-01..13 scores, calibrated rubric)
  -> tarmac-tier    (T1-T4 classification, SLA conformance / DIM-13)
  -> tarmac-gap     (gap map at a chosen scale, under-served regions, null results)
  -> design proposals  (conceptual Aviation 2.0 upgrades)
  -> .roles review     (parliament + editorial gate)
  -> reports / artifacts
```

## Dependencies

| Dependency | Purpose | Boundary / Risk | Verification |
|---|---|---|---|
| `petgraph` | Graph data structure + algorithms in PKG-001. | Well-scoped; low risk. | future cargo test |
| `serde` / `csv` | Corpus + data IO in PKG-002. | Low risk. | future cargo test |
| `clap` | CLI in PKG-006. | Low risk. | future cargo test |
| FLETCH (portfolio) | Source-byte/paged data acquisition + cache manifests. | Planned; not yet wired. Avoid TRACKER-relative paths (CON). | intake + future gate |
| PROOF (portfolio) | Markdown QA for docs/artifacts. | Tool/CLI relationship, not runtime. | `proof check .` (active) |
| METIS-CORE / RLINE (portfolio) | Optional graph partitioning / shared kernels for gap analysis. | Deferred until gap wave. | deferred |

## Failure Modes

| Failure Mode | Impact | Mitigation | Evidence |
|---|---|---|---|
| Missing/blocked source for a corpus quantity. | Incomplete score. | Hold row with `source-needed` label (REQ-005); never fabricate. | future VER-005 |
| Element lacks stable identity or scale tag. | Unsafe joins / cross-scale mixing. | Reject/hold at PKG-002 schema gate (SPEC-001/013). | future VER-004/016 |
| Rubric not yet calibrated. | Scores provisional. | Label provisional; calibrate from corpus (REQ-006). | future VER-006 |
| Demand basis unknown (peak/average; IMC/VMC). | Capacity/delay claim unfounded. | Require basis named; hold if unknown (REQ-007/SPEC-DB-01). | future VER-007 |
| Scheduled data taken as actual. | Overstated capacity / understated delay. | Label as proxy (SPEC-DB-02); prefer actual data. | future VER-003 |

## Open Risks

- Cross-scale data openness and scheduled-vs-actual ambiguity (SPEC-UNK-001/002) may force
  proxy-heavy early corpus.
- FLETCH integration is planned, not proven; until then acquisition is manual.
- Scale nesting representation (DEF-005) is unresolved; may affect the corpus schema.

## Role Review Notes

| Role Lens | Architecture Impact | Disposition |
|---|---|---|
| Systems engineering / Scope Keeper | Layered, downward-only dependencies; scale lives in the corpus layer and flows to gap, not in the kernel. | pass |
| Airport Engineer | Capacity/delay helpers live in the kernel; demand-basis-unknown failure mode forces a hold. | pass |
| Operations Officer / Slot-and-Hub Realist | Demand-basis and scheduled-vs-actual failure modes force holds/labels rather than fabricated claims. | pass |
| Optimization / network lens | Initial draft let `tarmac-corpus` depend on `tarmac-score`, risking a cycle; resolved by making score depend on corpus (one-way). | resolved |
| Citation Auditor / Numeracy Checker | No quantities or arithmetic asserted. | pass |

Fixed-point note: one actionable finding (potential dependency cycle) was raised and applied
by fixing the dependency direction. No unresolved critical/major finding. Detailed package
boundaries deferred to `PACKAGE_BOUNDARIES.md`.
