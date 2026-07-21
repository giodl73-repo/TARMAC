# Pulse 04: SPECIFICATION_BASELINE

Settled. Authored `docs/vtrace/SPECIFICATION_BASELINE.md` (target/provisional): the **scale
model** (SCALE-01..03), the aviation dimension pool `DIM-01..13`, the demand basis
(SPEC-DB-01/02: peak/average + IMC/VMC), the **T1–T4 tier model** (International Gateway Hub
→ National Hub → Regional → Local/GA) with capacity/delay/connectivity/access SLA terms,
controlled specs `SPEC-001..013`, public contracts `IF-001..004`, and the spec gate
(`pass_with_risk`).

Role-review fixed point (real `.roles`): one finding applied — capacity (DIM-02) risked
reading as slot-free, resolved by keeping slot constraint (DIM-09) in the capacity SLA term
+ SPEC-007. Pool weights, SLA thresholds, and scale nesting (DEF-005) explicitly provisional.
Next: TRACE.
