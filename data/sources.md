# Source registry

`bts-transtats-arrivals-2026-10-24`: U.S. Department of Transportation, Bureau of
Transportation Statistics, [TranStats Flight Delays at a Glance](https://transtats.bts.gov/homedrillchart.asp).
Frozen HTTPS response, January–August 2017–2026, All Airports, All Carriers,
Year-To-Date, Arrivals. National reporting operating carrier aggregate. Exact
bytes, acquisition time and SHA-256: [custody](bts/custody.json).

The same eight-month window is retained for every year. This is not full-year
data, all aviation, passengers, airport/route evidence or a constant carrier
cohort. The original coverage note names varying carrier counts only through
2021; later counts remain unassessed. Arrival delay threshold is 15 minutes or
more. No attribution to weather/ATC, controlled performance ranking, passenger
itinerary, community access, capacity, fare or project finding follows.

Original federal statistical HTML is preserved unchanged for reproducibility.
No agency logo or external media is copied into the task. Attribution is retained;
no BTS/DOT endorsement is claimed. The live upstream can change after capture.
`python tools/bts_source.py --write` deterministically derives ten aggregate rows
and source notes with the Python standard library. Without `--write` it verifies
the original hash, selected filters, all rows, counts and normalized JSON.
Rust embeds that frozen normalized file, validates counts and published precision,
and computes signed differences. Published rates remain distinct from counts.
The independent synthetic graph has no calibration or data join to these rows.
