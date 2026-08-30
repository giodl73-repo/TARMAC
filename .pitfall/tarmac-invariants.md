# TARMAC Invariants

These entries summarize properties that must remain true for TARMAC corpus, score,
tier/SLA, gap, review, and public-claim artifacts.

## TARMAC-I-01: Labels, Basis, And Scale Survive The Pipeline

**Status:** VERIFIED

**Claim:** Evidence labels, source IDs, demand basis, scale tags, market, tier, and SLA
fields are preserved from corpus through score, tier/SLA, gap, and CLI artifacts.

**Why it matters:** TARMAC claims are only auditable if source custody, operating basis, and
scale context survive every transformation.

**Enforcement:** Corpus validation, CR-007, and workspace tests preserve labels, basis, and
scale through the pipeline.

**Evidence:** `docs/vtrace/CODE_RIGOR.md`, `docs/vtrace/VERIFICATION.md`,
`crates/tarmac-corpus`, `crates/tarmac-score`, `crates/tarmac-tier`, and
`crates/tarmac-gap`.

## TARMAC-I-02: Operating Basis Is Typed

**Status:** VERIFIED

**Claim:** Capacity, delay, and tier/SLA conformance distinguish peak/average demand and
IMC/VMC weather instead of leaving operating context in prose.

**Why it matters:** Average/VMC conditions can understate delay, capacity, resilience, and
tier-service risk during peak or weather-constrained operation.

**Enforcement:** `tarmac-network` types demand basis, `tarmac-tier` consumes it for
conformance, and tests preserve the distinction.

**Evidence:** `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/CODE_RIGOR.md`,
`crates/tarmac-network`, and `crates/tarmac-tier`.

## TARMAC-I-03: Scale Filtering Is Fail-Closed

**Status:** VERIFIED

**Claim:** Gap analysis excludes other-scale entries unless an explicit cross-scale marker
or comparison basis is present.

**Why it matters:** Cross-scale mixing can make local access failures, regional service
gaps, or national hub concentration look stronger or weaker than the evidence supports.

**Enforcement:** Typed `Scale`, corpus validation, and `tarmac-gap` tests enforce
within-scale analysis.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `crates/tarmac-corpus`, and
`crates/tarmac-gap`.

## TARMAC-I-04: Null Results Are Valid Outputs

**Status:** VERIFIED

**Claim:** An already adequate aviation market can produce a labelled null result, and
TARMAC does not fabricate a gap when evidence does not support one.

**Why it matters:** Public aviation analysis becomes advocacy if the tool must always find
a deficiency or intervention.

**Enforcement:** REQ-008, gap tests, and review roles keep null results first-class.

**Evidence:** `docs/vtrace/TRACE.md`, `docs/vtrace/VERIFICATION.md`, and
`crates/tarmac-gap`.

## TARMAC-I-05: Public Claims Require Role Review

**Status:** PARTIAL

**Claim:** Every promotable corpus entry, gap finding, design proposal, tier/SLA definition,
or public result runs through the role review and scope boundary checks before downstream
use.

**Why it matters:** TARMAC has the role panel and process, but the gate is not
outcome-verified until the first cited public run exists.

**Enforcement:** `.roles/ROLE.md`, VTRACE REQ-009..011, and adoption worksheets require the
gate before public promotion.

**Evidence:** `.roles/ROLE.md`, `docs/vtrace/VERIFICATION.md`, and
`docs/adoption/first-public-run-worksheet.md`.

