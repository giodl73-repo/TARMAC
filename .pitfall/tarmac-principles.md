# TARMAC Principles

These entries summarize durable TARMAC decision rules for aviation-network evidence, scale,
demand basis, authority boundaries, null results, and public readiness.

## TARMAC-P-01: Air-Network Service Is An Access Promise

**Status:** ACTIVE

**Statement:** TARMAC evaluates connectivity, delay exposure, tier service, and regional
access as network promises, not as isolated airport facts.

**Rationale:** An airport can look adequate by capacity or connectivity while the route
network, tier SLA, delay resilience, or regional access promise fails.

**Decision rule:** Any promoted adequacy claim must name the relevant score, tier/SLA
posture, demand basis, scale, market, and evidence label.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/MISSION.md`, and
`docs/vtrace/REQUIREMENTS.md`.

## TARMAC-P-02: Operating Basis Stays Attached

**Status:** ACTIVE

**Statement:** Capacity, delay, and SLA claims must preserve whether they are based on
peak/average demand and IMC/VMC weather conditions.

**Rationale:** Average or fair-weather evidence can hide peak-load and weather-constrained
failure modes.

**Decision rule:** A score, tier/SLA finding, or public summary that uses capacity or delay
must keep demand and weather basis explicit.

**Evidence:** `CLAUDE.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, `crates/tarmac-network`, and
`crates/tarmac-tier`.

## TARMAC-P-03: Scale And Market Define The Run

**Status:** ACTIVE

**Statement:** International, national, regional, and local aviation-network runs must not
be compared or aggregated unless the comparison basis is explicit.

**Rationale:** Airport systems, route markets, hub catchments, and regional-access networks
answer different questions and operate under different authority.

**Decision rule:** Corpus entries, scores, gaps, tier/SLA results, concepts, and public
summaries must carry scale and market before promotion.

**Evidence:** `PRODUCT_PLAN.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, `crates/tarmac-corpus`, and
`crates/tarmac-gap`.

## TARMAC-P-04: Gaps Are Found, Not Invented

**Status:** ACTIVE

**Statement:** Adequate markets may produce null results, and TARMAC must preserve that
outcome instead of manufacturing a design opportunity.

**Rationale:** The method becomes advocacy if every run must discover a project or
intervention.

**Decision rule:** If corpus, score, tier/SLA, and gap evidence do not support a project,
record a null result or held finding instead of manufacturing a gap.

**Evidence:** `CLAUDE.md`, `docs/vtrace/VERIFICATION.md`, and `crates/tarmac-gap`.

## TARMAC-P-05: Public Authority Remains External

**Status:** ACTIVE

**Statement:** TARMAC can produce research tooling and conceptual design evidence, but it
cannot become an airspace procedure, slot allocation, airport-authority decision,
environmental review, route award, or FAA/ICAO/airline endorsement.

**Rationale:** Aviation-network changes are governed by regulators, airport authorities,
airlines, slot coordinators, environmental processes, and safety procedures outside TARMAC.

**Decision rule:** First-public-run and customer-facing language must keep authority,
procedure, allocation, approval, and endorsement claims held unless separately source-backed
and reviewed by the proper authority.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`, and
`docs/adoption/first-public-run-worksheet.md`.

