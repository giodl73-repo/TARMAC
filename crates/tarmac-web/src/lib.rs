//! Synthetic regional graph-rating adapter. No hydraulic or rights determination.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tarmac_network::{Airport, AirportRole, DemandBasis, Network, Route};

pub const MAX_INPUT_BYTES: usize = 65_536;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub id: String,
    pub from: String,
    pub to: String,
    pub ops_per_day: f64,
    pub basis: DemandBasis,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Site {
    pub id: String,
    pub name: String,
    pub role: AirportRole,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub scale: String,
    pub jurisdiction: String,
    pub comparison_window: String,
    pub selected_node: String,
    pub hub: String,
    pub nodes: Vec<Site>,
    pub links: Vec<Link>,
}

fn safe_id(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 64
        && text
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c))
}

/// Evaluate the native network's adjacent-capacity arithmetic only.
pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of synthetic graph JSON".into());
    }
    let input: Input = serde_json::from_str(input).map_err(|error| error.to_string())?;
    if input.scale != "regional"
        || input.jurisdiction != "fictional-cedar"
        || input.comparison_window != "synthetic-peak-operations"
    {
        return Err("Keep the regional fictional Cedar scope and common peak basis".into());
    }
    if input.nodes.is_empty() || input.nodes.len() > 16 || input.links.len() > 32 {
        return Err("Use 1–16 nodes and at most 32 links".into());
    }
    let mut graph = Network::new();
    for site in &input.nodes {
        if !safe_id(&site.id) || !safe_id(&site.name) {
            return Err("Use short fictional identifiers for nodes, names and kinds".into());
        }
        graph
            .add_airport(Airport {
                id: site.id.clone(),
                name: site.name.clone(),
                role: site.role,
            })
            .map_err(|error| error.to_string())?;
    }
    let mut links = HashSet::new();
    let mut pairs = HashSet::new();
    for link in &input.links {
        let pair = if link.from < link.to {
            (&link.from, &link.to)
        } else {
            (&link.to, &link.from)
        };
        if !pairs.insert(pair) {
            return Err("Use one aggregate route per unordered airport pair".into());
        }
        if !safe_id(&link.id)
            || !links.insert(&link.id)
            || link.from == link.to
            || !input.nodes.iter().any(|node| node.id == link.from)
            || !input.nodes.iter().any(|node| node.id == link.to)
        {
            return Err("Each link needs a unique safe id and two distinct known nodes".into());
        }
        if !link.ops_per_day.is_finite()
            || link.ops_per_day <= 0.0
            || link.ops_per_day > 1_000_000_000.0
        {
            return Err(
                "Link ratings must be finite, positive and at most 1 billion operations/day".into(),
            );
        }
        if link.basis != DemandBasis::Peak {
            return Err("Mixed or average basis is not comparable in this peak example".into());
        }
        if link.enabled {
            graph
                .add_route(
                    &link.from,
                    &link.to,
                    Route {
                        id: link.id.clone(),
                        ops_per_day: link.ops_per_day,
                        basis: link.basis,
                    },
                )
                .map_err(|error| error.to_string())?;
        }
    }
    if !input
        .nodes
        .iter()
        .any(|site| site.id == input.hub && site.role == AirportRole::Hub)
    {
        return Err("Name a known Hub target".into());
    }
    let degree = graph
        .degree(&input.selected_node)
        .ok_or_else(|| "Unknown selected airport".to_string())?;
    let incident = graph.incident_ops(&input.selected_node);
    let centrality = graph.degree_centrality(&input.selected_node);
    let connected = graph.is_connected(&input.selected_node, &input.hub);
    // Canonical object keys are sorted by serde_json's default map implementation.
    serde_json::to_string(&serde_json::json!({
        "schema":"tarmac.synthetic-regional-connectivity.v1", "evidence":"simulated",
        "scope":{"scale":input.scale,"jurisdiction":input.jurisdiction,"window":input.comparison_window,"unit":"operations-per-day"},
        "selected_node":input.selected_node,"hub":input.hub,
        "metrics":{"enabled_incident_routes":degree,"incident_ops_per_day":incident,"degree_centrality":centrality,"path_connected_to_hub":connected},
        "nodes":input.nodes,"links":input.links,
        "not_assessed":{"flight_schedule":null,"delay_reliability":null,"slots_gates":null,"fares_competition":null,"actual_community_access":null,"ground_access":null,"noise_emissions":null},
        "authority":{"engineering":false,"allocation":false,"rights":false,"savings":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

pub fn response(input: &str) -> String {
    match evaluate(input) {
        Ok(canonical) => serde_json::json!({"ok":true,"canonical":canonical,"result":serde_json::from_str::<serde_json::Value>(&canonical).expect("serialized result")}).to_string(),
        Err(error) => serde_json::json!({"ok":false,"error":error}).to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
mod abi;

#[cfg(test)]
mod tests {
    use super::*;
    const RESET: &str = include_str!("../../../fixtures/browser-reset.json");
    #[test]
    fn native_route_metrics_preserve_schedule_unknowns() {
        let value: serde_json::Value = serde_json::from_str(&evaluate(RESET).unwrap()).unwrap();
        assert_eq!(value["metrics"]["incident_ops_per_day"], 12.0);
        assert_eq!(value["metrics"]["degree_centrality"], 1.0 / 3.0);
        assert_eq!(value["metrics"]["path_connected_to_hub"], true);
        assert!(value["not_assessed"]["flight_schedule"].is_null());
    }
    #[test]
    fn route_loss_disconnects_the_fictional_community() {
        let input = include_str!("../../../fixtures/browser-outage.json");
        let value: serde_json::Value = serde_json::from_str(&evaluate(input).unwrap()).unwrap();
        assert_eq!(value["metrics"]["incident_ops_per_day"], 0.0);
        assert_eq!(value["metrics"]["path_connected_to_hub"], false);
    }
    #[test]
    fn rejects_mixed_basis_unknown_selection_and_parallel_pairs() {
        assert!(evaluate(&RESET.replace("Peak", "Average")).is_err());
        assert!(evaluate(&RESET.replace(
            "\"selected_node\": \"community\"",
            "\"selected_node\": \"missing\""
        ))
        .is_err());
        let mut input: serde_json::Value = serde_json::from_str(RESET).unwrap();
        let mut duplicate = input["links"][0].clone();
        duplicate["id"] = serde_json::json!("duplicate-pair");
        input["links"].as_array_mut().unwrap().push(duplicate);
        assert!(evaluate(&input.to_string()).is_err());
        assert!(evaluate(&"x".repeat(65537)).is_err());
    }
}
