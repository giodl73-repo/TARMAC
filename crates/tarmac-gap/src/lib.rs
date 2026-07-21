//! Gap analysis for the TARMAC project.
//!
//! This crate identifies dimensions of a market [`Scale`](tarmac_corpus::Scale)
//! whose provisional scores fall below an adequacy threshold, producing a
//! [`GapAnalysis`] describing the weak regions and under-served tails together
//! with any tier SLA gaps that apply to the analysed entries.
//!
//! Cross-scale analysis (considering corpus entries regardless of their declared
//! scale) requires passing `cross_scale = true` to [`find_gaps`]. When
//! `cross_scale = false`, only entries whose scale equals `Some(scale)` are
//! considered.

use serde::{Deserialize, Serialize};
use tarmac_corpus::{CorpusEntry, EvidenceLabel, Scale};
use tarmac_score::{Dimension, DimensionScorer, ProvisionalScorer, Rubric};
use tarmac_tier::TierSlaGap;

/// Threshold below which a dimension's mean score is considered a gap.
const ADEQUACY_THRESHOLD: f64 = 5.0;

/// Share of scored entries below threshold at or above which a dispersion gap is
/// reclassified from a concentrated tail to a systemic deficit.
const SYSTEMIC_SHARE: f64 = 0.5;

/// A single weak region of the market: one [`Dimension`] at one [`Scale`] whose
/// mean provisional score fell strictly below the adequacy threshold.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapRegion {
    pub dimension: Dimension,
    pub scale: Scale,
    pub mean_score: f64,
    pub member_ids: Vec<String>,
    pub label: EvidenceLabel,
}

/// A dimension whose under-served tail falls below the adequacy threshold even
/// if the corpus mean does not.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TailGapRegion {
    pub dimension: Dimension,
    pub scale: Scale,
    pub tail_mean: f64,
    pub tail_member_ids: Vec<String>,
    /// Fraction of scored entries below threshold.
    pub share_below_threshold: f64,
    /// True when the below-threshold share reaches [`SYSTEMIC_SHARE`].
    pub systemic: bool,
    pub label: EvidenceLabel,
}

/// The result of analysing a corpus for gaps at a given scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub scale: Scale,
    pub regions: Vec<GapRegion>,
    pub tail_regions: Vec<TailGapRegion>,
    pub tier_sla_gaps: Vec<TierSlaGap>,
    pub null_result: bool,
}

