//! Frozen reporting-carrier counts. No source-to-graph calibration.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const DATA: &str = include_str!("../../../fixtures/bts/summary.json");
const COUNT_FIELDS: [&str; 5] = [
    "on_time",
    "delayed",
    "cancelled",
    "diverted",
    "flight_operations",
];
const RATE_FIELDS: [&str; 3] = [
    "on_time_percent_hundredths",
    "delayed_percent_hundredths",
    "cancelled_percent_hundredths",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    view: String,
    from_year: u16,
    to_year: u16,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Source {
    source_id: String,
    url: String,
    window: String,
    scale: String,
    filters: BTreeMap<String, String>,
    captured_at: String,
    raw_sha256: String,
    notes: Vec<String>,
    rows: Vec<Row>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Row {
    year: u16,
    published: Vec<String>,
    on_time: u64,
    on_time_percent_hundredths: u64,
    delayed: u64,
    delayed_percent_hundredths: u64,
    cancelled: u64,
    cancelled_percent_hundredths: u64,
    diverted: u64,
    flight_operations: u64,
}

fn source(data: &str) -> Result<Source, String> {
    let data: Source = serde_json::from_str(data).map_err(|e| e.to_string())?;
    let filters = BTreeMap::from([
        ("Airport".into(), "All Airports".into()),
        ("Carrier".into(), "All Carriers".into()),
        ("CboTime".into(), "Year-To-Date".into()),
        ("CboSelection".into(), "Arrivals".into()),
    ]);
    if data.source_id != "bts-transtats-arrivals-2026-10-24"
        || data.url != "https://transtats.bts.gov/homedrillchart.asp"
        || data.window != "January-August"
        || data.scale != "national"
        || data.filters != filters
        || data.rows.len() != 10
        || data.notes.len() != 1
        || !data.notes[0].contains("15 minutes")
        || !data.notes[0].contains("17 in 2021")
        || data.raw_sha256.len() != 64
        || !data.raw_sha256.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("Frozen source identity, filters, window or notes differ".into());
    }
    for (index, row) in data.rows.iter().enumerate() {
        let counts = [row.on_time, row.delayed, row.cancelled, row.diverted];
        let rates = [
            row.on_time_percent_hundredths,
            row.delayed_percent_hundredths,
            row.cancelled_percent_hundredths,
        ];
        if row.year != 2017 + index as u16
            || row.flight_operations == 0
            || row.flight_operations > 1_000_000_000
            || counts.iter().any(|n| *n > row.flight_operations)
            || counts.iter().sum::<u64>() != row.flight_operations
            || row.published.len() != 8
        {
            return Err("Frozen source years or counts do not reconcile".into());
        }
        let values = [
            row.on_time,
            rates[0],
            row.delayed,
            rates[1],
            row.cancelled,
            rates[2],
            row.diverted,
            row.flight_operations,
        ];
        for (i, text) in row.published.iter().enumerate() {
            let normalized = text.replace([',', '.', '%'], "");
            if normalized.parse::<u64>().ok() != Some(values[i]) {
                return Err("Published source strings differ from numeric values".into());
            }
        }
        for i in 0..3 {
            if rates[i] > 10_000
                || (counts[i] * 10_000 + row.flight_operations / 2) / row.flight_operations
                    != rates[i]
            {
                return Err("Published rate does not match count precision".into());
            }
        }
    }
    Ok(data)
}

pub fn evaluate(input: &str) -> Result<String, String> {
    let query: Query = serde_json::from_str(input).map_err(|e| e.to_string())?;
    if query.view != "bts-arrival-comparison" {
        return Err("Use view bts-arrival-comparison".into());
    }
    let mut data = source(DATA)?;
    let find = |year| {
        data.rows
            .iter()
            .find(|r| r.year == year)
            .ok_or_else(|| "Choose a frozen year from 2017 through 2026".to_string())
    };
    let from = serde_json::to_value(find(query.from_year)?).map_err(|e| e.to_string())?;
    let to = serde_json::to_value(find(query.to_year)?).map_err(|e| e.to_string())?;
    let mut changes = BTreeMap::new();
    for field in COUNT_FIELDS.into_iter().chain(RATE_FIELDS) {
        changes.insert(
            field,
            to[field].as_i64().ok_or("Invalid count")?
                - from[field].as_i64().ok_or("Invalid count")?,
        );
    }
    data.rows.clear();
    let mut metadata = serde_json::to_value(data).map_err(|e| e.to_string())?;
    metadata
        .as_object_mut()
        .ok_or("Invalid metadata")?
        .remove("rows");
    serde_json::to_string(&json!({
        "schema":"tarmac.bts-arrival-comparison.v1", "evidence":"published-source-aggregate",
        "source": metadata, "from":from, "to":to, "changes":changes,
        "units":{"counts":"flights in January-August; not passengers or ops/day",
                 "rates":"published percent in hundredths", "rate_changes":"hundredths of a percentage point"},
        "reconciliation":{"from_counts_match_operations":true,"to_counts_match_operations":true},
        "comparability":{"same_month_window":true,"constant_carrier_cohort":false,
                         "annualized":false,"carrier_counts_after_2021":Value::Null,
                         "interpretation":"Descriptive changes only. Reporting carriers vary; no causal attribution or controlled performance ranking."},
        "not_assessed":{"airport_route_performance":null,"passenger_itinerary":null,
            "actual_community_access":null,"weather_atc_causality":null,"slots_gates":null,
            "fares_competition":null,"noise_emissions":null},
        "authority":{"engineering":false,"allocation":false,"rights":false,"savings":false,"rate_change":false}
    })).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn query(from: u16, to: u16) -> Value {
        serde_json::from_str(
            &evaluate(
                &json!({"view":"bts-arrival-comparison","from_year":from,"to_year":to}).to_string(),
            )
            .unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn published_counts_precision_and_window_are_retained() {
        let value = query(2024, 2026);
        assert_eq!(value["from"]["flight_operations"], 4_714_957);
        assert_eq!(value["to"]["flight_operations"], 4_728_732);
        assert_eq!(value["changes"]["flight_operations"], 13_775);
        assert_eq!(value["changes"]["cancelled"], 24_173);
        assert_eq!(value["changes"]["on_time_percent_hundredths"], -78);
        assert_eq!(value["source"]["window"], "January-August");
    }
    #[test]
    fn pandemic_year_is_descriptive_not_a_cohort_or_access_claim() {
        let value = query(2019, 2020);
        assert_eq!(value["to"]["cancelled"], 271_488);
        assert_eq!(value["to"]["cancelled_percent_hundredths"], 828);
        assert_eq!(value["comparability"]["constant_carrier_cohort"], false);
        assert!(value["comparability"]["carrier_counts_after_2021"].is_null());
        assert!(value["not_assessed"]
            .as_object()
            .unwrap()
            .values()
            .all(Value::is_null));
        assert!(value["authority"]
            .as_object()
            .unwrap()
            .values()
            .all(|v| v == false));
    }
    #[test]
    fn reversed_and_same_year_changes_are_valid() {
        let a = query(2024, 2026);
        let b = query(2026, 2024);
        let same = query(2020, 2020);
        for field in COUNT_FIELDS.into_iter().chain(RATE_FIELDS) {
            assert_eq!(
                a["changes"][field].as_i64().unwrap(),
                -b["changes"][field].as_i64().unwrap()
            );
            assert_eq!(same["changes"][field], 0);
        }
    }
    #[test]
    fn rejects_unknown_years_fields_types_and_views() {
        for input in [
            r#"{"view":"bts-arrival-comparison","from_year":2016,"to_year":2026}"#,
            r#"{"view":"bts-arrival-comparison","from_year":2024,"to_year":2026,"airport":"ATL"}"#,
            r#"{"view":"bts-arrival-comparison","from_year":null,"to_year":2026}"#,
            r#"{"view":"other","from_year":2024,"to_year":2026}"#,
        ] {
            assert!(super::super::evaluate(input).is_err());
        }
        assert!(super::super::evaluate(&" ".repeat(65_537)).is_err());
    }
    #[test]
    fn rejects_corrupted_source_counts_rates_and_filters() {
        for changed in [
            DATA.replace("4714957", "4714958"),
            DATA.replace("7528", "7529"),
            DATA.replace("All Airports", "ATL"),
        ] {
            assert!(source(&changed).is_err());
        }
        assert!(source(DATA).is_ok());
    }
}
