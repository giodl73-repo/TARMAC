use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tarmac_corpus::CorpusEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Dimension {
    Dim01,
    Dim02,
    Dim03,
    Dim04,
    Dim05,
    Dim06,
    Dim07,
    Dim08,
    Dim09,
    Dim10,
    Dim11,
    Dim12,
    Dim13,
}

impl Dimension {
    pub fn code(&self) -> &'static str {
        match self {
            Dimension::Dim01 => "DIM-01",
            Dimension::Dim02 => "DIM-02",
            Dimension::Dim03 => "DIM-03",
            Dimension::Dim04 => "DIM-04",
            Dimension::Dim05 => "DIM-05",
            Dimension::Dim06 => "DIM-06",
            Dimension::Dim07 => "DIM-07",
            Dimension::Dim08 => "DIM-08",
            Dimension::Dim09 => "DIM-09",
            Dimension::Dim10 => "DIM-10",
            Dimension::Dim11 => "DIM-11",
            Dimension::Dim12 => "DIM-12",
            Dimension::Dim13 => "DIM-13",
        }
    }

    pub fn all() -> [Dimension; 13] {
        [
            Dimension::Dim01,
            Dimension::Dim02,
            Dimension::Dim03,
            Dimension::Dim04,
            Dimension::Dim05,
            Dimension::Dim06,
            Dimension::Dim07,
            Dimension::Dim08,
            Dimension::Dim09,
            Dimension::Dim10,
            Dimension::Dim11,
            Dimension::Dim12,
            Dimension::Dim13,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Score(f64);

impl Score {
    pub fn new(value: f64) -> Option<Score> {
        if (0.0..=10.0).contains(&value) {
            Some(Score(value))
        } else {
            None
        }
    }

    pub fn clamped(value: f64) -> Score {
        Score(value.clamp(0.0, 10.0))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rubric {
    pub version: String,
    pub weights: BTreeMap<Dimension, f64>,
}

impl Rubric {
    pub fn v0() -> Rubric {
        let mut weights = BTreeMap::new();
        for dimension in Dimension::all() {
            weights.insert(dimension, 1.0);
        }
        Rubric {
            version: "v0".to_string(),
            weights,
        }
    }

    pub fn rationale(&self) -> &'static str {
        "v0 rubric assigns equal unit weight to every dimension pending calibration."
    }
}

pub trait DimensionScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Score;
}

#[derive(Clone, Debug)]
pub struct ProvisionalScorer {
    pub rubric: Rubric,
}

impl DimensionScorer for ProvisionalScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Score {
        let value = entry.scores.get(dimension.code()).copied().unwrap_or(0.0);
        Score::clamped(value)
    }
}

impl Default for ProvisionalScorer {
    fn default() -> Self {
        ProvisionalScorer {
            rubric: Rubric::v0(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_new_rejects_above_range() {
        assert!(Score::new(10.5).is_none());
    }

    #[test]
    fn score_new_rejects_below_range() {
        assert!(Score::new(-0.5).is_none());
    }

    #[test]
    fn score_new_accepts_in_range() {
        let score = Score::new(7.5).unwrap();
        assert_eq!(score.value(), 7.5);
    }

    #[test]
    fn score_clamped_maps_out_of_range() {
        assert_eq!(Score::clamped(15.0).value(), 10.0);
        assert_eq!(Score::clamped(-3.0).value(), 0.0);
    }

    #[test]
    fn rubric_v0_is_complete() {
        let rubric = Rubric::v0();
        assert_eq!(rubric.version, "v0");
        for dimension in Dimension::all() {
            assert_eq!(rubric.weights.get(&dimension).copied(), Some(1.0));
        }
        assert_eq!(rubric.weights.len(), 13);
        assert!(!rubric.rationale().is_empty());
    }

    #[test]
    fn provisional_scorer_reads_present_dimension() {
        let mut entry = CorpusEntry::default();
        entry.scores.insert("DIM-01".to_string(), 4.0);
        let scorer = ProvisionalScorer::default();
        let score = scorer.score(&entry, Dimension::Dim01);
        assert_eq!(score.value(), 4.0);
    }

    #[test]
    fn provisional_scorer_defaults_when_absent() {
        let entry = CorpusEntry::default();
        let scorer = ProvisionalScorer::default();
        let score = scorer.score(&entry, Dimension::Dim02);
        assert_eq!(score.value(), 0.0);
    }

    #[test]
    fn provisional_scorer_bounds_out_of_range() {
        let mut entry = CorpusEntry::default();
        entry.scores.insert("DIM-03".to_string(), 42.0);
        let scorer = ProvisionalScorer::default();
        let score = scorer.score(&entry, Dimension::Dim03);
        assert_eq!(score.value(), 10.0);
    }
}
