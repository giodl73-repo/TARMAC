# TARMAC — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of corpus
entries, gap findings, design proposals, tier/SLA definitions, and VTRACE deliverables run
against these files and record dispositions (`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is the
output, not consensus. No voice is skipped. A good project survives all seven; a weak one
collapses under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/aviation-system-planner.md` | Aviation System Planner | System fluidity + public interest vs. single-hub framing |
| `parliament/airport-engineer.md` | Airport / Civil Engineer | Buildable runway/airspace capacity vs. schedule-fantasy throughput |
| `parliament/operations-atc-officer.md` | Operations & ATC Reliability Officer | On-time/weather operability vs. over-scheduling optimism |
| `parliament/aviation-economist.md` | Aviation Economist | Benefit-cost + demand vs. forecast inflation/overbuild |
| `parliament/regional-access-advocate.md` | Regional-Access Advocate | Small-community access vs. hub-optimized abandonment |
| `parliament/environmental-community-advocate.md` | Environmental & Community Advocate | Noise/emissions/land vs. throughput expansion |
| `parliament/slot-and-hub-realist.md` | Slot & Fortress-Hub Realist | Slots/gates/hub power vs. free-access & easy-entry assumptions |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, **scale**, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (ops/pax/min/$); magnitudes sane; arithmetic and 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the network serves, used during corpus scoring, gap
analysis, and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/regional-traveler.md` | Regional Traveler | Nonstop access, fare, reliable connections |
| `stakeholders/small-community-resident.md` | Small-Community Resident | Any scheduled service, lifeline access, drive distance |
| `stakeholders/airline.md` | Airline | Slot/gate access, route profitability, reliability |
| `stakeholders/airport-authority.md` | Airport Authority | Capacity, capital funding, community relations |
| `stakeholders/cargo-shipper.md` | Air-Cargo / Logistics Shipper | Freighter access, reliability, intermodal connection |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for TARMAC research outputs. See
`panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from
parliament and editorial.

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or tier/SLA
definition is being settled, the relevant subset of this panel is applied and dispositions
are recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major actionable finding
remains and every deferred item names a later stage or work package.
