# Verification Plan

## Scope

Repo: TARMAC

VTRACE adoption scope: define verification methods and command levels for TARMAC's
requirements. TARMAC now has a fixture-backed Rust workspace and CLI, so implementation
checks are recorded against repo fixtures and unit tests. A fixture pass is not a public
aviation-network finding; public claims still require a cited corpus run and role review.

## Verification Matrix

| Req ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | inspection / demonstration | `cargo test --workspace --locked`; `tarmac-cli corpus` fixtures | a documented regeneration path with labels + scale preserved | passed_fixture | EVID-001 |
| REQ-002 | inspection / review | `cargo test --workspace --locked` corpus label tests | every material quantity carries an evidence label | passed_fixture | EVID-002 |
| REQ-003 | citation audit | `data/sources.md` + corpus fixtures | every cited quantity resolves to a registry source or is labelled | passed_fixture | EVID-003 |
| REQ-004 | schema check / inspection | `cargo test --workspace --locked` identity/schema tests | stable airport/route/network id present; labels are not keys | passed_fixture | EVID-004 |
| REQ-005 | gate / data inspection | `cargo test --workspace --locked` hold/reject tests | such rows held, not promoted | passed_fixture | EVID-005 |
| REQ-006 | calibration record | `cargo test --workspace --locked` score/rubric tests | rubric changes are versioned and justified | passed_fixture | EVID-006 |
| REQ-007 | analysis / inspection | `cargo test --workspace --locked` demand-basis tests | peak-vs-average and IMC-vs-VMC basis named on each claim | passed_fixture | EVID-007 |
| REQ-008 | gap inspection / review | `cargo test --workspace --locked` null-result tests | null result recorded, no manufactured gap | passed_fixture | EVID-008 |
| REQ-009 | review inspection | confirm parliament + editorial gate ran on a promoted claim | review records exist with dispositions | pass_with_risk | EVID-009 / EVID-PF-05 (panel exists; first public run release boundary blocks promotion until exercised) |
| REQ-010 | role review | confirm demand/capacity/delay/resilience/access/competition/environment/cost/slot lenses represented | stakeholder lenses present in `.roles/` and applied | pass_with_risk | EVID-010 / EVID-PF-05 (`.roles/` panel built and boundary names required roles) |
| REQ-011 | editorial review | inspect public claims for scope boundary | outputs framed as research/tooling/conceptual design | pass_with_risk | EVID-011 / EVID-PF-05 (`README`/adoption boundary blocks public finding promotion) |
| REQ-012 | git inspection | `git status --short`; confirm no TRACKER pointer touched | TARMAC changes stay in the child repo | passed | EVID-012 |
| REQ-013 | wave ledger / review | inspect wave ledger + pulses for one-at-a-time discipline | each VTRACE stage settled to a fixed point in sequence | passed | EVID-013 |
| REQ-014 | schema check / inspection | `cargo test --workspace --locked` tier tests | every element classified T1–T4 with declared SLA | passed_fixture | EVID-014 |
| REQ-015 | gate / gap inspection | `cargo test --workspace --locked` tier-SLA gap tests | tier-SLA shortfalls reported before adequacy claimed | passed_fixture | EVID-015 |
| REQ-016 | schema check / gate | `cargo test --workspace --locked` scale-filter tests | every element scale-tagged; cross-scale notes explicit | passed_fixture | EVID-016 |
| REQ-DOC-001 | doc QA | `proof check .` | markdown QA clean across repo docs | passed | EVID-DOC-001 |

## Commands

```powershell
# Doc QA (active now)
proof check .
git diff --check

# Implementation levels
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p tarmac-cli -- --help

# Public-claim level
# tarmac corpus ... ; tarmac tier-sla ... ; tarmac gap --scale national
# role review on the cited output packet
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast doc/sanity for the active VTRACE stage. | `proof check .`, `git diff --check` | passed |
| L1 | Full repo confidence before push. | L0 + `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --locked`, `cargo run -p tarmac-cli -- --help` | passed |
| L2 | Readiness proof before a public claim. | cited corpus regeneration + `tier-sla --gate` + scale-filtered gap + role review + first public run release boundary | pass_with_risk (fixture baseline exists; public finding promotion blocked until cited run evidence and role dispositions are recorded) |

## Evidence Ledger

| Evidence ID | Type | Path / Command | Covers | Result |
|---|---|---|---|---|
| EVID-DOC-001 | report | `proof check .` (0 errors) | REQ-DOC-001 | passed |
| EVID-012 | command | `git status --short` (standalone child repo) | REQ-012 | passed |
| EVID-013 | review | `context/waves/2026-06-26-vtrace-foundation/` ledger + pulses | REQ-013 | passed |
| EVID-009..011 | review | `.roles/` panel present and applied in stage reviews | REQ-009/010/011 | pass_with_risk |
| EVID-PF-05 | policy check | `tests/check-first-public-run-release-boundary.ps1` plus `docs/adoption/first-public-run-release-boundary.md` | REQ-009/010/011 and `TARMAC-PF-05` | passed |
| EVID-CR-001 | command | `cargo fmt --all -- --check` | CR-005 | passed |
| EVID-CR-002 | command | `cargo clippy --workspace --all-targets -- -D warnings` | CR-005/006 | passed |
| EVID-CR-003 | command | `cargo test --workspace --locked` (34 tests) | REQ-001..008/014/015/016, CR-004 | passed_fixture |
| EVID-CLI-001 | command | `cargo run -p tarmac-cli -- --help` | IF-006, REQ-001 | passed |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| No cited public aviation-network corpus has completed an L2 run. | Fixture evidence can be mistaken for a real market finding. | mitigated by first public run release boundary; keep public claims gated behind cited corpus output + role review |
| Review gate not yet exercised on a real corpus claim. | REQ-009/010/011 are process-verified, not outcome-verified. | `REQ-009..011 remain` pass_with_risk until first corpus entry records role dispositions |

## Role Review Notes

| Role Lens | Verification Impact | Disposition |
|---|---|---|
| V&V lens | Methods are credible and mapped 1:1 to requirements; unrun checks are `pending`, not faked. | pass |
| Citation Auditor | Evidence pointers are real (commands run) or explicitly future. | pass |
| Numeracy Checker | The one quantity (0 errors) is a real command result. | pass |
| Scope Keeper | Verification stays at method/result level; REQ-016 scale check named. | pass |

Fixed-point note: PITFALL review found stale greenfield wording after the implementation
baseline landed. This revision separates fixture-backed implementation evidence from the
still-pending public aviation-network evidence.
