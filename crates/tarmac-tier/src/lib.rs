use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub fn label(&self) -> &'static str {
        match self {
            Tier::T1 => "T1",
            Tier::T2 => "T2",
            Tier::T3 => "T3",
            Tier::T4 => "T4",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sla {
    pub ops_capacity: f64,
    pub delay_minutes: f64,
    pub availability: f64,
    pub affordability: f64,
}

pub fn provisional_sla(tier: Tier) -> Sla {
    match tier {
        Tier::T1 => Sla {
            ops_capacity: 400.0,
            delay_minutes: 5.0,
            availability: 0.999,
            affordability: 0.95,
        },
        Tier::T2 => Sla {
            ops_capacity: 250.0,
            delay_minutes: 10.0,
            availability: 0.99,
            affordability: 0.9,
        },
        Tier::T3 => Sla {
            ops_capacity: 120.0,
            delay_minutes: 20.0,
            availability: 0.97,
            affordability: 0.85,
        },
        Tier::T4 => Sla {
            ops_capacity: 50.0,
            delay_minutes: 40.0,
            availability: 0.95,
            affordability: 0.8,
        },
    }
}

pub fn classify(entry: &tarmac_corpus::CorpusEntry) -> Tier {
    match entry.tier.as_deref() {
        Some("T1") => Tier::T1,
        Some("T2") => Tier::T2,
        Some("T3") => Tier::T3,
        Some("T4") => Tier::T4,
        _ => Tier::T4,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dim13 {
    pub score: tarmac_score::Score,
    pub basis: tarmac_corpus::DemandBasis,
    pub redundancy: bool,
}

pub fn conformance(entry: &tarmac_corpus::CorpusEntry, network: &tarmac_network::Network) -> Dim13 {
    let required = provisional_sla(classify(entry));

    let observed_ops = entry
        .quantities
        .iter()
        .find(|q| q.unit.to_lowercase().contains("ops"))
        .map(|q| q.value)
        .unwrap_or(0.0);

    let redundancy = match network.degree(&entry.id) {
        Some(d) => d >= 2,
        None => false,
    };

    let mut result = (observed_ops / required.ops_capacity).min(1.0) * 10.0;
    if !redundancy {
        result -= 2.0;
    }

    Dim13 {
        score: tarmac_score::Score::clamped(result),
        basis: tarmac_corpus::DemandBasis::Peak,
        redundancy,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TierSlaGap {
    pub entry_id: String,
    pub tier: Tier,
    pub required_ops: f64,
    pub observed_ops: f64,
    pub label: tarmac_corpus::EvidenceLabel,
}

pub fn tier_sla_gap(entry: &tarmac_corpus::CorpusEntry) -> Option<TierSlaGap> {
    let tier = classify(entry);
    let required = provisional_sla(tier);

    let observed_ops = entry
        .quantities
        .iter()
        .find(|q| q.unit.to_lowercase().contains("ops"))
        .map(|q| q.value)
        .unwrap_or(0.0);

    if observed_ops < required.ops_capacity {
        Some(TierSlaGap {
            entry_id: entry.id.clone(),
            tier,
            required_ops: required.ops_capacity,
            observed_ops,
            label: tarmac_corpus::EvidenceLabel::Provisional,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_network() -> tarmac_network::Network {
        let mut net = tarmac_network::Network::new();
        net.add_airport(tarmac_network::Airport {
            id: String::from("A"),
            name: String::from("A"),
            role: tarmac_network::AirportRole::Hub,
        })
        .unwrap();
        net.add_airport(tarmac_network::Airport {
            id: String::from("B"),
            name: String::from("B"),
            role: tarmac_network::AirportRole::Regional,
        })
        .unwrap();
        net.add_airport(tarmac_network::Airport {
            id: String::from("C"),
            name: String::from("C"),
            role: tarmac_network::AirportRole::Regional,
        })
        .unwrap();
        net.add_route(
            "A",
            "B",
            tarmac_network::Route {
                id: String::from("r1"),
                ops_per_day: 100.0,
                basis: tarmac_network::DemandBasis::Peak,
            },
        )
        .unwrap();
        net.add_route(
            "B",
            "C",
            tarmac_network::Route {
                id: String::from("r2"),
                ops_per_day: 50.0,
                basis: tarmac_network::DemandBasis::Peak,
            },
        )
        .unwrap();
        net.add_route(
            "A",
            "C",
            tarmac_network::Route {
                id: String::from("r3"),
                ops_per_day: 70.0,
                basis: tarmac_network::DemandBasis::Peak,
            },
        )
        .unwrap();
        net
    }

    fn conforming_entry() -> tarmac_corpus::CorpusEntry {
        tarmac_corpus::CorpusEntry {
            id: String::from("A"),
            tier: Some(String::from("T1")),
            quantities: vec![tarmac_corpus::Quantity {
                value: 500.0,
                unit: String::from("ops"),
                label: tarmac_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        }
    }

    #[test]
    fn classify_maps_tier_strings_and_defaults() {
        let mut e = tarmac_corpus::CorpusEntry {
            tier: Some(String::from("T1")),
            ..Default::default()
        };
        assert_eq!(classify(&e), Tier::T1);

        e.tier = Some(String::from("T2"));
        assert_eq!(classify(&e), Tier::T2);

        e.tier = Some(String::from("T3"));
        assert_eq!(classify(&e), Tier::T3);

        e.tier = Some(String::from("T4"));
        assert_eq!(classify(&e), Tier::T4);

        e.tier = None;
        assert_eq!(classify(&e), Tier::T4);
    }

    #[test]
    fn conforming_element_has_no_gap_and_high_score() {
        let net = make_network();
        let entry = conforming_entry();

        assert!(tier_sla_gap(&entry).is_none());

        let dim = conformance(&entry, &net);
        assert!(dim.redundancy);
        assert!(dim.score.value() >= 9.0);
    }

    #[test]
    fn shortfall_yields_provisional_gap() {
        let entry = tarmac_corpus::CorpusEntry {
            id: String::from("A"),
            tier: Some(String::from("T1")),
            quantities: vec![tarmac_corpus::Quantity {
                value: 100.0,
                unit: String::from("ops"),
                label: tarmac_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        };

        let gap = tier_sla_gap(&entry).unwrap();
        assert_eq!(gap.label, tarmac_corpus::EvidenceLabel::Provisional);
        assert_eq!(gap.tier, Tier::T1);
        assert!(gap.observed_ops < gap.required_ops);
    }

    #[test]
    fn redundancy_increases_conformance_score() {
        let net = make_network();

        let diverse = conforming_entry();

        let isolated = tarmac_corpus::CorpusEntry {
            id: String::from("Z"),
            tier: Some(String::from("T1")),
            quantities: vec![tarmac_corpus::Quantity {
                value: 500.0,
                unit: String::from("ops"),
                label: tarmac_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        };

        let diverse_dim = conformance(&diverse, &net);
        let isolated_dim = conformance(&isolated, &net);

        assert!(diverse_dim.redundancy);
        assert!(!isolated_dim.redundancy);
        assert!(isolated_dim.score.value() < diverse_dim.score.value());
    }
}
