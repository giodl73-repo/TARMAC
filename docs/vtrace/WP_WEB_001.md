# WP-WEB-001 — Synthetic regional route-connectivity task

Status: implemented; local exit verified, 2026-10-23. Owner objective:
deliver all sixteen TAXLANE satellites, including software publication.

Select a fictional regional airport, change a route's operations-per-day input
or disable a route, and compare degree, degree centrality, incident operations
and graph connectivity to a named hub using unchanged tarmac-network methods.
All routes use one declared Peak demand basis. Regional scale and fictional
Cedar market are fixed; no cross-scale or Peak/Average mixing.

This is an undirected topological illustration, not an itinerary, flight
schedule, missed-connection, delay, runway/airspace capacity or reliability
forecast. Slots/gates, fares, weather performance, small-community actual
access, ground access, emissions/noise and delay remain unassessed/null.
No slot allocation, route award, operating disruption, engineering, savings,
fiscal-admission or tax-rate authority. Rating/enablement edits are illustrative
graph scenarios, not proposed infrastructure or airline service decisions.

Add a standalone tarmac-web adapter and native replay with strict typed JSON,
unknown-field rejection, <=64 KiB, 1–16 airports, <=32 routes, unique short safe
identifiers, finite positive bounded operations, valid endpoints and a typed
Hub target. Reject unknown selected airports before native zero fallback.
No business arithmetic copied to JS; no cross-repo runtime dependency. Local
worker only, no upload/storage/person records. Missing reset observations
stay explicitly null. Preserve every airport/route and independent unknown.

Paths: manifests/toolchain/lock, crates/tarmac-web, synthetic fixtures, web,
build/CI tooling, locked browser packages/tests, README, this contract and local
wave. Exit: native workspace fmt/check/clippy/tests; actual-WASM/native byte
parity for reset/route-loss/changed examples; browser controls, topology and
centrality/incident/connectivity observations, independent nulls, validation,
reset/download, worker loading/failure, keyboard/mobile/prefix; <5 MB artifact
with source, fixtures, WASM, licenses and exact-commit provenance. PR checks
precede master-only deploy; verify live artifact and TAXLANE roundtrip before
pointer snapshot. Rollback is revert and previous Pages artifact.

## Role Review Notes

Implementing-agent role lenses, no independent endorsement. Seven Parliament
voices precede editorial review:

- Aviation system planner: pass for declared regional topology, no network promise.
- Airport/civil engineer: pass with no runway/airspace throughput inference.
- Operations/ATC officer: pass with Peak basis and delay/weather left null.
- Aviation economist: pass without fare/demand/benefit-cost claim; defer actual
  economics to PRODUCT_PLAN's first cited end-to-end aviation run.
- Regional-access advocate: pass with every fictional community visible and
  actual access/itinerary unknown; defer real service adequacy to that cited run.
- Environmental/community advocate: pass with noise/emissions unknown, deferred
  to that cited run before any real project promotion.
- Slot/fortress-hub realist: pass with slots/gates unassessed and no assumed
  permission to fly; defer market-entry evidence to that cited run.
- Citation auditor: pass with simulated labels and fixture/build custody.
- Scope keeper: pass for regional graph, no engineering, schedule or award.
- Numeracy checker: pass for same-basis operations/day and native metrics.

Regional traveler, small-community resident, airline, airport-authority and
cargo-shipper lenses retain separate actual access, fare, capacity, reliability
and intermodal unknowns, deferred to the cited run. No critical/major actionable
finding remains in this contract. Repeat role review at implementation exit;
advance no canonical research or source-backed project hold. Commit/push child
before scoped TRACKER pointer update.

## Implementation exit

All 37 native workspace tests, format and strict native Clippy pass. Seven
actual-WASM Chromium cases pass: native canonical parity for reset/route-loss/
changed examples, route controls, airport/hub selection, topology and all four
metrics, independent nulls and closed authority in downloads, unknown/missing/
mixed/oversized inputs, stale-result clearing, reset, loading/worker failure,
keyboard/mobile/prefix, an added airport with null baseline, and singleton-hub
centrality remaining null. Desktop and 390px screenshots inspected. Artifact is
approximately 385 KB, including full upstream notices, below 5 MB.

All seven Parliament and three editorial lenses repeat pass for this regional
simulation. Resolved findings: prohibit parallel aggregate routes so normalized
degree does not count duplicate airport pairs; distinguish topology and incident
operations from actual itineraries/service; keep undefined centrality and absent
reset observations null; retain all seven independent unknowns. Stakeholder
defers remain with the first cited end-to-end aviation run. No critical/major
actionable finding remains; no independent panel is claimed. Hosted native/WASM
checks gate publication; TRACKER records exact live artifact and navigation.
