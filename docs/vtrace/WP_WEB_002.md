# WP-WEB-002 — Frozen BTS reporting-carrier arrival comparison

Status: accepted for implementation, 2026-10-24. Owner instruction: continue
TAXLANE satellite source-evidence follow-through after the SLATE release.

Compare two named years from the frozen BTS TranStats Flight Delays at a Glance
summary, January–August 2017–2026. Captured filters: All Airports, All Carriers,
Year-To-Date, Arrivals. National reporting-carrier aggregate, not all aviation,
not airport/route microdata, not passengers or a constant carrier cohort. Preserve
the exact HTML and selected filters, ten rows, source notes, raw counts and
published hundredth-percent rates. Counts reconcile four outcome categories to
flight operations. Report signed count changes and published percentage-point
changes without annualizing, attributing causes or inferring service adequacy.
Source notes define the 15-minute delay threshold and warn that reporting carrier
counts vary; their historical coverage note ends in 2021. Do not invent later
carrier counts or describe these comparisons as controlled performance changes.

Strict closed query: view=bts-arrival-comparison, from_year and to_year from the
frozen set. Reject extra fields, unknown years and >64 KiB. Rust validates the
embedded source, reconciles counts and computes differences. JS only orchestrates
and renders. A separate worker/task shares the raw-ABI WASM, with no runtime
dependency on SLATE or another product. The existing fictional regional graph
and its library semantics remain independent. No source rows become graph inputs.

Expose explicit nulls for airport/route performance, passenger itinerary,
community access, weather/ATC attribution, slots/gates, fares and environmental
effects. Authority for engineering, allocation, rights, savings and rates remains
false. No canonical dimension/tier/gap finding, intervention or fiscal promotion.
No person-level records, uploads or persistence. Original HTML is a download,
never embedded or executed by the task. Frozen normalized JSON, custody, license,
WASM and exact clean-build provenance accompany the release, under 5 MB.

Exit: deterministic extraction and raw hash verification; native tests for
reconciliation, published precision, pandemic-year contrast, reversed/same-year
queries, unknown/extra/malformed/oversized input and independent nulls; canonical
native/WASM parity; source controls, stale clearing, downloads/reset, failure,
keyboard/mobile/prefix, graph regressions; format/check/strict native and target
Clippy; hosted CI; exact live artifact and TAXLANE transportation roundtrip before
TRACKER snapshot. Revert/previous artifact is rollback.

## Role Review Notes

Implementing-agent lenses, not an independent expert panel. Parliament first:

- Aviation system planner: pass for national reporting-carrier source exploration;
  defer real regional connectivity/service findings to the cited network run.
- Airport engineer: pass with runway/gate/airspace capacity unassessed; no ops/day
  or Peak/IMC conversion of the eight-month counts.
- Operations/ATC officer: pass with exact arrival window and threshold; defer
  weather/ATC causality and recovery evidence to the cited network run.
- Aviation economist: pass with changing coverage visible; no demand, fare,
  fixed-cohort or benefit-cost claim. Defer economics to that run.
- Regional-access advocate: pass with no passenger or community-access inference;
  defer maintained service and itinerary evidence to that run.
- Environmental/community advocate: pass with noise/emissions and neighborhood
  burden null; defer environmental evidence to that run.
- Slot/fortress-hub realist: pass with no slot/gate/entry assumptions; defer
  market-control evidence to that run.

Editorial: citation auditor passes exact capture/registry and derived labels;
scope keeper passes national, eight-month, reporting-carrier-only comparison;
numeracy checker passes integer count differences and hundredth-point published
rate differences, never sum/average rates. Traveler, small-community, airline,
airport-authority and cargo lenses retain distinct itinerary/access/fare/capacity/
freight unknowns for the cited network run. No critical/major actionable contract
finding remains. Repeat these lenses at implementation exit.

## Implementation exit

All 42 native workspace tests, formatting/check and strict native/wasm-target
Clippy pass. Six new source browser cases and all seven graph regressions pass
with actual WASM. Four native source references match executing WASM byte for
byte, alongside the original three graph references. The source artifact is
approximately 628 KB. Original HTML hash, selected filters, ten extracted rows,
published rates and count reconciliation verify. Desktop and 390px mobile
screenshots inspected; no viewport overflow.

All seven Parliament and three editorial lenses repeat pass for this bounded
source-exploration contract. Resolved findings: preserve the eight-month window
in every comparison; retain changing-carrier and post-2021 coverage gaps; do not
join national aggregates into regional routes; separate source/graph statuses
and error clearing; retain signed percentage-point precision. Browser tests
corrected a textContent/innerText assertion mismatch and Windows test-file
encoding; neither altered source data or native arithmetic. All stakeholder
defers remain with the cited network run. No critical/major actionable exit
finding remains; these are implementing-agent reviews, not independent approval.
Hosted checks gate publication. Exact live custody and TAXLANE roundtrip are
recorded in TRACKER after child release, before its scoped pointer snapshot.
