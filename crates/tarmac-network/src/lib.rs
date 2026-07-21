use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::algo::has_path_connecting;
use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AirportRole {
    Hub,
    Regional,
    Reliever,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    Peak,
    Average,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Airport {
    pub id: String,
    pub name: String,
    pub role: AirportRole,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub ops_per_day: f64,
    pub basis: DemandBasis,
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("airport with id `{0}` already exists")]
    DuplicateAirport(String),
    #[error("unknown airport id `{0}`")]
    UnknownAirport(String),
    #[error("route ops per day must be positive, got {0}")]
    NonPositiveOps(f64),
}

pub struct Network {
    graph: UnGraph<Airport, Route>,
    index: HashMap<String, NodeIndex>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            graph: UnGraph::new_undirected(),
            index: HashMap::new(),
        }
    }

    pub fn add_airport(&mut self, airport: Airport) -> Result<(), NetworkError> {
        if self.index.contains_key(&airport.id) {
            return Err(NetworkError::DuplicateAirport(airport.id));
        }
        let id = airport.id.clone();
        let idx = self.graph.add_node(airport);
        self.index.insert(id, idx);
        Ok(())
    }

    pub fn add_route(
        &mut self,
        from_id: &str,
        to_id: &str,
        route: Route,
    ) -> Result<(), NetworkError> {
        if route.ops_per_day <= 0.0 {
            return Err(NetworkError::NonPositiveOps(route.ops_per_day));
        }
        let from = *self
            .index
            .get(from_id)
            .ok_or_else(|| NetworkError::UnknownAirport(from_id.to_string()))?;
        let to = *self
            .index
            .get(to_id)
            .ok_or_else(|| NetworkError::UnknownAirport(to_id.to_string()))?;
        self.graph.add_edge(from, to, route);
        Ok(())
    }

    pub fn airport_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn route_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn degree(&self, id: &str) -> Option<usize> {
        let idx = self.index.get(id)?;
        Some(self.graph.edges(*idx).count())
    }

    pub fn degree_centrality(&self, id: &str) -> Option<f64> {
        let deg = self.degree(id)?;
        let n = self.airport_count();
        if n < 2 {
            return None;
        }
        Some(deg as f64 / (n - 1) as f64)
    }

    pub fn is_connected(&self, a: &str, b: &str) -> bool {
        match (self.index.get(a), self.index.get(b)) {
            (Some(&ia), Some(&ib)) => has_path_connecting(&self.graph, ia, ib, None),
            _ => false,
        }
    }

    pub fn has_diverse_path(&self, a: &str, b: &str) -> bool {
        let ia = match self.index.get(a) {
            Some(&i) => i,
            None => return false,
        };
        let ib = match self.index.get(b) {
            Some(&i) => i,
            None => return false,
        };
        if ia == ib {
            return false;
        }
        let empty: HashSet<NodeIndex> = HashSet::new();
        let path1 = match self.find_path(ia, ib, &empty) {
            Some(p) => p,
            None => return false,
        };
        let blocked: HashSet<NodeIndex> = path1
            .iter()
            .copied()
            .filter(|&n| n != ia && n != ib)
            .collect();
        self.find_path(ia, ib, &blocked).is_some()
    }

    pub fn incident_ops(&self, id: &str) -> f64 {
        let idx = match self.index.get(id) {
            Some(&i) => i,
            None => return 0.0,
        };
        let mut total = 0.0;
        for e in self.graph.edge_indices() {
            if let Some((s, t)) = self.graph.edge_endpoints(e) {
                if s == idx || t == idx {
                    if let Some(route) = self.graph.edge_weight(e) {
                        total += route.ops_per_day;
                    }
                }
            }
        }
        total
    }

    fn find_path(
        &self,
        start: NodeIndex,
        goal: NodeIndex,
        blocked: &HashSet<NodeIndex>,
    ) -> Option<Vec<NodeIndex>> {
        if start == goal {
            return Some(vec![start]);
        }
        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut prev: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            for nb in self.graph.neighbors(node) {
                if blocked.contains(&nb) || visited.contains(&nb) {
                    continue;
                }
                visited.insert(nb);
                prev.insert(nb, node);
                if nb == goal {
                    let mut path = vec![goal];
                    let mut cur = goal;
                    while let Some(&p) = prev.get(&cur) {
                        path.push(p);
                        cur = p;
                    }
                    path.reverse();
                    return Some(path);
                }
                queue.push_back(nb);
            }
        }
        None
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn airport(id: &str, role: AirportRole) -> Airport {
        Airport {
            id: id.to_string(),
            name: format!("{} Airport", id),
            role,
        }
    }

    fn route(id: &str, ops: f64) -> Route {
        Route {
            id: id.to_string(),
            ops_per_day: ops,
            basis: DemandBasis::Peak,
        }
    }

    fn ring_network() -> Network {
        let mut n = Network::new();
        n.add_airport(airport("A", AirportRole::Hub)).unwrap();
        n.add_airport(airport("B", AirportRole::Regional)).unwrap();
        n.add_airport(airport("C", AirportRole::Regional)).unwrap();
        n.add_route("A", "B", route("AB", 10.0)).unwrap();
        n.add_route("B", "C", route("BC", 5.0)).unwrap();
        n.add_route("A", "C", route("AC", 20.0)).unwrap();
        n
    }

    #[test]
    fn counts_and_degree() {
        let n = ring_network();
        assert_eq!(n.airport_count(), 3);
        assert_eq!(n.route_count(), 3);
        assert_eq!(n.degree("A"), Some(2));
        assert_eq!(n.degree("Z"), None);
    }

    #[test]
    fn connectivity() {
        let mut n = ring_network();
        n.add_airport(airport("D", AirportRole::Reliever)).unwrap();
        assert!(n.is_connected("A", "C"));
        assert!(!n.is_connected("A", "D"));
    }

    #[test]
    fn incident_ops_sum() {
        let n = ring_network();
        assert_eq!(n.incident_ops("A"), 30.0);
        assert_eq!(n.incident_ops("B"), 15.0);
    }

    #[test]
    fn diverse_path_ring_true() {
        let n = ring_network();
        assert!(n.has_diverse_path("A", "C"));
    }

    #[test]
    fn diverse_path_chain_false() {
        let mut n = Network::new();
        n.add_airport(airport("X", AirportRole::Hub)).unwrap();
        n.add_airport(airport("Y", AirportRole::Regional)).unwrap();
        n.add_airport(airport("Z", AirportRole::Regional)).unwrap();
        n.add_route("X", "Y", route("XY", 4.0)).unwrap();
        n.add_route("Y", "Z", route("YZ", 6.0)).unwrap();
        assert!(!n.has_diverse_path("X", "Z"));
    }

    #[test]
    fn centrality_in_range() {
        let n = ring_network();
        let c = n.degree_centrality("A").unwrap();
        assert!(c > 0.0 && c <= 1.0);
    }

    #[test]
    fn rejects_duplicate_airport() {
        let mut n = Network::new();
        n.add_airport(airport("A", AirportRole::Hub)).unwrap();
        let err = n.add_airport(airport("A", AirportRole::Regional));
        assert!(matches!(err, Err(NetworkError::DuplicateAirport(_))));
    }

    #[test]
    fn rejects_bad_route() {
        let mut n = Network::new();
        n.add_airport(airport("A", AirportRole::Hub)).unwrap();
        n.add_airport(airport("B", AirportRole::Regional)).unwrap();
        let bad_ops = n.add_route("A", "B", route("AB", 0.0));
        assert!(matches!(bad_ops, Err(NetworkError::NonPositiveOps(_))));
        let unknown = n.add_route("A", "Q", route("AQ", 5.0));
        assert!(matches!(unknown, Err(NetworkError::UnknownAirport(_))));
    }
}
