# TARMAC Pitfalls

These entries capture recurring Aviation 2.0 failure classes and map them to TARMAC's
existing controls.

## TARMAC-PF-01: Airport Capacity Is Mistaken For Network Adequacy

**Status:** MITIGATED

**Pattern:** Airport capacity, connectivity, or hub role is treated as proof that the whole
route, access, delay, tier/SLA, and market-service promise is adequate.

**Domain:** Corpus entries, DIM scoring, tier/SLA findings, gap analysis, concept notes,
public summaries, and customer handoffs.

**Detection difficulty:** Airport-level capacity and traffic are easy to cite, while route
redundancy, tail risk, regional access, and SLA shortfalls sit across different records.

**Structural solution:** Keep route service, tier/SLA, source labels, demand basis, scale,
and null-result handling attached to every adequacy claim.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/TRACE.md`, and
`crates/tarmac-gap`.

## TARMAC-PF-02: Average/VMC Evidence Hides Peak/IMC Failure

**Status:** MITIGATED

**Pattern:** Average or fair-weather operating evidence is used to describe capacity,
delay, resilience, or adequacy while peak demand or IMC failure remains unresolved.

**Domain:** Network model, corpus quantities, score artifacts, tier/SLA conformance, gap
reports, and first-public-run findings.

**Detection difficulty:** Average/VMC measurements are cleaner and more common, while
peak/IMC constraints may be episodic, operationally sensitive, or recorded elsewhere.

**Structural solution:** Type demand and weather basis, keep basis attached through tier/SLA
and gap analysis, and require operating-basis evidence before promotion.

**Evidence:** `CLAUDE.md`, `docs/vtrace/CODE_RIGOR.md`, `crates/tarmac-network`, and
`crates/tarmac-tier`.

## TARMAC-PF-03: Slot, Procedure, Or Authority Boundaries Are Bypassed

**Status:** MITIGATED

**Pattern:** A network concept is presented as buildable, approved, or operationally valid
because the score/gap artifact looks favorable, while slot, procedure, environmental,
airport-authority, airline, or regulator constraints remain unresolved.

**Domain:** Design proposals, public run reports, source-contribution worksheets, role
review, and public/customer claims.

**Detection difficulty:** Quantified route or access gaps can appear decisive before the
proper aviation authority has acted.

**Structural solution:** Keep public-authority language held, include scope and aviation
operations lenses in review, and require source-backed authority evidence before promotion.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`, and
`docs/adoption/first-public-run-worksheet.md`.

## TARMAC-PF-04: Stale VTRACE Evidence Misstates Readiness

**Status:** MITIGATED

**Pattern:** VTRACE verification, trace, code-rigor, or wave rows continue to say
implementation is pending after the Rust workspace and CLI have been built, or they imply
public validation has happened when only fixture evidence exists.

**Domain:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/TRACE.md`,
`docs/vtrace/CODE_RIGOR.md`, implementation waves, README status, and portfolio reports.

**Detection difficulty:** VTRACE documents are created before code and may stay locally
coherent even after later work packages change the repo state.

**Structural solution:** Update verification, trace, code-rigor, and wave rows when
implementation evidence lands, and keep fixture-backed workspace evidence separate from
first public run validation.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/TRACE.md`,
`docs/vtrace/CODE_RIGOR.md`, and
`context/waves/2026-06-26-tarmac-implementation/WAVE.md`.

## TARMAC-PF-05: Fixture Baseline Becomes Public Aviation Finding

**Status:** MITIGATED

**Pattern:** Passing workspace tests, seed fixtures, source schema, or CLI help are treated
as a cited end-to-end aviation-network finding.

**Domain:** README evidence status, first public run, customer scoping, source manifests,
gap artifacts, and role review.

**Detection difficulty:** The implementation baseline is complete and useful, so it is easy
to overread fixture evidence as external system validation.

**Structural solution:** Keep first-public-run status open until one bounded source-backed
corpus, source manifest, tier/SLA run, scale-filtered gap, findings report, and role review
are published. TARMAC now uses the first public run release boundary and
`tests/check-first-public-run-release-boundary.ps1` to keep fixture-backed
implementation evidence from becoming a public aviation finding until source
manifest, regeneration command, scale, market, operating basis, artifacts, held
claims, role dispositions, and downstream owner acceptance are visible.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`docs/adoption/first-public-run-worksheet.md`,
`docs/adoption/first-public-run-release-boundary.md`,
`docs/vtrace/VERIFICATION.md`, `.roles/ROLE.md`, and
`tests/check-first-public-run-release-boundary.ps1`.
