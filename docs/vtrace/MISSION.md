# Mission

## Scope

Repo: TARMAC

VTRACE adoption scope: establish the mission baseline for TARMAC before creating
requirements, specification baselines, trace rows, or work packages. This file is the
leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`, `WP-*`,
verification, and validation records. TARMAC is greenfield: this mission defines intent
ahead of any implementation, and implementation must trace back to the needs and
constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | TARMAC shall turn public aviation data (e.g. BTS On-Time, T-100, FAA OPSNET/ASPM, OpenFlights, ICAO) into a reproducible scored corpus of existing air-network elements. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | TARMAC shall identify and explain air-network gaps — congestion/delay, lost connectivity, single-runway fragility, fortress-hub fares, weak intermodal access, and eroded small-community service — without overstating the evidence. | Every material claim is tied to a data artifact, command, source label, confidence label, or review record. | accepted |
| NEED-003 | TARMAC shall convert analysis into defensible conceptual Aviation 2.0 upgrade options, not engineering studies, airspace designs, or advocacy briefs. | Proposed projects and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, with the demand and economic basis labelled before publication. | accepted |
| NEED-004 | TARMAC shall keep network identity stable as analysis moves from raw airports/routes to scored networks, gap regions, and design proposals. | Element-bearing artifacts join through a stable airport/route/network identifier rather than a transient label, carrier, or map id. | accepted |
| NEED-005 | TARMAC shall expose aviation tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | TARMAC shall report a rigorous null result as a valid finding. | When the scored corpus shows a system is already fluid, connected, and accessible, the artifacts say so rather than manufacturing a gap. | accepted |
| NEED-007 | TARMAC shall classify each element into a four-tier hierarchy (T1 International Gateway Hub, T2 National Hub, T3 Regional Airport, T4 Local/General Aviation) and define capacity, delay, connectivity, and access SLAs per tier, so that "is air service adequate here?" is answered against an explicit tier promise. | Every analyzed element carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |
| NEED-008 | TARMAC shall apply the same methodology at multiple scales — international (intercontinental routes, gateway hubs), national, regional, and local — with every element tagged by scale and market, and analysis runnable at a chosen scale. | Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale; a gap run can target a single scale without cross-scale leakage. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| TARMAC maintainer | Know which commands, artifacts, and review gates define the current truthful repo state at a given scale. | A clean validation bundle runs and the resulting artifacts match the documented claims and declared scale. |
| Aviation analyst | Inspect scored networks, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces, confidence posture, and scale. |
| Airport / system planner | Understand why a network, tier, or project is supported, held, or downgraded. | Each claim names the data, scenario, role review, scale, and next evidence step that governs it. |
| Operations / ATC reviewer | See how TARMAC handles delay, weather, and runway resilience conceptually. | Capacity and delay claims expose their demand basis (peak/IMC vs average/VMC) and evidence level, not just an aggregate score. |
| Slot / hub stakeholder | See whether slot, gate, and fortress-hub control are represented honestly. | Access and entry assumptions are explicit and priced, not assumed free. |
| Regional / community reviewer | See small-community access and noise/emissions exposure before a project is promoted. | Access and environmental claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, scale, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

TARMAC will be a data corpus, review system, and research/design process for Aviation 2.0,
with an implementation built later from accepted VTRACE work packages. It is **multi-scale
by design**: the same corpus, dimension pool, and tier model apply to a single airport, a
state system, a national hub network, or an international route map, and a run targets a
stated scale. Work happens inside a dirty portfolio checkout, so repo-local changes must
stay scoped and must not depend on TRACKER-relative paths for build correctness. TARMAC is
not yet a TRACKER submodule until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE anchor that
later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) and the scale frame (NEED-008) extend the portfolio pattern
shared with ROUTE, PYLON, GAUGE, BASIN, and PACKET: aviation is a tiered SLA system
(gateway hub to GA field) that, like water and internet, must be analyzed at whatever scale
(local to international) the question demands.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | TARMAC public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable airport/route/network identity; carriers, flight numbers, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable physical identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | TARMAC implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | TARMAC must not claim construction readiness, capacity/safety validity of record, slot/route determination, or official agency/airline endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |
| CON-007 | Every claim must declare its scale, and scores/tiers/gaps must not be compared or aggregated across scales without an explicit, labelled cross-scale note. | Prevents misleading mixing of local and national/international evidence (NEED-008). | accepted |

## Non-Goals

- TARMAC is not an engineering study, airspace/procedure design, or environmental review.
- TARMAC is not a slot allocation, route award, or regulatory determination.
- TARMAC is not an advocacy brief for a specific airport, airline, or policy.
- TARMAC does not predict what the FAA, airports, or airlines will build or fly.
- TARMAC does not treat illustrative maps or heuristic forecasts as proof-grade evidence
  unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, tiered SLAs, and multi-scale applicability. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals, and names the multi-scale rule. | pass |
| Citation Auditor | Mission makes no quantitative claims; source links are repo-local context artifacts. | pass |
| Numeracy Checker | Mission contains no arithmetic, delay, capacity, or cost claims. | pass |
| Aviation System Planner | Mission names connectivity, tiering, multi-scale, and public-interest intent. | pass |
| Operations & ATC Officer | Mission requires demand-basis framing for capacity/delay (NEED-002/003). | pass |
| Slot-and-Hub Realist | Initial draft underplayed slot/gate/fortress-hub control; resolved by adding the Slot/hub user lens and CON-006 determination boundary. | resolved |
| Regional-Access & Environmental advocates | Mission names small-community access and noise/emissions as first-class via users and NEED-002. | pass |

Fixed-point note: one actionable finding (slot/gate/fortress-hub control under-represented)
was raised and applied. No unresolved critical or major finding remains. Deferred:
dimension pool, scoring rubric, tier SLA thresholds, demand methodology, and the
scale-tagging schema to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`