/// Analyse `corpus` for score gaps at `scale`.
///
/// When `cross_scale` is `false`, only entries whose scale equals `Some(scale)`
/// are selected; when `cross_scale` is `true`, all entries are selected
/// regardless of their declared scale. Cross-scale analysis therefore requires
/// `cross_scale = true`.
///
/// Each selected entry is scored on every [`Dimension`] using
/// [`ProvisionalScorer::default`]. For each dimension whose mean score across
/// the selected entries is strictly below `5.0`, a [`GapRegion`] is emitted. The
/// subset of `tier_gaps` whose `entry_id` is among the selected entry ids is
/// collected into [`GapAnalysis::tier_sla_gaps`]. The result is marked
/// `null_result` when there are no regions, tail regions, or collected tier
/// gaps.
///
/// The `rubric` is retained for provenance.
pub fn find_gaps(
    corpus: &[CorpusEntry],
    rubric: &Rubric,
    scale: Scale,
    tier_gaps: &[TierSlaGap],
    cross_scale: bool,
) -> GapAnalysis {
    let _ = rubric;

    let selected: Vec<&CorpusEntry> = corpus
        .iter()
        .filter(|entry| cross_scale || entry.scale == Some(scale))
        .collect();

    let selected_ids: Vec<String> = selected.iter().map(|entry| entry.id.clone()).collect();

    let scorer = ProvisionalScorer::default();
    let mut regions = Vec::new();
    let mut tail_regions = Vec::new();

    if !selected.is_empty() {
        let count = selected.len() as f64;
        for dimension in Dimension::all() {
            let mut scored: Vec<(&str, f64)> = selected
                .iter()
                .map(|entry| (entry.id.as_str(), scorer.score(entry, dimension).value()))
                .collect();
            let mean = scored.iter().map(|(_, value)| value).sum::<f64>() / count;

            if mean < ADEQUACY_THRESHOLD {
                regions.push(GapRegion {
                    dimension,
                    scale,
                    mean_score: mean,
                    member_ids: selected_ids.clone(),
                    label: EvidenceLabel::Provisional,
                });
            }
            // Tail (dispersion) gap: even when the mean clears the bar, flag the
            // dimension if its worst-served quartile is inadequate.
            let under: Vec<String> = scored
                .iter()
                .filter(|(_, value)| *value < ADEQUACY_THRESHOLD)
                .map(|(id, _)| (*id).to_string())
                .collect();
            if !under.is_empty() {
                scored.sort_by(|a, b| a.1.total_cmp(&b.1));
                let quartile = selected.len().div_ceil(4).max(1);
                let tail_mean = scored
                    .iter()
                    .take(quartile)
                    .map(|(_, value)| value)
                    .sum::<f64>()
                    / quartile as f64;
                if tail_mean < ADEQUACY_THRESHOLD {
                    let share = under.len() as f64 / count;
                    tail_regions.push(TailGapRegion {
                        dimension,
                        scale,
                        tail_mean,
                        tail_member_ids: under,
                        share_below_threshold: share,
                        systemic: share >= SYSTEMIC_SHARE,
                        label: EvidenceLabel::Provisional,
                    });
                }
            }
        }
    }

    let tier_sla_gaps: Vec<TierSlaGap> = tier_gaps
        .iter()
        .filter(|gap| selected_ids.iter().any(|id| id == &gap.entry_id))
        .cloned()
        .collect();

    let null_result = regions.is_empty() && tail_regions.is_empty() && tier_sla_gaps.is_empty();

    GapAnalysis {
        scale,
        regions,
        tail_regions,
        tier_sla_gaps,
        null_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_with(id: &str, scale: Scale, value: f64) -> CorpusEntry {
        let mut scores = std::collections::BTreeMap::new();
        for dim in tarmac_score::Dimension::all() {
            scores.insert(String::from(dim.code()), value);
        }
        CorpusEntry {
            id: String::from(id),
            scale: Some(scale),
            scores,
            ..Default::default()
        }
    }

    #[test]
    fn low_scores_yield_gap_region() {
        let corpus = vec![entry_with("A", Scale::National, 2.0)];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);

        assert!(!analysis.regions.is_empty());
        assert!(!analysis.null_result);
    }

    #[test]
    fn adequate_market_is_null_result() {
        let corpus = vec![entry_with("A", Scale::National, 8.0)];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tier_sla_gaps.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn split_corpus_flags_tail_gap_even_when_mean_clears_bar() {
        let corpus = vec![
            entry_with("low1", Scale::Regional, 1.0),
            entry_with("low2", Scale::Regional, 1.0),
            entry_with("high1", Scale::Regional, 9.0),
            entry_with("high2", Scale::Regional, 9.0),
        ];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::Regional, &[], false);

        assert!(
            analysis.regions.is_empty(),
            "mean is 5.0, not below the bar"
        );
        assert!(!analysis.tail_regions.is_empty(), "the tail is inadequate");
        assert!(!analysis.null_result);
        let tail = analysis.tail_regions.first().unwrap();
        assert!(tail.tail_mean < 5.0);
        assert!(tail.tail_member_ids.contains(&"low1".to_string()));
        assert!(!tail.tail_member_ids.contains(&"high1".to_string()));
    }

    #[test]
    fn adequate_market_has_no_tail_gap() {
        let corpus = vec![
            entry_with("A", Scale::National, 7.0),
            entry_with("B", Scale::National, 5.0),
        ];
        let analysis = find_gaps(&corpus, &Rubric::v0(), Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn tail_share_classifies_minority_vs_systemic() {
        let minority = vec![
            entry_with("low1", Scale::Regional, 1.0),
            entry_with("high1", Scale::Regional, 9.0),
            entry_with("high2", Scale::Regional, 9.0),
            entry_with("high3", Scale::Regional, 9.0),
        ];
        let analysis = find_gaps(&minority, &Rubric::v0(), Scale::Regional, &[], false);
        let tail = analysis.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.25).abs() < 1e-9);
        assert!(!tail.systemic);

        let majority = vec![
            entry_with("low1", Scale::Regional, 1.0),
            entry_with("low2", Scale::Regional, 1.0),
            entry_with("low3", Scale::Regional, 1.0),
            entry_with("high1", Scale::Regional, 9.0),
        ];
        let analysis = find_gaps(&majority, &Rubric::v0(), Scale::Regional, &[], false);
        let tail = analysis.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.75).abs() < 1e-9);
        assert!(tail.systemic);
    }

    #[test]
    fn other_scale_excluded_unless_cross_scale() {
        let corpus = vec![entry_with("A", Scale::National, 2.0)];

        let excluded = find_gaps(&corpus, &Rubric::v0(), Scale::Regional, &[], false);
        assert!(excluded.regions.is_empty());
        assert!(excluded.null_result);

        let included = find_gaps(&corpus, &Rubric::v0(), Scale::Regional, &[], true);
        assert!(!included.regions.is_empty());
        assert!(!included.null_result);
    }
}
