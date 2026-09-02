$ErrorActionPreference = 'Stop'

function Assert-Contains {
  param(
    [string]$Path,
    [string]$Needle
  )

  $text = Get-Content -Raw -LiteralPath $Path
  if ($text.IndexOf($Needle, [StringComparison]::Ordinal) -lt 0) {
    throw "Missing expected text in ${Path}: ${Needle}"
  }
}

Assert-Contains 'README.md' 'first public run release boundary'
Assert-Contains 'README.md' 'TARMAC-PF-05'
Assert-Contains 'README.md' 'fixture-backed or scoping evidence only'

Assert-Contains 'docs/adoption/README.md' 'first public run release boundary'
Assert-Contains 'docs/adoption/README.md' 'Aviation System Planner'
Assert-Contains 'docs/adoption/README.md' 'Slot & Fortress-Hub Realist'

Assert-Contains 'docs/adoption/first-public-run-worksheet.md' 'Release Boundary'
Assert-Contains 'docs/adoption/first-public-run-worksheet.md' 'TARMAC-PF-05'
Assert-Contains 'docs/adoption/first-public-run-worksheet.md' 'parliament and editorial dispositions'

Assert-Contains 'docs/adoption/first-public-run-release-boundary.md' '`TARMAC-PF-05`'
Assert-Contains 'docs/adoption/first-public-run-release-boundary.md' 'source manifest path and regeneration command'
Assert-Contains 'docs/adoption/first-public-run-release-boundary.md' 'If any field is missing'

Assert-Contains 'docs/vtrace/VERIFICATION.md' 'first public run release boundary'
Assert-Contains 'docs/vtrace/VERIFICATION.md' 'TARMAC-PF-05'
Assert-Contains 'docs/vtrace/VERIFICATION.md' 'REQ-009..011 remain'

Assert-Contains '.roles/ROLE.md' '## PITFALL gates'
Assert-Contains '.roles/ROLE.md' '`TARMAC-PF-05`'
Assert-Contains '.roles/ROLE.md' 'Aviation System Planner; Airport / Civil Engineer; Operations & ATC Reliability Officer; Aviation Economist; Regional-Access Advocate; Environmental & Community Advocate; Slot & Fortress-Hub Realist; Citation Auditor; Scope Keeper; Numeracy Checker'

Assert-Contains '.pitfall/tarmac-pitfalls.md' '**Status:** MITIGATED'
Assert-Contains '.pitfall/tarmac-pitfalls.md' 'tests/check-first-public-run-release-boundary.ps1'
Assert-Contains '.pitfall/tarmac-invariants.md' 'TARMAC-I-06'
Assert-Contains '.pitfall/tarmac-invariants.md' 'Fixture Evidence Is Not Public Aviation Evidence'
Assert-Contains '.pitfall/tarmac-invariants.md' 'TARMAC-PF-05'

Write-Host 'TARMAC first public run release boundary check passed.'
