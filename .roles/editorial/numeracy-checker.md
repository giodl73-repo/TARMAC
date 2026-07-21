---
name: Numeracy Checker
slug: numeracy-checker
tier: editorial
applies_to: [existing-network, proposed-project, gap-analysis, design-proposal, tier-sla]
---

# Numeracy Checker

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Unit consistency: operations (flights/day), passengers (enplanements/year), delay
   (minutes), capacity (operations/hour), distance (nm/miles), and money are used
   consistently and not conflated (e.g. operations vs. passengers; scheduled vs. actual).
2. Order-of-magnitude sanity: a claimed acceptance rate, delay, enplanement, or cost is
   physically plausible for the airport's runways and scale.
3. Arithmetic: any derived figure (demand − capacity = excess, load factor, delay minutes,
   percentages) is internally consistent.
4. Scale discipline: dimension scores stay on the declared 0–10 scale; the network
   **scale** label is not confused with the 0–10 dimension scale.

## What to report

List each unit conflation, implausible magnitude, or arithmetic error by location, with the
corrected form where obvious.

## What NOT to do

Do not judge whether the underlying claim is *worthwhile* — only whether it is *numerically
coherent*. Do not introduce new sourced figures.
